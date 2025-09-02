//! Single-thread pipeline: synth → play → synth → play … with silence padding

use std::{
    io::{self, Write},
    path::Path,
    sync::{Arc, LazyLock, Mutex},
    thread::available_parallelism,
    time::{Duration, SystemTime},
    process::Stdio,
};
use ndarray::{Array, Ix2, Dim};
use ort::{
    inputs,
    execution_providers::{CPUExecutionProvider, DirectMLExecutionProvider},
    session::{Session, builder::GraphOptimizationLevel},
};
use tokio::{fs::read, process::Command};
use cmudict_fast::Cmudict;
use once_cell::sync::Lazy;
use rand::rngs::ThreadRng;
use rand::RngCore;
use regex::Regex;
use std::collections::HashMap;
use serde_json::Value;
use std::fs;
use rodio::{OutputStream, Sink, buffer::SamplesBuffer};

// =============================================================================
// g2p: eSpeak NG IPA → fallback CMU, with robust punctuation handling
// =============================================================================

/// Original CMU‐based fallback renamed for clarity.
fn g2p_fallback(text: &str) -> String {
    static CMU: Lazy<Cmudict> = Lazy::new(|| {
        Cmudict::new("cmudict.dict")
            .unwrap_or_else(|e| panic!("cmudict.dict error: {}", e))
    });
    static ARPA_IPA_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
        let mut m = HashMap::new();
        for &(arp, ipa) in &[
            ("AA","ɑː"),("AE","æ"),("AH","ə"),("AO","ɔː"),
            ("AW","aʊ"),("AY","aɪ"),("B","b"),("CH","tʃ"),
            ("D","d"),("DH","ð"),("EH","ɛ"),("ER","ɜːr"),
            ("EY","eɪ"),("F","f"),("G","ɡ"),("HH","h"),
            ("IH","ɪ"),("IY","iː"),("JH","dʒ"),("K","k"),
            ("L","l"),("M","m"),("N","n"),("NG","ŋ"),
            ("OW","oʊ"),("OY","ɔɪ"),("P","p"),("R","ɹ"),
            ("S","s"),("SH","ʃ"),("T","t"),("TH","θ"),
            ("UH","ʊ"),("UW","uː"),("V","v"),("W","w"),
            ("Y","j"),("Z","z"),("ZH","ʒ"),("AX","ə"),
            ("AXR","ɚ"),("DX","ɾ"),("NX","ɾ̃"),("SIL",""),
            ("AH0","ʌ"),("AO0","ɒ"),("IY0","i"),("IY1","iː"),
            ("IY2","i"),("UW0","u"),("UW1","uː"),("UW2","u"),
            ("HH0","h"),("HH1","h"),("HH2","h"),("ER0","ɜːr"),
            ("ER1","ɜːr"),("ER2","ɜːr"),("ˈ","ˈ"),("ˌ","ˌ"),
        ] {
            m.insert(arp, ipa);
        }
        m
    });
    static LETTERS_IPA_MAP: Lazy<HashMap<char, &'static str>> = Lazy::new(|| {
        let mut m = HashMap::new();
        for &(c, s) in &[
            ('a',"ɑ"),('b',"b"),('c',"k"),('d',"d"),('e',"ɛ"),
            ('f',"f"),('g',"g"),('h',"h"),('i',"i"),('j',"dʒ"),
            ('k',"k"),('l',"l"),('m',"m"),('n',"n"),('o',"o"),
            ('p',"p"),('q',"k"),('r',"ɹ"),('s',"s"),('t',"t"),
            ('u',"u"),('v',"v"),('w',"w"),('x',"ks"),('y',"j"),
            ('z',"z")
        ] {
            m.insert(c, s);
        }
        m
    });

    fn letters_to_ipa(letters: &str) -> String {
        letters.chars()
            .filter_map(|c| LETTERS_IPA_MAP.get(&c).copied())
            .collect()
    }
    fn arpa_to_ipa(token: &str) -> String {
        let re = Regex::new(r"^([A-Z!]+)(\d?)$").unwrap();
        if let Some(caps) = re.captures(token) {
            let mut out = match &caps[2] {
                "1" => "ˈ".to_string(),
                "2" => "ˌ".to_string(),
                _   => String::new(),
            };
            out.push_str(ARPA_IPA_MAP.get(&caps[1]).copied().unwrap_or(""));
            out
        } else {
            String::new()
        }
    }
    fn digit_to_word(d: char) -> &'static str {
        match d {
            '0'=>"zero",'1'=>"one",'2'=>"two",
            '3'=>"three",'4'=>"four",'5'=>"five",
            '6'=>"six",'7'=>"seven",'8'=>"eight",
            '9'=>"nine", _=>"",
        }
    }
    fn word2ipa(word: &str) -> String {
        let dict = &*CMU;
        let key = word.to_lowercase();
        if let Some(entries) = dict.get(&key) {
            let idx = (ThreadRng::default().next_u32() as usize) % entries.len();
            return entries[idx].pronunciation()
                .iter()
                .map(|p| arpa_to_ipa(&p.to_string()))
                .collect();
        }
        letters_to_ipa(word)
    }

    // we split on letters+numbers (\p{L}\p{N}), punctuation (\p{P}), or other
    let token_re = Regex::new(r"(\p{L}+\p{N}*|\p{N}+|\p{P}|\s+)").unwrap();
    token_re.captures_iter(text)
        .map(|cap| {
            let tok = &cap[0];
            if tok.chars().all(|c| c.is_ascii_whitespace()) {
                tok.to_string()
            } else if tok.chars().all(|c| c.is_ascii_punctuation()) {
                tok.to_string()
            } else if tok.chars().all(|c| c.is_digit(10)) {
                tok.chars().map(|d| word2ipa(digit_to_word(d))).collect()
            } else if tok.chars().any(|c| c.is_alphabetic()) {
                word2ipa(tok)
            } else {
                tok.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

/// Async wrapper: try eSpeak NG, else fallback.
pub async fn g2p(text: &str) -> String {
    match Command::new("espeak-ng")
        .arg("--ipa=1").arg("-q").arg("--pho").arg(text)
        .stdout(Stdio::piped())
        .output()
        .await
    {
        Ok(out) if out.status.success() => {
            let ipa = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !ipa.is_empty() { return ipa; }
        }
        _ => {}
    }
    g2p_fallback(text)
}

// =============================================================================
// tokenizer: char → ID map from tokenizer.json (unchanged)
// =============================================================================

static CHAR_TO_ID: Lazy<HashMap<char, i64>> = Lazy::new(|| {
    let txt = fs::read_to_string("tokenizer.json").unwrap();
    let j: Value = serde_json::from_str(&txt).unwrap();
    j["model"]["vocab"].as_object().unwrap().iter()
        .filter_map(|(k,v)| k.chars().next().map(|c| (c, v.as_i64().unwrap())))
        .collect()
});

pub fn get_token_ids(phonemes: &str) -> Vec<i64> {
    let mut ids = Vec::with_capacity(phonemes.len() + 2);
    ids.push(0);
    for ch in phonemes.chars() {
        if let Some(&i) = CHAR_TO_ID.get(&ch) {
            ids.push(i);
        }
    }
    ids.push(0);
    ids
}

// =============================================================================
// loader, synthesizer, main: unchanged from your previous version
// =============================================================================

static VOICE_PACK: LazyLock<Mutex<Vec<Vec<f32>>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static MODEL:      LazyLock<Mutex<Option<Arc<Session>>>> = LazyLock::new(|| Mutex::new(None));

pub async fn load(model_path: impl AsRef<Path>, voice_file: impl AsRef<Path>) {
    let buf = read(&voice_file).await.unwrap();
    let floats: Vec<f32> = buf.chunks_exact(4)
        .map(|b| f32::from_le_bytes([b[0],b[1],b[2],b[3]]))
        .collect();
    *VOICE_PACK.lock().unwrap() = floats.chunks(256).map(|c| c.to_vec()).collect();

    let cores = available_parallelism().unwrap().get();
    let cpu = CPUExecutionProvider::default().build();
    let dml = DirectMLExecutionProvider::default().build();
    let sess = Session::builder().unwrap()
        .with_optimization_level(GraphOptimizationLevel::Level3).unwrap()
        .with_intra_threads(cores).unwrap()
        .with_inter_threads(cores).unwrap()
        .with_execution_providers([cpu, dml]).unwrap()
        .commit_from_file(model_path).unwrap();
    *MODEL.lock().unwrap() = Some(Arc::new(sess));
}

pub fn style_for(token_index: usize) -> Array<f32, Ix2> {
    let pack = VOICE_PACK.lock().unwrap();
    Array::from_shape_vec(
        Dim([1, pack[token_index].len()]),
        pack[token_index].clone(),
    ).unwrap()
}

pub fn session() -> Arc<Session> {
    MODEL.lock().unwrap().as_ref().unwrap().clone()
}

pub async fn synth(text: String, speed: f32) -> (Vec<f32>, Duration) {
    let phoneme_str = g2p(&text).await;
    println!("> Phonemes: {}", phoneme_str);

    let ids = get_token_ids(&phoneme_str);
    let arr = Array::from_shape_vec((1, ids.len()), ids.clone()).unwrap();
    let style = style_for(arr.shape()[1] - 1);
    let speed_arr = Array::from_vec(vec![speed]);

    let model = session();
    let t0 = SystemTime::now();
    let inputs = inputs![
        "input_ids" => arr.view(),
        "style"     => style.view(),
        "speed"     => speed_arr.view()
    ].unwrap();

    let outputs = model.run_async(inputs).unwrap().await.unwrap();
    let took = t0.elapsed().unwrap();

    let wave = outputs["waveform"]
        .try_extract_tensor::<f32>().unwrap()
        .as_slice().unwrap().to_vec();
    (wave, took)
}

#[tokio::main]
async fn main() {
    print!("Enter a message: ");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let msg = buf.trim().to_string();
    if msg.is_empty() { return; }

    load("onnx/model.onnx", "voices/af_bella.bin").await;
    println!("Model & voice loaded.");

    let chars: Vec<char> = msg.chars().collect();
    let mut chunks = Vec::new();
    let mut start = 0;
    while start < chars.len() {
        let end = usize::min(start + 200, chars.len());
        chunks.push(chars[start..end].iter().collect());
        start = end;
    }

    let (_st, handle) = OutputStream::try_default().unwrap();
    let player = Arc::new(Sink::try_new(&handle).unwrap());

    for (i, chunk) in chunks.into_iter().enumerate() {
        let (audio, took) = synth(chunk, 1.0).await;
        println!("Chunk: {} | Synth: {:?}", i, took);
        player.append(SamplesBuffer::new(1, 24000, audio));
    }

    player.sleep_until_end();
}
