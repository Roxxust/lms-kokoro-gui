use super::*;
use crate::heteronyms::HETERONYMS;
use crate::contractions::word2ipa;
use lazy_static::lazy_static;
use futures::executor::block_on;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::mpsc::sync_channel;
use std::time::Duration;
use rodio::{OutputStream, Sink, buffer::SamplesBuffer};
use std::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::oneshot;
use std::thread;

pub const AVAILABLE_VOICES: &[&str] = &[
//female voices:
    "af_bella.bin", "af_heart.bin", "af_sarah.bin", "af_sky.bin", "af_river.bin",
    "af_nova.bin", "af_kore.bin", "af_jessica.bin", "af_aoede.bin", "af_alloy.bin",
    "af_maple.bin", "af_sol.bin",
    //bri'ish 
    "bf_emma.bin", "bf_lily.bin", "bf_isabella.bin", "bf_alice.bin", "bf_vale.bin",
    //japanese
    "jf_nezumi.bin", "jf_alpha.bin", "jf_gongitsune.bin", "jf_tebukuro.bin",
//male voices:
    "am_adam.bin", "am_liam.bin", "am_eric.bin", "am_echo.bin", "am_onyx.bin", "am_michael.bin",
    "am_santa.bin",
    //bri'ish
    "bm_daniel.bin", "bm_george.bin", "bm_lewis.bin",
    //japanese
    "jm_kumo.bin",
];

pub static CMU: Lazy<Cmudict> = Lazy::new(|| {
    Cmudict::new(TTS_CMU_DICT_PATH)
        .unwrap_or_else(|e| panic!("CMU Dict error: {}", e))
});

pub static ARPA_IPA_MAP: Lazy<std::collections::HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = std::collections::HashMap::new();
    for &(arp, ipa) in &[
        ("AA", "ɑː"), ("AE", "æ"), ("AH", "ə"), ("AO", "ɔː"),
        ("AW", "aʊ"), ("AY", "aɪ"), ("B", "b"), ("CH", "tʃ"),
        ("D", "d"), ("DH", "ð"), ("EH", "ɛ"), ("ER", "ɜːr"),
        ("EY", "eɪ"), ("F", "f"), ("G", "ɡ"), ("HH", "h"),
        ("IH", "ɪ"), ("IY", "iː"), ("JH", "dʒ"), ("K", "k"),
        ("L", "l"), ("M", "m"), ("N", "n"), ("NG", "ŋ"),
        ("OW", "oʊ"), ("OY", "ɔɪ"), ("P", "p"), ("R", "ɹ"),
        ("S", "s"), ("SH", "ʃ"), ("T", "t"), ("TH", "θ"),
        ("UH", "ʊ"), ("UW", "uː"), ("V", "v"), ("W", "w"),
        ("Y", "j"), ("Z", "z"), ("ZH", "ʒ"), ("SIL", "")
    ] {
        m.insert(arp, ipa);
    }
    m
});

enum AudioWorkerMessage {
    Play {
        samples: Vec<f32>,
        stop_flag: Arc<AtomicBool>,
        completion_sender: oneshot::Sender<()>,
    }
}

lazy_static! {
    static ref AUDIO_WORKER: AudioWorker = AudioWorker::new();
}

struct AudioWorker {
    sender: Sender<AudioWorkerMessage>,
}

impl AudioWorker {
    fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || audio_worker_thread(receiver));
        AudioWorker { sender }
    }
    
    fn play(&self, samples: Vec<f32>, stop_flag: Arc<AtomicBool>) -> oneshot::Receiver<()> {
        let (completion_sender, completion_receiver) = oneshot::channel();
        self.sender.send(AudioWorkerMessage::Play {
            samples,
            stop_flag,
            completion_sender,
        }).expect("Audio worker thread has stopped");
        completion_receiver
    }
}

fn audio_worker_thread(receiver: Receiver<AudioWorkerMessage>) {
    let (_stream, stream_handle) = match OutputStream::try_default() {
        Ok((stream, handle)) => (stream, handle),
        Err(e) => {
            eprintln!("Failed to open audio stream: {}", e);
            return;
        }
    };
    
    for message in receiver {
        match message {
            AudioWorkerMessage::Play { samples, stop_flag, completion_sender } => {
                let sink = match Sink::try_new(&stream_handle) {
                    Ok(sink) => sink,
                    Err(e) => {
                        eprintln!("Failed to create sink: {}", e);
                        let _ = completion_sender.send(());
                        continue;
                    }
                };
                
                let buffer = SamplesBuffer::new(1, 24000, samples);
                sink.append(buffer);
                sink.play();
                
                while !sink.empty() && !stop_flag.load(Ordering::Relaxed) {
                    thread::sleep(Duration::from_millis(5));
                }
                
                if stop_flag.load(Ordering::Relaxed) {
                    sink.stop();
                }
                
                let _ = completion_sender.send(());
            }
        }
    }
}

pub fn play_waveform(wave: Vec<f32>, stop_flag: Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut completion_receiver = AUDIO_WORKER.play(wave, stop_flag.clone());
    
    loop {
        if stop_flag.load(Ordering::Relaxed) {
            break;
        }
        
        match completion_receiver.try_recv() {
            Ok(_) => break,
            Err(oneshot::error::TryRecvError::Empty) => {
                thread::sleep(Duration::from_millis(10));
            }
            Err(oneshot::error::TryRecvError::Closed) => break,
        }
    }
    
    Ok(())
}

pub fn letters_to_ipa(word: &str) -> String {
    pub static BASE_LETTERS_IPA_MAP: Lazy<std::collections::HashMap<char, &'static str>> = Lazy::new(|| {
         let mut m = std::collections::HashMap::new();
         m.insert('a', "æ");
         m.insert('b', "b");
         m.insert('c', "k");
         m.insert('d', "d");
         m.insert('e', "ɛ");
         m.insert('f', "f");
         m.insert('g', "ɡ");
         m.insert('h', "h");
         m.insert('i', "ɪ");
         m.insert('j', "dʒ");
         m.insert('k', "k");
         m.insert('l', "l");
         m.insert('m', "m");
         m.insert('n', "n");
         m.insert('o', "ɑ");
         m.insert('p', "p");
         m.insert('q', "k");
         m.insert('r', "ɹ");
         m.insert('s', "s");
         m.insert('t', "t");
         m.insert('u', "ʌ");
         m.insert('v', "v");
         m.insert('w', "w");
         m.insert('x', "ks");
         m.insert('y', "j");
         m.insert('z', "z");
         m
    });
    let chars: Vec<char> = word.chars().collect();
    let len = chars.len();
    let mut ipa_result = String::new();
    let mut i = 0;
    while i < len {
        let current_char_lower = chars[i].to_lowercase().next().unwrap_or(chars[i]);
        let next_char_lower = if i + 1 < len { chars[i + 1].to_lowercase().next().unwrap_or(chars[i + 1]) } else { '\0' };
        let next_next_char_lower = if i + 2 < len { chars[i + 2].to_lowercase().next().unwrap_or(chars[i + 2]) } else { '\0' };
        let next_next_next_char_lower = if i + 3 < len { chars[i + 3].to_lowercase().next().unwrap_or(chars[i + 3]) } else { '\0' };
        let prev_char_lower = if i > 0 { chars[i - 1].to_lowercase().next().unwrap_or(chars[i - 1]) } else { '\0' };
        let _prev_prev_char_lower = if i > 1 { chars[i - 2].to_lowercase().next().unwrap_or(chars[i - 2]) } else { '\0' };
        let is_vowel = |c: char| ['a', 'e', 'i', 'o', 'u', 'y'].contains(&c);
        let is_consonant = |c: char| ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'z'].contains(&c);
        let is_liquid_or_nasal = |c: char| ['l', 'm', 'n', 'r', 'w'].contains(&c);
        match current_char_lower {
            'a' => {
                if (next_char_lower == 'i' && (i + 2 >= len || !chars[i + 2].is_alphabetic())) ||
                   (next_char_lower == 'y' && (i + 2 >= len || !chars[i + 2].is_alphabetic())) ||
                   (next_char_lower == 'i' && next_next_char_lower == 'g' && next_next_next_char_lower == 'h' && (i + 4 >= len || !chars[i + 4].is_alphabetic())) {
                    ipa_result.push_str("eɪ");
                    i += if next_char_lower == 'i' && next_next_char_lower == 'g' { 4 } else { 2 };
                    continue;
                }
                else if (next_char_lower == 'u' || next_char_lower == 'w') ||
                        (next_char_lower == 'l' && i + 2 < len && ['f', 'm', 's', 't', 'b', 'd', 'g', 'k', 'p', 'v', 'z'].contains(&next_next_char_lower)) {
                     ipa_result.push_str("ɔː");
                     i += 2;
                     continue;
                }
                else if next_char_lower == 'r' && i + 2 < len && is_vowel(next_next_char_lower) {
                     ipa_result.push_str("ɛə");
                     i += 1;
                     continue;
                }
                else if next_char_lower == 'r' && next_next_char_lower == 'e' && (i + 3 >= len || !chars[i + 3].is_alphabetic()) {
                     ipa_result.push_str("ɛə");
                     i += 3;
                     continue;
                }
                else if i + 2 < len && is_consonant(next_char_lower) && next_next_char_lower == 'e' && (i + 3 >= len || !chars[i + 3].is_alphabetic()) {
                     ipa_result.push_str("eɪ");
                     i += 3;
                     continue;
                }
                else if next_char_lower == 'l' && i + 2 < len && ['m', 'n', 'k'].contains(&next_next_char_lower) {
                     ipa_result.push_str("ɔː");
                     i += 1;
                     continue;
                }
                ipa_result.push('æ');
                i += 1;
                continue;
            }
            'e' => {
                if next_char_lower == 'e' || next_char_lower == 'a' || next_char_lower == 'i' || next_char_lower == 'y' {
                    if i == 0 && len == 3 && next_char_lower == 'y' && next_next_char_lower == 'e' {
                         ipa_result.push_str("aɪ");
                         i += 3;
                         continue;
                    }
                    ipa_result.push_str("iː");
                    i += 2;
                    continue;
                }
                else if next_char_lower == 'r' && i + 2 < len && is_vowel(next_next_char_lower) {
                     ipa_result.push_str("ɪə");
                     i += 1;
                     continue;
                }
                else if next_char_lower == 'w' {
                     ipa_result.push_str("juː");
                     i += 2;
                     continue;
                }
                else if i == len - 1 && prev_char_lower != '\0' && is_consonant(prev_char_lower) {
                     i += 1;
                     continue;
                }
                else if i + 2 < len && is_consonant(next_char_lower) && next_next_char_lower == 'e' && (i + 3 >= len || !chars[i + 3].is_alphabetic()) {
                     ipa_result.push_str("iː");
                     i += 3;
                     continue;
                }
                ipa_result.push('ɛ');
                i += 1;
                continue;
            }
            'i' => {
                if next_char_lower == 'e' && (i + 2 >= len || !chars[i + 2].is_alphabetic()) {
                    ipa_result.push_str("aɪ");
                    i += 2;
                    continue;
                }
                else if next_char_lower == 'g' && next_next_char_lower == 'h' {
                     ipa_result.push_str("aɪ");
                     i += 3;
                     continue;
                }
                else if next_char_lower == 'r' && i + 2 < len && is_vowel(next_next_char_lower) {
                     ipa_result.push_str("aɪə");
                     i += 1;
                     continue;
                }
                else if next_char_lower == 'r' && next_next_char_lower == 'e' && (i + 3 >= len || !chars[i + 3].is_alphabetic()) {
                     ipa_result.push_str("aɪə");
                     i += 3;
                     continue;
                }
                else if i + 2 < len && is_consonant(next_char_lower) && next_next_char_lower == 'e' && (i + 3 >= len || !chars[i + 3].is_alphabetic()) {
                     ipa_result.push_str("aɪ");
                     i += 3;
                     continue;
                }
                ipa_result.push('ɪ');
                i += 1;
                continue;
            }
            'o' => {
                if next_char_lower == 'o' {
                    ipa_result.push_str("uː");
                    i += 2;
                    continue;
                }
                else if (next_char_lower == 'a' || next_char_lower == 'e') ||
                        (next_char_lower == 'w' && i + 2 < len) {
                     ipa_result.push_str("əʊ");
                     i += 2;
                     continue;
                }
                else if next_char_lower == 'i' || next_char_lower == 'y' {
                     ipa_result.push_str("ɔɪ");
                     i += 2;
                     continue;
                }
                else if next_char_lower == 'u' {
                     ipa_result.push_str("aʊ");
                     i += 2;
                     continue;
                }
                else if i + 2 < len && is_consonant(next_char_lower) && next_next_char_lower == 'e' && (i + 3 >= len || !chars[i + 3].is_alphabetic()) {
                     ipa_result.push_str("əʊ");
                     i += 3;
                     continue;
                }
                else if next_char_lower == 'r' && i + 2 < len && is_vowel(next_next_char_lower) {
                     ipa_result.push_str("ɔː");
                     i += 1;
                     continue;
                }
                else if next_char_lower == 'r' && next_next_char_lower == 'e' {
                     ipa_result.push_str("ɔː");
                     i += 3;
                     continue;
                }
                else if next_char_lower == 'o' && next_next_char_lower == 'r' {
                     ipa_result.push_str("ɔː");
                     i += 3;
                     continue;
                }
                else if next_char_lower == 'u' && next_next_char_lower == 'r' {
                     ipa_result.push_str("aʊə");
                     i += 3;
                     continue;
                }
                else if next_char_lower == '\'' {
                     ipa_result.push_str("əʊ");
                     i += 1;
                     continue;
                }
                else if next_char_lower == 'w' && i + 2 == len {
                     ipa_result.push_str("aʊ");
                     i += 2;
                     continue;
                }
                else if next_char_lower == 'n' && i + 2 < len && ['g', 'k'].contains(&next_next_char_lower) {
                     ipa_result.push_str("ɔː");
                     i += 1;
                     continue;
                }
                ipa_result.push('ɒ');
                i += 1;
                continue;
            }
            'u' => {
                if next_char_lower == 'e' && (i + 2 >= len || !chars[i + 2].is_alphabetic()) {
                    ipa_result.push_str("juː");
                    i += 2;
                    continue;
                }
                else if next_char_lower == 'i' {
                     ipa_result.push_str("juː");
                     i += 2;
                     continue;
                }
                else if next_char_lower == 'r' && i + 2 < len && is_vowel(next_next_char_lower) {
                     ipa_result.push_str("ɜː");
                     i += 1;
                     continue;
                }
                else if next_char_lower == 'r' && next_next_char_lower == 'e' {
                     ipa_result.push_str("jʊə");
                     i += 3;
                     continue;
                }
                else if i + 2 < len && is_consonant(next_char_lower) && next_next_char_lower == 'e' && (i + 3 >= len || !chars[i + 3].is_alphabetic()) {
                     ipa_result.push_str("juː");
                     i += 3;
                     continue;
                }
                else if next_char_lower == 'i' && next_next_char_lower == 'r' {
                     ipa_result.push_str("aɪə");
                     i += 3;
                     continue;
                }
                else if next_char_lower == 'o' && i + 2 < len && ['l', 'r', 's'].contains(&next_next_char_lower) {
                     ipa_result.push('ʊ');
                     i += 2;
                     continue;
                }
                ipa_result.push('ʌ');
                i += 1;
                continue;
            }
            'c' => {
                if ['e', 'i', 'y'].contains(&next_char_lower) {
                    ipa_result.push('s');
                } else {
                    ipa_result.push('k');
                }
                i += 1;
                continue;
            }
            'g' => {
                if ['e', 'i', 'y'].contains(&next_char_lower) && !(i > 0 && prev_char_lower == 'g') {
                    if next_char_lower == 'h' {
                        if i + 2 == len || ['t', 's', 'u'].contains(&next_next_char_lower) {
                        } else {
                            ipa_result.push_str("dʒ");
                        }
                    } else {
                        ipa_result.push_str("dʒ");
                    }
                } else {
                    ipa_result.push('ɡ');
                }
                i += 1;
                continue;
            }
            'h' => {
                if i == 0 && is_vowel(next_char_lower) {
                    ipa_result.push('h');
                } else if i > 0 && is_vowel(prev_char_lower) && is_vowel(next_char_lower) {
                    ipa_result.push('h');
                } else if i > 0 && is_consonant(prev_char_lower) {
                } else {
                    ipa_result.push('h');
                }
                i += 1;
                continue;
            }
            't' => {
                if next_char_lower == 'h' {
                    let is_voiced = if i + 2 < len && next_next_char_lower != '\0' {
                        is_vowel(next_next_char_lower) && (is_vowel(prev_char_lower) || is_liquid_or_nasal(prev_char_lower))
                    } else {
                        false
                    };
                    if is_voiced {
                        ipa_result.push_str("ð");
                    } else {
                        ipa_result.push_str("θ");
                    }
                    i += 2;
                    continue;
                }
                else if next_char_lower == 'i' && next_next_char_lower == 'o' && i + 3 < len && next_next_next_char_lower == 'n' {
                     if prev_char_lower == 's' || (prev_char_lower == 'l' || prev_char_lower == 'n') {
                         ipa_result.push_str("ʒ");
                     }
                     else if prev_char_lower != '\0' && is_consonant(prev_char_lower) {
                         ipa_result.push_str("ʃ");
                     } else {
                         ipa_result.push_str("t");
                         i += 1;
                         continue;
                     }
                     i += 3;
                     ipa_result.push('ə');
                     ipa_result.push('n');
                     i += 1;
                     continue;
                }
                else if next_char_lower == 'c' && next_next_char_lower == 'h' {
                     ipa_result.push_str("tʃ");
                     i += 3;
                     continue;
                }
                else if next_char_lower == 't' && next_next_char_lower == 'l' && next_next_next_char_lower == 'e' && (i + 4 >= len || !chars[i + 4].is_alphabetic()) {
                     ipa_result.push('t');
                     ipa_result.push('ə');
                     ipa_result.push('l');
                     i += 4;
                     continue;
                }
                ipa_result.push('t');
                i += 1;
                continue;
            }
             's' => {
                let is_z_sound = if i + 1 < len && next_char_lower != '\0' {
                    is_vowel(prev_char_lower) && is_vowel(next_char_lower)
                } else {
                     is_vowel(prev_char_lower)
                };
                if is_z_sound {
                    ipa_result.push('z');
                }
                else if next_char_lower == 'h' {
                     ipa_result.push_str("ʃ");
                     i += 2;
                     continue;
                }
                else if next_char_lower == 'i' && next_next_char_lower == 'o' && i + 3 < len && next_next_next_char_lower == 'n' {
                     ipa_result.push_str("ʒ");
                     i += 3;
                     ipa_result.push('ə');
                     ipa_result.push('n');
                     i += 1;
                     continue;
                }
                else if next_char_lower == 's' {
                     ipa_result.push('s');
                     i += 2;
                     continue;
                }
                else {
                    ipa_result.push('s');
                }
                i += 1;
                continue;
            }
            'y' => {
                if i == 0 && next_char_lower != '\0' {
                    ipa_result.push('j');
                }
                else if i == len - 1 {
                    if is_vowel(prev_char_lower) {
                        ipa_result.push('i');
                    } else {
                        ipa_result.push_str("aɪ");
                    }
                }
                else {
                    ipa_result.push('i');
                }
                i += 1;
                continue;
            }
            'q' => {
                if next_char_lower == 'u' {
                    ipa_result.push_str("kw");
                    i += 2;
                    continue;
                } else {
                    ipa_result.push('k');
                    i += 1;
                    continue;
                }
            }
            'w' => {
                if next_char_lower == 'h' {
                    ipa_result.push_str("ʍ");
                    i += 2;
                    continue;
                }
                ipa_result.push('w');
                i += 1;
                continue;
            }
            'x' => {
                ipa_result.push_str("ks");
                i += 1;
                continue;
            }
            'k' => {
                if next_char_lower == 'n' && i == 0 {
                    i += 1;
                    continue;
                }
                else if i > 0 && prev_char_lower == 'c' && next_char_lower == 'n' {
                     ipa_result.push('k');
                     i += 1;
                     continue;
                }
                ipa_result.push('k');
                i += 1;
                continue;
            }
            _ => {
                if let Some(&ipa) = BASE_LETTERS_IPA_MAP.get(&current_char_lower) {
                    ipa_result.push_str(ipa);
                } else {
                    eprintln!("Warning: Unknown character '{}' in fallback G2P for word '{}'", chars[i], word);
                }
                i += 1;
                continue;
            }
        }
    }
    if word.len() > 2 && word.to_lowercase().ends_with("ed") && ipa_result.len() >= 2 {
        let len = ipa_result.len();
        if len >= 3 {
             let ipa_chars: Vec<char> = ipa_result.chars().collect();
             let last_sound_char = ipa_chars[len - 3];
             let second_last_sound_char = if len >= 4 { ipa_chars[len - 4] } else { '\0' };
             if last_sound_char == 't' || last_sound_char == 'd' {
                 ipa_result.pop();
                 ipa_result.push_str("ɪd");
             }
             else if ['p', 'k', 'f', 's', 'ʃ', 'θ'].contains(&last_sound_char) ||
                     (last_sound_char == 'ʃ' && second_last_sound_char == 't') {
                 ipa_result.pop();
                 ipa_result.push('t');
             }
        }
    }
    if word.len() > 3 && word.to_lowercase().ends_with("ing") && ipa_result.len() >= 3 {
         let len = ipa_result.len();
         if len >= 3 {
             let chars_rev: Vec<char> = ipa_result.chars().rev().collect();
             if chars_rev.len() >= 3 && chars_rev[1] == 'n' && (chars_rev[2] == 'i' || chars_rev[2] == 'e') {
                 ipa_result.replace_range(len - 3..len - 1, "ɪŋ");
             } else if chars_rev.len() >= 2 && chars_rev[1] == 'n' {
                 ipa_result.replace_range(len - 2..len, "ŋ");
             } else {
                 if !(ipa_result.ends_with("ɪŋ") || ipa_result.ends_with("iŋ") || ipa_result.ends_with("eng")) {
                     ipa_result.push_str("ɪŋ");
                 }
             }
         }
    }
    if word.len() > 2 && word.to_lowercase().ends_with("es") && ipa_result.len() >= 2 {
        if let Some(penultimate_char) = ipa_result.chars().nth(ipa_result.len().saturating_sub(2)) {
            if ['s', 'ʃ', 'z', 'ʒ', 'θ'].contains(&penultimate_char) ||
               (penultimate_char == 't' && ipa_result.len() >= 3 && ipa_result.chars().nth(ipa_result.len().saturating_sub(3)) == Some('t')) || // tʃ
               (penultimate_char == 'd' && ipa_result.len() >= 3 && ipa_result.chars().nth(ipa_result.len().saturating_sub(3)) == Some('t')) { // dʒ
                if ipa_result.ends_with('s') {
                    ipa_result.pop();
                    ipa_result.push_str("ɪz");
                } else if ipa_result.ends_with('z') {
                    ipa_result.pop();
                    ipa_result.push_str("ɪz");
                } else if ipa_result.ends_with("ʃ") {
                    ipa_result.pop();
                    ipa_result.push_str("ɪz");
                } else if ipa_result.ends_with("ʒ") {
                    ipa_result.pop();
                    ipa_result.push_str("ɪz");
                } else if ipa_result.ends_with("θ") {
                    ipa_result.pop();
                    ipa_result.push_str("ɪz");
                } else if ipa_result.len() >= 2 && 
                         ipa_result.chars().nth(ipa_result.len()-2) == Some('t') && 
                         ipa_result.ends_with("ʃ") {
                    // tʃ
                    ipa_result.pop();
                    ipa_result.pop();
                    ipa_result.push_str("tʃɪz");
                } else if ipa_result.len() >= 2 && 
                         ipa_result.chars().nth(ipa_result.len()-2) == Some('d') && 
                         ipa_result.ends_with("ʒ") {
                    // dʒ
                    ipa_result.pop();
                    ipa_result.pop();
                    ipa_result.push_str("dʒɪz");
                }
            }
        }
    }
    if word.len() > 3 && word.to_lowercase().ends_with("ers") && ipa_result.len() >= 3 {
        if let Some(last_char) = ipa_result.chars().last() {
             if ['r', 'l', 'n', 'm', 'd', 't', 'k', 'ɡ', 'p', 'b', 's', 'z', 'f', 'v', 'θ', 'ð'].contains(&last_char) ||
                (last_char == 'ʃ' && ipa_result.len() >= 2 && ipa_result.chars().nth(ipa_result.len()-2) != Some('t')) || // Not tʃ
                (last_char == 'ʒ' && ipa_result.len() >= 2 && ipa_result.chars().nth(ipa_result.len()-2) != Some('d')) { // Not dʒ
                  ipa_result.push_str("ərz");
             } else {
                  ipa_result.push_str("rz");
             }
        }
    }
    if word.len() > 3 && word.to_lowercase().ends_with("est") && ipa_result.len() >= 3 {
        if let Some(last_char) = ipa_result.chars().last() {
             if ['t', 'd', 's', 'z', 'ʃ', 'ʒ', 'θ'].contains(&last_char) ||
                (last_char == 'ʃ' && ipa_result.len() >= 2 && ipa_result.chars().nth(ipa_result.len()-2) == Some('t')) || // tʃ
                (last_char == 'ʒ' && ipa_result.len() >= 2 && ipa_result.chars().nth(ipa_result.len()-2) == Some('d')) { // dʒ
                  ipa_result.push_str("ɪst");
             } else {
                  ipa_result.push_str("st");
             }
        }
    }
    if word.len() > 2 && word.to_lowercase().ends_with("ly") && ipa_result.len() >= 2 {
        if ipa_result.ends_with("i") {
            ipa_result.pop();
            ipa_result.push_str("aɪli");
        } else if ipa_result.ends_with("l") {
            ipa_result.push_str("li");
        } else {
            ipa_result.push_str("li");
        }
    }
    if word.len() > 4 && word.to_lowercase().ends_with("tion") && ipa_result.len() >= 4 {
        ipa_result.replace_range(ipa_result.len()-4..ipa_result.len()-2, "ʃ");
        ipa_result.push_str("ən");
    }
    if word.len() > 4 && word.to_lowercase().ends_with("sion") && ipa_result.len() >= 4 {
        if ipa_result.len() >= 5 {
            let chars_rev: Vec<char> = ipa_result.chars().rev().collect();
            if chars_rev.len() >= 5 && chars_rev[3] == 's' {
                ipa_result.replace_range(ipa_result.len()-4..ipa_result.len()-2, "ʒ");
                ipa_result.push_str("ən");
            } else {
                ipa_result.replace_range(ipa_result.len()-4..ipa_result.len()-2, "ʃ");
                ipa_result.push_str("ən");
            }
        } else {
            ipa_result.replace_range(ipa_result.len()-4..ipa_result.len()-2, "ʃ");
            ipa_result.push_str("ən");
        }
    }
    if word.len() > 1 && word.to_lowercase().ends_with('e') && ipa_result.ends_with("ɛ") {
        ipa_result.pop();
    }
    if !ipa_result.contains("ˈ") && !ipa_result.contains("ˌ") && !ipa_result.is_empty() {
        let mut stressed_ipa = String::new();
        let ipa_chars: Vec<char> = ipa_result.chars().collect();
        let ipa_len = ipa_chars.len();
        let mut primary_added = false;
        let mut i = 0;
        let ipa_vowel_chars = ['a', 'e', 'i', 'o', 'u', 'æ', 'ɛ', 'ɪ', 'ɑ', 'ɔ', 'ʊ', 'ʌ', 'ə', 'ɜ', 'ɚ', 'ɝ', 'ɐ', 'ɵ', 'ɘ', 'ʏ', 'ø', 'œ', 'ɶ'];
        let ipa_vowel_strings = ["ɑ̃", "ɛ̃", "ɔ̃", "œ̃"];
        let ipa_diphthongs_triphthongs_r_colored = ["eɪ", "aɪ", "ɔɪ", "aʊ", "oʊ", "ɪə", "eə", "ʊə", "aɪə", "aʊə", "ɔɪə", "ɜː", "ɪr", "ɛr", "ɑr", "ɔr", "ʊr", "ʌr", "ər"];
        let is_ipa_vowel = |c: char| ipa_vowel_chars.contains(&c) || ipa_vowel_strings.iter().any(|&s| s.chars().next() == Some(c)) || ipa_diphthongs_triphthongs_r_colored.iter().any(|&s| s.chars().next() == Some(c));
        while i < ipa_len {
            let current_ipa = ipa_chars[i];
            if i + 2 < ipa_len {
                let potential_three_char = format!("{}{}{}", current_ipa, ipa_chars[i+1], ipa_chars[i+2]);
                if ipa_diphthongs_triphthongs_r_colored.contains(&potential_three_char.as_str()) ||
                   ipa_vowel_strings.iter().any(|&s| s == potential_three_char.as_str()) ||
                   ["aɪə", "aʊə", "ɔɪə"].contains(&potential_three_char.as_str()) {
                     if !primary_added {
                         stressed_ipa.push('ˈ');
                         primary_added = true;
                     } else {
                         stressed_ipa.push('ˌ');
                     }
                     stressed_ipa.push_str(&potential_three_char);
                     i += 3;
                     continue;
                }
            }
            if i + 1 < ipa_len {
                let potential_two_char = format!("{}{}", current_ipa, ipa_chars[i+1]);
                if ipa_diphthongs_triphthongs_r_colored.contains(&potential_two_char.as_str()) ||
                   ipa_vowel_strings.iter().any(|&s| s == potential_two_char.as_str()) ||
                   ["eɪ", "aɪ", "ɔɪ", "aʊ", "oʊ", "ɪə", "eə", "ʊə", "ɜː", "ɪr", "ɛr", "ɑr", "ɔr", "ʊr", "ʌr", "ər"].contains(&potential_two_char.as_str()) {
                    if !primary_added {
                        stressed_ipa.push('ˈ');
                        primary_added = true;
                    } else {
                        stressed_ipa.push('ˌ');
                    }
                    stressed_ipa.push_str(&potential_two_char);
                    i += 2;
                    continue;
                }
            }
            if is_ipa_vowel(current_ipa) {
                if !primary_added {
                    stressed_ipa.push('ˈ');
                    primary_added = true;
                } else {
                    stressed_ipa.push('ˌ');
                }
            }
            stressed_ipa.push(current_ipa);
            i += 1;
        }
        if primary_added {
             ipa_result = stressed_ipa;
        } else {
            ipa_result = format!("ˈ{}", ipa_result);
        }
    }
    ipa_result
}

pub fn arpa_to_ipa(token: &str) -> String {
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

pub fn convert_under_thousand(n: u64) -> String {
    let ones = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen",
                "seventeen", "eighteen", "nineteen"];
    let tens = ["", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
    if n == 0 {
        return String::new();
    }
    if n < 20 {
        return ones[n as usize].to_string();
    }
    if n < 100 {
        let ten_part = tens[(n / 10) as usize];
        let one_part = if n % 10 != 0 { &ones[(n % 10) as usize] } else { "" };
        if !one_part.is_empty() {
            return format!("{} {}", ten_part, one_part);
        } else {
            return ten_part.to_string();
        }
    }
    let hundreds_digit = n / 100;
    let remainder = n % 100;
    let hundred_part = format!("{} hundred", ones[hundreds_digit as usize]);
    if remainder == 0 {
        return hundred_part;
    } else {
        let under_hundred = convert_under_thousand(remainder);
        return format!("{} and {}", hundred_part, under_hundred);
    }
}

pub fn number_to_words(mut num: u64) -> String {
    if num == 0 {
        return "zero".to_string();
    }
    let scales = ["", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion"];
    let mut parts = Vec::new();
    let mut scale_index = 0;
    while num > 0 {
        let chunk = num % 1000;
        if chunk != 0 {
            let chunk_words = convert_under_thousand(chunk);
            if !scales[scale_index].is_empty() {
                parts.push(format!("{} {}", chunk_words, scales[scale_index]));
            } else {
                parts.push(chunk_words);
            }
        }
        num /= 1000;
        scale_index += 1;
    }
    parts.reverse();
    parts.join(" ")
}

pub fn string_number_to_words(number_str: &str) -> String {
    if let Some(dot_pos) = number_str.find('.') {
        let integer_part = &number_str[..dot_pos];
        let fractional_part = &number_str[dot_pos + 1..];
        let mut result = String::new();
        let cleaned_integer: String = integer_part.chars().filter(|c| c.is_ascii_digit()).collect();
        if !cleaned_integer.is_empty() && cleaned_integer != "0" {
             if let Ok(int_num) = cleaned_integer.parse::<u64>() {
                result.push_str(&number_to_words(int_num));
             } else {
                result.push_str(integer_part);
             }
             result.push_str(" point ");
        } else if cleaned_integer == "0" {
            result.push_str("zero point ");
        } else {
            result.push_str("point ");
        }
        for digit_char in fractional_part.chars() {
            if digit_char.is_ascii_digit() {
                let digit_word = match digit_char {
                    '0' => "zero", '1' => "one", '2' => "two", '3' => "three", '4' => "four",
                    '5' => "five", '6' => "six", '7' => "seven", '8' => "eight", '9' => "nine",
                    _ => "",
                };
                result.push_str(digit_word);
                result.push(' ');
            }
        }
        result.trim_end().to_string()
    } else {
        let cleaned_str: String = number_str.chars().filter(|c| c.is_ascii_digit()).collect();
        if let Ok(num) = cleaned_str.parse::<u64>() {
            number_to_words(num)
        } else {
            eprintln!("Warning: Failed to parse large integer string '{}'", number_str);
            number_str.to_string()
        }
    }
}

pub fn word2ipa_with_context(word: &str, context_before: &str, context_after: &str) -> String {
    let lower_word = word.to_lowercase();
    if let Some(rules) = HETERONYMS.get(lower_word.as_str()) {
        let full_context = format!("{} {} {}", context_before, word, context_after);
        for (pattern, ipa) in rules {
            let re = Regex::new(pattern).unwrap();
            if re.is_match(&full_context) {
                return ipa.to_string();
            }
        }
    }
    if let Some(entries) = CMU.get(&lower_word) {
        if entries.len() == 1 {
             return entries[0].pronunciation()
                .iter()
                .map(|p| arpa_to_ipa(&p.to_string()))
                .collect();
        } else {
            return entries[0].pronunciation()
                .iter()
                .map(|p| arpa_to_ipa(&p.to_string()))
                .collect();
        }
    }
    letters_to_ipa(word)
}

pub fn g2p_with_context(text: &str) -> String {
    let re = Regex::new(r#"[\w']+(?:'[\w']+)*|\d{1,3}(?:,\d{3})*(?:\.\d+)?|\d+(?:\.\d+)?|\W+"#).unwrap();
    let tokens: Vec<&str> = re.find_iter(text).map(|mat| mat.as_str()).collect();
    let mut out = String::new();
    for (i, &tok) in tokens.iter().enumerate() {
        if tok.chars().all(|c| c.is_ascii_digit() || c == ',' || c == '.') && tok.chars().any(|c| c.is_ascii_digit()) {
             out.push_str(&word2ipa(&string_number_to_words(tok)));
        } else if tok.chars().all(|c| c.is_alphanumeric() || c == '\'') {
            let context_before = if i > 0 { tokens[i - 1] } else { "" };
            let context_after = if i + 1 < tokens.len() { tokens[i + 1] } else { "" };
            out.push_str(&word2ipa_with_context(tok, context_before, context_after));
        } else {
            out.push_str(tok);
        }
    }
    out.trim().to_string()
}

pub fn g2p(text: &str) -> String {
    g2p_with_context(text)
}

pub mod tokenizer {
    use super::*;
    use serde_json::Value;
    use std::{collections::HashMap, fs};
    pub static CHAR_TO_ID: Lazy<HashMap<char, i64>> = Lazy::new(|| {
        let txt = fs::read_to_string(TTS_TOKENIZER_PATH).unwrap();
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
}

pub mod loader {
    use super::*;
    use ndarray::Dim;
    use ort::{
        execution_providers::{CPUExecutionProvider, DirectMLExecutionProvider},
        session::builder::GraphOptimizationLevel,
        session::Session,
    };
    pub static VOICE_PACK: LazyLock<Mutex<Vec<Vec<f32>>>> = LazyLock::new(|| Mutex::new(Vec::new()));
    pub static MODEL:      LazyLock<Mutex<Option<Arc<Session>>>> = LazyLock::new(|| Mutex::new(None));
    pub async fn load<P: AsRef<Path>>(model_path: P, voice_file: P) {
        let buf = tokio::fs::read(&voice_file).await.unwrap();
        let floats: Vec<f32> = buf.chunks_exact(4)
            .map(|b| f32::from_le_bytes([b[0],b[1],b[2],b[3]]))
            .collect();
        *VOICE_PACK.lock().unwrap() = floats.chunks(256).map(|c| c.to_vec()).collect();
        let cores = available_parallelism().unwrap().get();
        let cpu = CPUExecutionProvider::default().build();
        let dml = DirectMLExecutionProvider::default().build();
        let session = Session::builder().unwrap()
            .with_optimization_level(GraphOptimizationLevel::Level3).unwrap()
            .with_intra_threads(cores).unwrap()
            .with_inter_threads(cores).unwrap()
            .with_execution_providers([cpu, dml]).unwrap()
            .commit_from_file(model_path).unwrap();
        *MODEL.lock().unwrap() = Some(Arc::new(session));
    }
    pub fn style_for(token_index: usize) -> Array<f32, Ix2> {
        let pack = VOICE_PACK.lock().unwrap();
        if token_index >= pack.len() {
            return Array::zeros(Dim([1, 256]));
        }
        Array::from_shape_vec(Dim([1, pack[token_index].len()]), pack[token_index].clone()).unwrap()
    }
    pub fn session() -> Arc<Session> {
        MODEL.lock().unwrap().as_ref().unwrap().clone()
    }
    pub fn is_loaded() -> bool {
        MODEL.lock().unwrap().is_some()
    }
}

pub mod synthesizer {
    use super::*;
    use std::time::Instant;
    pub async fn synth(text: String, speed: f32) -> (Vec<f32>, Duration) {
        if !loader::is_loaded() {
            eprintln!("Warning: TTS model not loaded before synthesis. Attempting to load...");
        }
        let phoneme_str = tts::g2p(&text);
        println!("> Phonemes: {}", phoneme_str);
        let ids = tokenizer::get_token_ids(&phoneme_str);
        let arr = Array::from_shape_vec((1, ids.len()), ids.clone()).unwrap();
        let style_index = if arr.shape()[1] > 1 { arr.shape()[1] - 1 } else { 0 };
        let style = loader::style_for(style_index);
        let speed_arr = Array::from_vec(vec![speed]);
        let model = loader::session();
        let t0 = Instant::now();
        let inputs = inputs![
            "input_ids" => arr.view(),
            "style"     => style.view(),
            "speed"     => speed_arr.view()
        ].unwrap();
        let outputs = model.run_async(inputs).unwrap().await.unwrap();
        let took = t0.elapsed();
        let wave = outputs["waveform"]
            .try_extract_tensor::<f32>().unwrap()
            .as_slice().unwrap().to_vec();
        (wave, took)
    }
}

pub async fn ensure_tts_model_loaded() {
    if !TTS_MODEL_LOADED.load(Ordering::Relaxed) {
        println!("Loading TTS model...");
        let selected_voice_filename = SELECTED_VOICE_PATH.lock().unwrap().clone();
        let full_voice_path = format!("voices/{}", selected_voice_filename);
        loader::load(TTS_MODEL_PATH, &full_voice_path).await;
        TTS_MODEL_LOADED.store(true, Ordering::Relaxed);
        println!("TTS model loaded.");
    }
}

pub fn process_tts(text: &str, enabled: &Arc<AtomicBool>, tts_stop_flag: Arc<AtomicBool>) {
    tts_stop_flag.store(false, Ordering::Relaxed);
    let text_for_tts = strip_code_blocks(text);
    if enabled.load(Ordering::Relaxed) && !text_for_tts.trim().is_empty() {
        let text_clone = text_for_tts.clone();
        let flag_clone = tts_stop_flag.clone();
        let flag_clone_synthesis = tts_stop_flag.clone();
        tokio::spawn(async move {
            ensure_tts_model_loaded().await;
            let (tx, rx) = sync_channel::<(Vec<f32>, String)>(2);
            let synthesis_handle = std::thread::spawn(move || {
                let sentences: Vec<&str> = text_clone.split(|c| c == '.' || c == '!' || c == '?').collect();
                for sentence in sentences {
                    if flag_clone_synthesis.load(Ordering::Relaxed) {
                        break;
                    }
                    let trimmed_sentence = sentence.trim();
                    if !trimmed_sentence.is_empty() {
                        let (wave, _duration) = block_on(synthesizer::synth(trimmed_sentence.to_string(), 1.0));
                        if let Err(_send_err) = tx.send((wave, trimmed_sentence.to_string())) {
                            break;
                        }
                    }
                }
            });
            {
                 while let Ok((wave, sentence_text)) = rx.recv() {
                    if flag_clone.load(Ordering::Relaxed) {
                        break;
                    }
                    let flag_clone_inner = flag_clone.clone();
                    let result = tokio::task::spawn_blocking(move || {
                        play_waveform(wave, flag_clone_inner)
                    }).await;
                    if let Ok(Err(e)) = result {
                        eprintln!("TTS Playback error for sentence '{}': {:?}", sentence_text, e);
                    }
                    if flag_clone.load(Ordering::Relaxed) {
                        break;
                    }
                }
            }
            let _ = synthesis_handle.join();
        });
    }
}