
use eframe::egui;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound;
use std::{collections::{hash_map::DefaultHasher, HashMap},fs,hash::{Hash, Hasher},io::Read,process::{Command, Stdio},sync::{atomic::{AtomicBool, Ordering},mpsc::{self, SyncSender, sync_channel},Arc, Mutex},thread,time::{Duration, Instant, SystemTime, UNIX_EPOCH}};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use bincode::serde::{encode_to_vec, decode_from_slice};
use bincode::config;
use ndarray::{Array, Ix2};
use ort::inputs;
use std::sync::LazyLock;
use std::thread::available_parallelism;
use std::path::Path;
use regex::Regex;
use cmudict_fast::Cmudict;
use once_cell::sync::Lazy;
use syntect::{highlighting::ThemeSet, parsing::SyntaxSet, easy::HighlightLines};
use simple_transcribe_rs::{transcriber::Transcriber, model_handler::ModelHandler};
use futures::executor::block_on;
use rfd::FileDialog;
use image::{ImageOutputFormat, imageops::FilterType};
use base64::{engine::general_purpose, Engine};
use crate::tts::{process_tts, AVAILABLE_VOICES};
pub mod heteronyms;
pub mod contractions;
pub mod tts;


const SETTINGS_FILE: &str = "settings.json";
const MEMORY_FILE: &str = "memory.bin";
const TEMP_AUDIO_FILE: &str = "temp_audio.wav";
const TTS_MODEL_PATH: &str = "onnx/model.onnx";
const TTS_CMU_DICT_PATH: &str = "cmudict.dict";
const TTS_TOKENIZER_PATH: &str = "tokenizer.json";
static TTS_MODEL_LOADED: AtomicBool = AtomicBool::new(false);
static SELECTED_VOICE_PATH: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new("af_bella.bin".to_string()));

#[derive(Clone, PartialEq)]
enum Sender {
    User,
    Model,
    System,
}

#[derive(Clone)]
struct ChatBubble {
    content: String,
    attachment_content: Option<String>,
    sender: Sender,
    is_thinking: bool,
    is_code: bool,
    language: Option<String>,
    id: egui::Id,
    timestamp: Option<Instant>,
    persistent: bool,
}

enum BubbleMessage {
    New(ChatBubble),
    Update { id: egui::Id, content: String },
    Remove(egui::Id),
}

#[derive(Serialize, Deserialize)]
struct AppSettings {
    api_url: String,
    selected_model: String,
    models: Vec<String>,
    tts_enabled: bool,
    streaming_enabled: bool,
    temperature: f32,
    top_p: f32,
    min_p: f32,
    top_k: u32,
    repeat_penalty: f32,
    max_completion_tokens: u32,
    send_stt: bool,
    selected_voice: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            api_url: "http://localhost:1234/v1/chat/completions".to_owned(),
            selected_model: "Select Model".to_owned(),
            models: vec!["Select Model".to_owned()],
            tts_enabled: true,
            streaming_enabled: false,
            temperature: 0.8,
            top_p: 0.95,
            min_p: 0.05,
            top_k: 40,
            repeat_penalty: 1.1,
            max_completion_tokens: 10024,
            send_stt: false,
            selected_voice: "af_bella.bin".to_owned(),
        }
    }
}

fn unique_id(prefix: &str, content: &str) -> egui::Id {
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    let hash = hasher.finish();
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    egui::Id::new(format!("{}-{}-{}", prefix, now, hash))
}


fn save_memory(memory: &Vec<Value>) {
    if let Ok(serialized) = encode_to_vec(memory, config::standard()) {
        let _ = fs::write(MEMORY_FILE, serialized);
    }
}

fn load_memory() -> Vec<Value> {
    if let Ok(data) = fs::read(MEMORY_FILE) {
        if let Ok((mem_vec, _)) =
            decode_from_slice::<Vec<Value>, _>(&data, config::standard())
        {
            return mem_vec;
        }
    }
    vec![serde_json::json!({"role":"system", "content": "You are an AI companion.", "id":"system-0"})]
}

fn save_app_settings(settings: &AppSettings) {
    if let Ok(json_str) = serde_json::to_string_pretty(settings) {
        let _ = fs::write(SETTINGS_FILE, json_str);
    }
}

fn load_app_settings() -> AppSettings {
    if let Ok(mut file) = fs::File::open(SETTINGS_FILE) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            if let Ok(s) = serde_json::from_str::<AppSettings>(&contents) {
                return s;
            }
        }
    }
    AppSettings::default()
}

fn render_markdown(ui: &mut egui::Ui, markdown: &str) {
    for line in markdown.lines() {
        ui.add(egui::Label::new(egui::RichText::new(line).color(egui::Color32::WHITE)).wrap());
    }
}

fn render_collapsible_bubble<F: FnOnce(&mut egui::Ui)>(
    ui: &mut egui::Ui,
    label: &str,
    id: egui::Id,
    render_content: F,
) {
    egui::CollapsingHeader::new(label)
        .id_salt(id)
        .show(ui, |ui| {
            render_content(ui);
        });
}

fn highlight_code_job(code: &str, language: Option<&str>) -> egui::text::LayoutJob {
    use egui::{Color32, FontId, TextFormat};
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = if let Some(lang) = language {
        ps.find_syntax_by_token(lang)
            .unwrap_or_else(|| ps.find_syntax_by_extension("rs").unwrap_or_else(|| ps.find_syntax_plain_text()))
    } else {
        ps.find_syntax_by_extension("rs").unwrap_or_else(|| ps.find_syntax_plain_text())
    };
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    let mut job = egui::text::LayoutJob::default();
    let font = FontId::monospace(14.0);
    for line in code.lines() {
        if let Ok(ranges) = h.highlight_line(line, &ps) {
            for (style, text) in ranges {
                let color = Color32::from_rgb(style.foreground.r, style.foreground.g, style.foreground.b);
                job.append(text, 0.0, TextFormat { font_id: font.clone(), color, ..Default::default() });
            }
            job.append("\n", 0.0, TextFormat { font_id: font.clone(), color: Color32::WHITE, ..Default::default() });
        } else {
            job.append(line, 0.0, TextFormat { font_id: font.clone(), color: Color32::WHITE, ..Default::default() });
            job.append("\n", 0.0, TextFormat { font_id: font.clone(), color: Color32::WHITE, ..Default::default() });
        }
    }
    job
}

fn split_content_into_bubbles(sender: Sender, content: &str, is_thinking: bool) -> Vec<ChatBubble> {
    let mut bubbles = Vec::new();
    let mut remaining = content;
    while let Some(start) = remaining.find("```") {
        let text_before = &remaining[..start];
        if !text_before.trim().is_empty() {
            bubbles.push(ChatBubble {
                sender: sender.clone(),
                content: text_before.to_string(),
                attachment_content: None,
                is_thinking,
                is_code: false,
                language: None,
                id: unique_id("bubble", text_before),
                timestamp: None,
                persistent: true,
            });
        }
        let after_ticks = &remaining[start + 3..];
        let (language, code_start) = if let Some(newline_index) = after_ticks.find('\n') {
            let potential_lang = after_ticks[..newline_index].trim();
            if !potential_lang.is_empty() {
                (Some(potential_lang.to_string()), newline_index + 1)
            } else {
                (None, 0)
            }
        } else {
            (None, 0)
        };
        let code_content_start = start + 3 + code_start;
        if let Some(end) = remaining[code_content_start..].find("```") {
            let code_text = &remaining[code_content_start..code_content_start + end];
            if !code_text.trim().is_empty() {
                bubbles.push(ChatBubble {
                    sender: sender.clone(),
                    content: code_text.to_string(),
                    attachment_content: None,
                    is_thinking,
                    is_code: true,
                    language,
                    id: unique_id("code", code_text),
                    timestamp: None,
                    persistent: true,
                });
            }
            remaining = &remaining[code_content_start + end + 3..];
        } else {
            bubbles.push(ChatBubble {
                sender: sender.clone(),
                content: remaining[code_content_start..].to_string(),
                attachment_content: None,
                is_thinking,
                is_code: true,
                language,
                id: unique_id("code", &remaining[code_content_start..]),
                timestamp: None,
                persistent: true,
            });
            remaining = "";
            break;
        }
    }
    if !remaining.trim().is_empty() {
        bubbles.push(ChatBubble {
            sender,
            content: remaining.to_string(),
            attachment_content: None,
            is_thinking,
            is_code: false,
            language: None,
            id: unique_id("bubble", remaining),
            timestamp: None,
            persistent: true,
        });
    }
    bubbles
}

fn strip_code_blocks(text: &str) -> String {
    let mut result = String::new();
    let mut in_code = false;
    for line in text.lines() {
        if line.trim().starts_with("```") {
            in_code = !in_code;
            continue;
        }
        if !in_code {
            result.push_str(line);
            result.push('\n');
        }
    }
    result
}

fn send_bubbles(tx: &UnboundedSender<BubbleMessage>, sender: Sender, content: &str, is_thinking: bool) {
    for bubble in split_content_into_bubbles(sender, content, is_thinking) {
        let _ = tx.send(BubbleMessage::New(bubble));
    }
}

fn send_reasoning(tx: &UnboundedSender<BubbleMessage>, sender: Sender, reasoning: &str) {
    if reasoning.trim().is_empty() {
        return;
    }
    let bubble = ChatBubble {
        sender,
        content: reasoning.to_owned(),
        attachment_content: None,
        is_thinking: true,
        is_code: false,
        language: None,
        id: unique_id("reasoning", reasoning),
        timestamp: None,
        persistent: false,
    };
    let _ = tx.send(BubbleMessage::New(bubble));
}

async fn call_model(
    client: &Client,
    api_url: &str,
    model: &str,
    messages: Vec<Value>,
    return_reasoning: bool,
    temperature: f32,
    top_p: f32,
    min_p: f32,
    top_k: u32,
    repeat_penalty: f32,
    max_completion_tokens: u32,
) -> ModelResponse {
    let payload = json!({
        "model": model,
        "messages": messages,
        "return_reasoning": return_reasoning,
        "temperature": temperature,
        "top_p": top_p,
        "min_p": min_p,
        "top_k": top_k,
        "repeat_penalty": repeat_penalty,
        "max_completion_tokens": max_completion_tokens,
    });
    match client.post(api_url).json(&payload).send().await {
        Ok(response) => match response.json::<Value>().await {
            Ok(json_resp) => {
                let content = json_resp["choices"][0]["message"]["content"]
                    .as_str()
                    .unwrap_or("(No valid response received)")
                    .to_owned();
                let reasoning = json_resp["choices"][0]["message"]["reasoning_content"]
                    .as_str()
                    .map(|s| s.to_owned());
                ModelResponse { content, reasoning }
            }
            Err(_) => ModelResponse {
                content: "(Failed to parse response)".to_owned(),
                reasoning: None,
            },
        },
        Err(err) => ModelResponse {
            content: format!("Request failed: {}", err),
            reasoning: None,
        },
    }
}

async fn call_model_streaming(
    client: &Client,
    api_url: &str,
    model: &str,
    messages: Vec<Value>,
    return_reasoning: bool,
    temperature: f32,
    top_p: f32,
    min_p: f32,
    top_k: u32,
    repeat_penalty: f32,
    max_completion_tokens: u32,
    tx: UnboundedSender<BubbleMessage>,
    tts_enabled: Arc<AtomicBool>,
    tts_stop_flag: Arc<AtomicBool>,
    history_arc: Arc<Mutex<Vec<Value>>>,
) {
    let payload = json!({
        "model": model,
        "messages": messages,
        "return_reasoning": return_reasoning,
        "temperature": temperature,
        "top_p": top_p,
        "min_p": min_p,
        "top_k": top_k,
        "repeat_penalty": repeat_penalty,
        "max_completion_tokens": max_completion_tokens,
        "stream": true,
    });
    let mut response = match client.post(api_url).json(&payload).send().await {
        Ok(resp) => resp,
        Err(err) => {
            let _ = tx.send(BubbleMessage::New(ChatBubble {
                sender: Sender::Model,
                content: format!("Request failed: {}", err),
                attachment_content: None,
                is_thinking: false,
                is_code: false,
                language: None,
                id: unique_id("bubble", ""),
                timestamp: None,
                persistent: true,
            }));
            return;
        }
    };
    let mut accumulated_content = String::new();
    let mut accumulated_reasoning = String::new();
    let content_bubble_id = unique_id("stream_content", "");
    let reasoning_bubble_id = unique_id("stream_reasoning", "");
    let mut content_created = false;
    let mut reasoning_created = false;
    let mut leftover = String::new();
    while let Ok(Some(chunk)) = response.chunk().await {
        let chunk_str = String::from_utf8_lossy(&chunk).to_string();
        leftover.push_str(&chunk_str);
        while let Some(pos) = leftover.find('\n') {
            let line = leftover[..pos].to_string();
            leftover = leftover[pos + 1..].to_string();
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let data = if line.starts_with("data:") {
                line.trim_start_matches("data:").trim()
            } else {
                line
            };
            if data == "[DONE]" {
                break;
            }
            if let Ok(json_val) = serde_json::from_str::<Value>(data) {
                if let Some(choices) = json_val.get("choices").and_then(|c| c.as_array()) {
                    for choice in choices {
                        if let Some(delta) = choice.get("delta") {
                            if let Some(reasoning) = delta.get("reasoning_content").and_then(|r| r.as_str()) {
                                accumulated_reasoning.push_str(reasoning);
                                if !accumulated_reasoning.trim().is_empty() {
                                    if !reasoning_created {
                                        let _ = tx.send(BubbleMessage::New(ChatBubble {
                                            sender: Sender::Model,
                                            content: accumulated_reasoning.trim().to_owned(),
                                            attachment_content: None,
                                            is_thinking: true,
                                            is_code: false,
                                            language: None,
                                            id: reasoning_bubble_id,
                                            timestamp: None,
                                            persistent: false,
                                        }));
                                        reasoning_created = true;
                                    } else {
                                        let _ = tx.send(BubbleMessage::Update {
                                            id: reasoning_bubble_id,
                                            content: accumulated_reasoning.trim().to_owned(),
                                        });
                                    }
                                }
                            }
                            if let Some(content) = delta.get("content").and_then(|c| c.as_str()) {
                                accumulated_content.push_str(content);
                                if !accumulated_content.trim().is_empty() {
                                    if !content_created {
                                        let _ = tx.send(BubbleMessage::New(ChatBubble {
                                            sender: Sender::Model,
                                            content: accumulated_content.trim().to_owned(),
                                            attachment_content: None,
                                            is_thinking: false,
                                            is_code: false,
                                            language: None,
                                            id: content_bubble_id,
                                            timestamp: None,
                                            persistent: true,
                                        }));
                                        content_created = true;
                                    } else {
                                        let _ = tx.send(BubbleMessage::Update {
                                            id: content_bubble_id,
                                            content: accumulated_content.trim().to_owned(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if !leftover.is_empty() && content_created {
            let current_update = accumulated_content.clone() + leftover.as_str();
            if !current_update.trim().is_empty() {
                let _ = tx.send(BubbleMessage::Update {
                    id: content_bubble_id,
                    content: current_update.trim().to_owned(),
                });
            }
        }
    }
    if !leftover.is_empty() {
        accumulated_content.push_str(leftover.as_str());
    }
    accumulated_content = accumulated_content.trim_start().to_owned();
    let _ = tx.send(BubbleMessage::Remove(content_bubble_id));
    let _ = tx.send(BubbleMessage::Remove(reasoning_bubble_id));
    if !accumulated_reasoning.trim().is_empty() {
        send_reasoning(&tx, Sender::Model, accumulated_reasoning.trim());
    }
    if !accumulated_content.trim().is_empty() {
        send_bubbles(&tx, Sender::Model, accumulated_content.trim(), false);
    }
    if !accumulated_content.trim().is_empty() {
        let mut history = history_arc.lock().unwrap();
        history.push(json!({
            "role": "assistant",
            "content": accumulated_content.trim(),
            "id": format!("{:?}", content_bubble_id)
        }));
        save_memory(&*history);
    }
    if !accumulated_content.trim().is_empty() {
        process_tts(&accumulated_content, &tts_enabled, tts_stop_flag);
    }
}

struct ModelResponse {
    content: String,
    reasoning: Option<String>,
}

fn load_model(selected_model: &str) {
    Command::new("lms")
        .arg("load")
        .arg(selected_model)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to load model");
}

fn unload_model(selected_model: &str) {
    Command::new("lms")
        .arg("unload")
        .arg(selected_model)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to unload model");
}

fn heavy_transcribe(
    _stt_writer: Option<Arc<Mutex<Option<hound::WavWriter<std::io::BufWriter<fs::File>>>>>>,
) -> Result<String, String> {
    eprintln!("heavy_transcribe: Starting transcription...");
    let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    let join_handle = rt.spawn_blocking(move || {
        let model_handler = block_on(ModelHandler::new("small", "models/"));
        let transcriber = Transcriber::new(model_handler);
        transcriber.transcribe(TEMP_AUDIO_FILE, None).map_err(|e| e.to_string())
    });
    let result = rt.block_on(join_handle).map_err(|e| e.to_string())??;
    eprintln!("heavy_transcribe: Removing temporary file...");
    fs::remove_file(TEMP_AUDIO_FILE).map_err(|e| e.to_string())?;
    let text = result.get_text().to_string();
    eprintln!("heavy_transcribe: Transcribed text: {}", text);
    Ok(text)
}

struct ChatApp {
    input_text: String,
    chat_bubbles: Vec<ChatBubble>,
    conversation_history: Arc<Mutex<Vec<Value>>>,
    client: Client,
    api_url: String,
    selected_model: String,
    model_list: Vec<String>,
    temperature: f32,
    top_p: f32,
    min_p: f32,
    top_k: u32,
    repeat_penalty: f32,
    max_completion_tokens: u32,
    tts_enabled: Arc<AtomicBool>,
    streaming_enabled: bool,
    tts_stop_flag: Arc<AtomicBool>,
    experimental_reasoning: bool,
    scroll_to_bottom: bool,
    show_settings: bool,
    temp_api_url: String,
    new_model_name: String,
    conversation_channels: Vec<UnboundedReceiver<BubbleMessage>>,
    editing_bubble: Option<usize>,
    input_panel_height: f32,
    default_settings: AppSettings,
    code_layout_cache: HashMap<egui::Id, egui::text::LayoutJob>,
    stt_recording: bool,
    stt_active: Arc<AtomicBool>,
    stt_stream: Option<cpal::Stream>,
    audio_sample_tx: Option<SyncSender<Vec<i16>>>,
    audio_thread_handle: Option<thread::JoinHandle<()>>,
    send_stt: bool,
    transcription_tx: mpsc::Sender<String>,
    transcription_rx: mpsc::Receiver<String>,
    last_repaint: Instant,
    // --- Add selected_voice field ---
    selected_voice: String,
}

impl ChatApp {
    fn new() -> Self {
        let settings = load_app_settings();
        let (tx, rx) = mpsc::channel();
        // --- Initialize selected_voice from settings ---
        let selected_voice = settings.selected_voice.clone();
        // --- Update the global static with the loaded setting ---
        *SELECTED_VOICE_PATH.lock().unwrap() = selected_voice.clone();
        Self {
            input_text: String::new(),
            chat_bubbles: Vec::new(),
            conversation_history: Arc::new(Mutex::new(load_memory())),
            client: Client::new(),
            api_url: settings.api_url.clone(),
            selected_model: settings.selected_model.clone(),
            model_list: settings.models.clone(),
            temperature: settings.temperature,
            top_p: settings.top_p,
            min_p: settings.min_p,
            top_k: settings.top_k,
            repeat_penalty: settings.repeat_penalty,
            max_completion_tokens: settings.max_completion_tokens,
            tts_enabled: Arc::new(AtomicBool::new(settings.tts_enabled)),
            streaming_enabled: settings.streaming_enabled,
            tts_stop_flag: Arc::new(AtomicBool::new(false)),
            experimental_reasoning: true,
            scroll_to_bottom: false,
            show_settings: false,
            temp_api_url: settings.api_url.clone(),
            new_model_name: String::new(),
            conversation_channels: Vec::new(),
            editing_bubble: None,
            input_panel_height: 60.0,
            default_settings: AppSettings::default(),
            code_layout_cache: HashMap::new(),
            stt_recording: false,
            stt_active: Arc::new(AtomicBool::new(false)),
            stt_stream: None,
            audio_sample_tx: None,
            audio_thread_handle: None,
            send_stt: settings.send_stt,
            transcription_tx: tx,
            transcription_rx: rx,
            last_repaint: Instant::now(),
            // --- Initialize selected_voice ---
            selected_voice,
        }
    }

    fn handle_file_upload(&mut self) {
        let allowed_extensions = [
            "plaintext", "docx", "pdf", "rs", "toml", "png", "jpeg", "jpg", "webp", "gif",
        ];
        if let Some(path) = FileDialog::new()
            .add_filter("Allowed files", &allowed_extensions)
            .pick_file()
        {
            let filename = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let ext = path
                .extension()
                .map(|e| e.to_string_lossy().to_lowercase())
                .unwrap_or_default();
            let header = format!("[Upload: {}]", filename);
            if allowed_extensions.contains(&ext.as_str()) {
                if ["png", "jpeg", "jpg", "webp", "gif"].contains(&ext.as_str()) {
                    if let Ok(metadata) = path.metadata() {
                        if metadata.len() > 20 * 1024 * 1024 {
                            self.input_text.push_str("[Error: Image file too large (>20MB)]\n");
                            return;
                        }
                    }
                    match image::open(&path) {
                        Ok(img) => {
                            let resized = img.resize_exact(512, 512, FilterType::Lanczos3);
                            let mut buffer = Vec::new();
                            if resized.write_to(&mut std::io::Cursor::new(&mut buffer), ImageOutputFormat::Png).is_ok() {
                                let encoded = general_purpose::STANDARD.encode(&buffer);
                                let data_url = format!("data:image/png;base64,{}", encoded);
                                self.input_text.push_str(&header);
                                self.chat_bubbles.push(ChatBubble {
                                    sender: Sender::User,
                                    content: header.clone(),
                                    attachment_content: Some(data_url),
                                    is_thinking: false,
                                    is_code: false,
                                    language: None,
                                    id: unique_id("attachment", &header),
                                    timestamp: Some(Instant::now()),
                                    persistent: true,
                                });
                            }
                        }
                        Err(e) => {
                            self.input_text.push_str(&format!("[Error: Failed to open image file: {}]\n", e));
                        }
                    }
                } else {
                    match fs::read(&path) {
                        Ok(bytes) => {
                            let full_text = match String::from_utf8(bytes.clone()) {
                                Ok(text) => text,
                                Err(_) => general_purpose::STANDARD.encode(&bytes),
                            };
                            self.input_text.push_str(&header);
                            self.chat_bubbles.push(ChatBubble {
                                sender: Sender::User,
                                content: header.clone(),
                                attachment_content: Some(full_text),
                                is_thinking: false,
                                is_code: false,
                                language: None,
                                id: unique_id("attachment", &header),
                                timestamp: Some(Instant::now()),
                                persistent: true,
                            });
                        }
                        Err(e) => {
                            self.input_text.push_str(&format!("[Error: Failed to read file: {}]\n", e));
                        }
                    }
                }
            } else {
                self.input_text.push_str("[Error: File type not supported]\n");
            }
        }
    }

    fn save_settings(&self) {
        let updated_settings = AppSettings {
            api_url: self.api_url.clone(),
            selected_model: self.selected_model.clone(),
            models: self.model_list.clone(),
            tts_enabled: self.tts_enabled.load(Ordering::Relaxed),
            streaming_enabled: self.streaming_enabled,
            temperature: self.temperature,
            top_p: self.top_p,
            min_p: self.min_p,
            top_k: self.top_k,
            repeat_penalty: self.repeat_penalty,
            max_completion_tokens: self.max_completion_tokens,
            send_stt: self.send_stt,
            // --- Add selected_voice to saved settings ---
            selected_voice: self.selected_voice.clone(),
        };
        save_app_settings(&updated_settings);
    }

    fn clear_history(&mut self) {
        self.chat_bubbles.clear();
        self.code_layout_cache.clear();
        let mut history = self.conversation_history.lock().unwrap();
        history.clear();
        save_memory(&*history);
    }

    fn rebuild_conversation_history(&self) {
        let mut new_history = Vec::new();
        for bubble in &self.chat_bubbles {
            if bubble.persistent {
                let role = match bubble.sender {
                    Sender::User => "user",
                    Sender::Model => "assistant",
                    Sender::System => "system",
                };
                let full_content = if let Some(ref attach) = bubble.attachment_content {
                    if attach.starts_with("data:image") {
                        json!([
                            { "type": "input_text", "text": bubble.content },
                            { "type": "input_image", "image_url": { "url": attach } }
                        ])
                    } else {
                        json!(format!("{}\n{}", bubble.content, attach))
                    }
                } else {
                    json!(bubble.content.clone())
                };
                new_history.push(json!({
                    "role": role,
                    "content": full_content,
                    "id": format!("{:?}", bubble.id)
                }));
            }
        }
        let mut history = self.conversation_history.lock().unwrap();
        *history = new_history;
        save_memory(&*history);
    }

    fn process_input(&mut self) {
        let trimmed = self.input_text.trim();
        if trimmed.is_empty() {
            return;
        }
        if let Some(edit_index) = self.editing_bubble.take() {
            let bubble = &mut self.chat_bubbles[edit_index];
            bubble.content = trimmed.to_string();
            {
                let mut history = self.conversation_history.lock().unwrap();
                for item in history.iter_mut() {
                    if let Some(id_value) = item.get("id").and_then(|v| v.as_str()) {
                        if id_value == format!("{:?}", bubble.id) {
                            item["content"] = json!(trimmed);
                            break;
                        }
                    }
                }
                save_memory(&*history);
            }
            self.input_text.clear();
            return;
        }
        let bubble_id = unique_id("bubble", trimmed);
        self.chat_bubbles.push(ChatBubble {
            sender: Sender::User,
            content: trimmed.to_string(),
            attachment_content: None,
            is_thinking: false,
            is_code: false,
            language: None,
            id: bubble_id,
            timestamp: None,
            persistent: true,
        });
        {
            let mut history = self.conversation_history.lock().unwrap();
            history.push(json!({
                "role": "user",
                "content": trimmed,
                "id": format!("{:?}", bubble_id)
            }));
            save_memory(&*history);
        }
        self.input_text.clear();
        self.scroll_to_bottom = true;
        let (tx, rx) = unbounded_channel();
        self.conversation_channels.push(rx);
        let client = self.client.clone();
        let api_url = self.api_url.clone();
        let model = self.selected_model.clone();
        let history_arc = Arc::clone(&self.conversation_history);
        let tts_enabled = self.tts_enabled.clone();
        let tts_stop_flag = self.tts_stop_flag.clone();
        let temperature = self.temperature;
        let top_p = self.top_p;
        let min_p = self.min_p;
        let top_k = self.top_k;
        let repeat_penalty = self.repeat_penalty;
        let max_completion_tokens = self.max_completion_tokens;
        let experimental_reasoning = self.experimental_reasoning;
        let streaming_enabled = self.streaming_enabled;
        tokio::spawn(async move {
            if streaming_enabled {
                call_model_streaming(
                    &client,
                    &api_url,
                    &model,
                    {
                        let history = history_arc.lock().unwrap();
                        history.clone()
                    },
                    experimental_reasoning,
                    temperature,
                    top_p,
                    min_p,
                    top_k,
                    repeat_penalty,
                    max_completion_tokens,
                    tx.clone(),
                    tts_enabled.clone(),
                    tts_stop_flag.clone(),
                    history_arc.clone(),
                )
                .await;
            } else {
                let mut model_response = call_model(
                    &client,
                    &api_url,
                    &model,
                    {
                        let history = history_arc.lock().unwrap();
                        history.clone()
                    },
                    experimental_reasoning,
                    temperature,
                    top_p,
                    min_p,
                    top_k,
                    repeat_penalty,
                    max_completion_tokens,
                )
                .await;
                if experimental_reasoning {
                    let re = regex::Regex::new(r"(?s)<think>(.*?)</think>").unwrap();
                    if let Some(captures) = re.captures(&model_response.content) {
                        let extracted_reasoning = captures.get(1).unwrap().as_str().trim().to_owned();
                        model_response.reasoning = Some(extracted_reasoning);
                        model_response.content = re.replace(&model_response.content, "").trim().to_string();
                    }
                }
                model_response.content = model_response.content.trim().to_string();
                if let Some(ref mut r) = model_response.reasoning {
                    *r = r.trim().to_string();
                }
                if experimental_reasoning {
                    if let Some(ref reasoning) = model_response.reasoning {
                        send_reasoning(&tx, Sender::Model, reasoning);
                    }
                }
                send_bubbles(&tx, Sender::Model, &model_response.content, false);
                {
                    let mut history = history_arc.lock().unwrap();
                    let bubble_id = unique_id("bubble", &model_response.content);
                    history.push(json!({
                        "role": "assistant",
                        "content": model_response.content.clone(),
                        "id": format!("{:?}", bubble_id)
                    }));
                    save_memory(&*history);
                }
                process_tts(&model_response.content, &tts_enabled, tts_stop_flag);
            }
        });
    }

    fn start_stt_recording(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.stt_active.store(true, Ordering::Relaxed);
        let host = cpal::default_host();
        let device = host.default_input_device().ok_or("No input device available")?;
        let config = device.default_input_config()?.config();
        let target_rate = 16000;
        let channels = config.channels as usize;
        let factor = (config.sample_rate.0 as f32 / target_rate as f32).round() as usize;
        let factor = if factor < 1 { 1 } else { factor };
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: target_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let _writer = hound::WavWriter::create(TEMP_AUDIO_FILE, spec)?;
        let (sample_tx, sample_rx): (SyncSender<Vec<i16>>, _) = sync_channel(256);
        self.audio_sample_tx = Some(sample_tx);
        let spec_clone = spec;
        let audio_thread = thread::spawn(move || {
            let mut writer = hound::WavWriter::create(TEMP_AUDIO_FILE, spec_clone)
                .expect("Failed to create WAV writer");
            while let Ok(samples) = sample_rx.recv() {
                for sample in samples {
                    if let Err(e) = writer.write_sample(sample) {
                        eprintln!("Error writing sample: {:?}", e);
                    }
                }
            }
            if let Err(e) = writer.finalize() {
                eprintln!("Error finalizing WAV writer: {:?}", e);
            }
        });
        self.audio_thread_handle = Some(audio_thread);
        let stt_active = self.stt_active.clone();
        let err_fn = move |err| {
            eprintln!("an error occurred on stream: {}", err);
        };
        let sample_tx_clone = self.audio_sample_tx.as_ref().unwrap().clone();
        let stream = device.build_input_stream(
            &config,
            move |data: &[i16], _: &cpal::InputCallbackInfo| {
                if !stt_active.load(Ordering::Relaxed) {
                    return;
                }
                let mut downsampled = Vec::with_capacity(data.len() / factor);
                for (i, frame) in data.chunks(channels).enumerate() {
                    if i % factor == 0 {
                        if let Some(&s) = frame.get(0) {
                            downsampled.push(s);
                        }
                    }
                }
                let _ = sample_tx_clone.try_send(downsampled);
            },
            err_fn,
            None,
        )?;
        stream.play()?;
        self.stt_stream = Some(stream);
        self.stt_recording = true;
        Ok(())
    }

    fn stop_stt_recording(&mut self) {
        self.stt_active.store(false, Ordering::Relaxed);
        self.stt_stream = None;
        self.audio_sample_tx = None;
        if let Some(handle) = self.audio_thread_handle.take() {
            let _ = handle.join();
        }
        self.stt_recording = false;
    }

    fn stop_stt_recording_and_transcribe_heavy(&mut self) {
        self.stop_stt_recording();
        let tx_clone = self.transcription_tx.clone();
        tokio::spawn(async move {
            let transcription_result =
                tokio::task::spawn_blocking(move || heavy_transcribe(None)).await;
            match transcription_result {
                Ok(Ok(text)) => {
                    tx_clone.send(text).expect("Failed to send transcription");
                }
                Ok(Err(e)) => {
                    let _ = tx_clone.send(format!("Transcription error: {}", e));
                }
                Err(e) => {
                    let _ = tx_clone.send(format!("Transcription join error: {:?}", e));
                }
            }
        });
    }

    fn update_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Clear Chat").clicked() {
                    self.clear_history();
                }
                if ui.button("Stop TTS").clicked() {
                    self.tts_stop_flag.store(true, Ordering::Relaxed);
                }
                if ui.button("Settings").clicked() {
                    self.show_settings = true;
                }
            });
        });
    }

    fn update_input_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("input_panel").show(ctx, |ui| {
            ui.vertical(|ui| {
                let separator_height = 8.0;
                let (drag_rect, drag_resp) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), separator_height),
                    egui::Sense::drag(),
                );
                ui.painter().rect_filled(drag_rect, 0.0, egui::Color32::DARK_GRAY);
                if drag_resp.dragged() {
                    self.input_panel_height = (self.input_panel_height - drag_resp.drag_delta().y)
                        .clamp(20.0, 300.0);
                }
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    let spacing = ui.spacing().item_spacing.x;
                    let send_button_width = 60.0;
                    let text_field_width =
                        (ui.available_width() - send_button_width - spacing).max(0.0);
                    let size = egui::Vec2::new(text_field_width, self.input_panel_height);
                    let (rect, _response) = ui.allocate_exact_size(size, egui::Sense::click());
                    ui.put(rect, |ui: &mut egui::Ui| {
                        egui::ScrollArea::vertical()
                            .id_salt("unique_input_text_scroll")
                            .show(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::multiline(&mut self.input_text)
                                        .lock_focus(true)
                                        .hint_text("Type your message here...")
                                        .desired_rows(1),
                                )
                            })
                            .inner
                    });
                    if ui.button(if self.editing_bubble.is_some() { "Save" } else { "Send" }).clicked()
                        || ui.input(|i| i.key_pressed(egui::Key::Enter) && !i.modifiers.shift)
                    {
                        self.process_input();
                    }
                });
                ui.horizontal(|ui| {
                    let stt_btn_text = if self.stt_recording { "Stop" } else { "STT" };
                    if ui.button(stt_btn_text).clicked()
                        || ui.input(|i| i.key_pressed(egui::Key::Backtick))
                    {
                        if self.stt_recording {
                            self.stop_stt_recording_and_transcribe_heavy();
                        } else {
                            if let Err(e) = self.start_stt_recording() {
                                eprintln!("STT error: {:?}", e);
                            }
                        }
                    }
                    if ui.button("Upload").clicked() {
                        self.handle_file_upload();
                    }
                });
            });
        });
    }

    fn update_settings_window(&mut self, ctx: &egui::Context) {
        if self.show_settings {
            let mut temp_api_url = self.temp_api_url.clone();
            let mut temperature = self.temperature;
            let mut top_p = self.top_p;
            let mut min_p = self.min_p;
            let mut repeat_penalty = self.repeat_penalty;
            let mut top_k = self.top_k;
            let mut max_completion_tokens = self.max_completion_tokens;
            let mut tts_enabled_val = self.tts_enabled.load(Ordering::Relaxed);
            let mut streaming_enabled_val = self.streaming_enabled;
            let mut send_stt_val = self.send_stt;
            let mut selected_model = self.selected_model.clone();
            // --- Add selected_voice variable ---
            let mut selected_voice = self.selected_voice.clone();
            let mut changed = false;
            egui::Window::new("Settings")
                .open(&mut self.show_settings)
                .show(ctx, |ui| {
                    ui.label("API URL:");
                    if ui.text_edit_singleline(&mut temp_api_url).changed() {
                        changed = true;
                    }
                    if ui.button("Save API URL").clicked() {
                        changed = true;
                    }
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Temp:");
                        if ui.add(egui::Slider::new(&mut temperature, 0.0..=2.0)).changed() {
                            changed = true;
                        }
                        if ui.button("Reset").clicked() {
                            temperature = self.default_settings.temperature;
                            changed = true;
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Top_p:");
                        if ui.add(egui::Slider::new(&mut top_p, 0.0..=1.0)).changed() {
                            changed = true;
                        }
                        if ui.button("Reset").clicked() {
                            top_p = self.default_settings.top_p;
                            changed = true;
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Min_p:");
                        if ui.add(egui::Slider::new(&mut min_p, 0.0..=1.0)).changed() {
                            changed = true;
                        }
                        if ui.button("Reset").clicked() {
                            min_p = self.default_settings.min_p;
                            changed = true;
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Repeat Penalty:");
                        if ui.add(egui::Slider::new(&mut repeat_penalty, 0.5..=2.0)).changed() {
                            changed = true;
                        }
                        if ui.button("Reset").clicked() {
                            repeat_penalty = self.default_settings.repeat_penalty;
                            changed = true;
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Top_k:");
                        if ui.add(egui::Slider::new(&mut top_k, 0..=100)).changed() {
                            changed = true;
                        }
                        if ui.button("Reset").clicked() {
                            top_k = self.default_settings.top_k;
                            changed = true;
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Max Tokens:");
                        if ui.add(egui::Slider::new(&mut max_completion_tokens, 10..=10024)).changed() {
                            changed = true;
                        }
                        if ui.button("Reset").clicked() {
                            max_completion_tokens = self.default_settings.max_completion_tokens;
                            changed = true;
                        }
                    });
                    ui.separator();
                    if ui.checkbox(&mut tts_enabled_val, "Enable TTS").changed() {
                        changed = true;
                    }
                    if ui.checkbox(&mut streaming_enabled_val, "Enable Streaming").changed() {
                        changed = true;
                    }
                    if ui.checkbox(&mut send_stt_val, "Send STT").changed() {
                        changed = true;
                    }
                    ui.separator();
                    // --- Add TTS Voice Dropdown ---
                    ui.label("TTS Voice:");
                    egui::ComboBox::from_label("Voice")
                        .selected_text(&selected_voice)
                        .show_ui(ui, |ui| {
                            for voice_filename in AVAILABLE_VOICES {
                                let display_name = voice_filename.trim_end_matches(".bin");
                                if ui.selectable_value(&mut selected_voice, voice_filename.to_string(), display_name).changed() {
                                    changed = true;
                                }
                            }
                        });
                    ui.separator();
                    ui.label("Manage Models:");
                    ui.horizontal(|ui| {
                        egui::ComboBox::from_label("")
                            .selected_text(selected_model.as_str())
                            .show_ui(ui, |ui| {
                                for model in &self.model_list {
                                    if ui.selectable_value(&mut selected_model, model.clone(), model).changed() {
                                        changed = true;
                                    }
                                }
                            });
                        if ui.button("Load Model").clicked() {
                            load_model(&selected_model);
                            self.chat_bubbles.push(ChatBubble {
                                sender: Sender::System,
                                content: format!("Model {} loaded.", selected_model),
                                attachment_content: None,
                                is_thinking: false,
                                is_code: false,
                                language: None,
                                id: unique_id("bubble", &selected_model),
                                timestamp: Some(Instant::now()),
                                persistent: true,
                            });
                        }
                        if ui.button("Unload Model").clicked() {
                            unload_model(&selected_model);
                            self.chat_bubbles.push(ChatBubble {
                                sender: Sender::System,
                                content: format!("Model {} unloaded.", selected_model),
                                attachment_content: None,
                                is_thinking: false,
                                is_code: false,
                                language: None,
                                id: unique_id("bubble", &selected_model),
                                timestamp: Some(Instant::now()),
                                persistent: true,
                            });
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("New Model:");
                        ui.text_edit_singleline(&mut self.new_model_name);
                        if ui.button("+").clicked() {
                            if !self.new_model_name.trim().is_empty() {
                                self.model_list.push(self.new_model_name.clone());
                                selected_model = self.new_model_name.clone();
                                self.new_model_name.clear();
                                changed = true;
                            }
                        }
                        if ui.button("-").clicked() {
                            if let Some(pos) = self.model_list.iter().position(|m| m == &selected_model) {
                                self.model_list.remove(pos);
                                selected_model = if !self.model_list.is_empty() {
                                    self.model_list[0].clone()
                                } else {
                                    "No models".to_owned()
                                };
                                changed = true;
                            }
                        }
                    });
                });
            if changed {
                self.temp_api_url = temp_api_url;
                self.temperature = temperature;
                self.top_p = top_p;
                self.min_p = min_p;
                self.repeat_penalty = repeat_penalty;
                self.top_k = top_k;
                self.max_completion_tokens = max_completion_tokens;
                self.tts_enabled.store(tts_enabled_val, Ordering::Relaxed);
                self.streaming_enabled = streaming_enabled_val;
                self.send_stt = send_stt_val;
                self.selected_model = selected_model;
                // --- Handle selected_voice change ---
                if self.selected_voice != selected_voice {
                    self.selected_voice = selected_voice.clone();
                    // --- Update the global static variable ---
                    *SELECTED_VOICE_PATH.lock().unwrap() = selected_voice.clone();
                    // --- Mark TTS model as unloaded to trigger reload with new voice ---
                    TTS_MODEL_LOADED.store(false, Ordering::Relaxed);
                }
                self.api_url = self.temp_api_url.clone();
                self.save_settings();
                self.chat_bubbles.push(ChatBubble {
                    sender: Sender::System,
                    content: "Settings updated.".to_owned(),
                    attachment_content: None,
                    is_thinking: false,
                    is_code: false,
                    language: None,
                    id: unique_id("bubble", "Settings updated"),
                    timestamp: Some(Instant::now()),
                    persistent: true,
                });
            }
        }
    }

    fn update_chat_area(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_width = ui.available_width();
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.set_width(available_width);
                for (i, bubble) in self.chat_bubbles.clone().into_iter().enumerate() {
                    render_chat_bubble(ui, &bubble, i, self);
                }
                if self.scroll_to_bottom {
                    ui.allocate_space(egui::vec2(0.0, 0.0));
                    ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                    self.scroll_to_bottom = false;
                }
            });
        });
    }

    fn process_conversation_channels(&mut self) {
        self.conversation_channels.retain_mut(|channel| {
            while let Ok(message) = channel.try_recv() {
                match message {
                    BubbleMessage::New(bubble) => {
                        self.chat_bubbles.push(bubble);
                        self.scroll_to_bottom = true;
                    }
                    BubbleMessage::Update { id, content } => {
                        if let Some(existing) = self.chat_bubbles.iter_mut().find(|b| b.id == id) {
                            existing.content = content;
                        }
                        self.scroll_to_bottom = true;
                    }
                    BubbleMessage::Remove(id) => {
                        self.chat_bubbles.retain(|b| b.id != id);
                        self.scroll_to_bottom = true;
                    }
                }
            }
            !channel.is_closed()
        });
    }

    fn update_app(&mut self, ctx: &egui::Context) {
        while let Ok(new_text) = self.transcription_rx.try_recv() {
            if self.send_stt && !new_text.trim().is_empty() {
                self.input_text = new_text;
                self.process_input();
                self.input_text.clear();
            } else {
                self.input_text = new_text;
            }
        }
        self.chat_bubbles.retain(|bubble| {
            if bubble.sender == Sender::System {
                bubble.timestamp.map(|ts| ts.elapsed() < Duration::from_secs(1)).unwrap_or(true)
            } else {
                true
            }
        });
        self.update_top_panel(ctx);
        self.update_input_panel(ctx);
        self.update_settings_window(ctx);
        self.update_chat_area(ctx);
        self.process_conversation_channels();
        self.rebuild_conversation_history();
        if self.last_repaint.elapsed() > Duration::from_millis(16)
            || self.scroll_to_bottom
            || !self.conversation_channels.is_empty()
            || !self.input_text.is_empty()
        {
            ctx.request_repaint();
            self.last_repaint = Instant::now();
        }
    }
}

fn render_chat_bubble(ui: &mut egui::Ui, bubble: &ChatBubble, index: usize, app: &mut ChatApp) {
    let bubble_color = match bubble.sender {
        Sender::User => egui::Color32::from_rgb(53, 51, 54),
        Sender::Model => egui::Color32::from_rgb(18, 107, 166),
        Sender::System => egui::Color32::from_rgb(153, 51, 54),
    };
    egui::Frame {
        fill: if bubble.is_code || bubble.is_thinking {
            egui::Color32::from_rgb(53, 51, 54)
        } else {
            bubble_color
        },
        rounding: egui::Rounding::same(10.0),
        inner_margin: egui::Margin::same(4.0),
        outer_margin: egui::Margin::same(2.0),
        stroke: egui::Stroke::new(1.0, egui::Color32::BLACK),
        shadow: eframe::egui::epaint::Shadow::default(),
    }
    .show(ui, |ui| {
        ui.set_max_width(ui.available_width() * 0.7);
        if bubble.is_code {
            render_collapsible_bubble(ui, "Code:", bubble.id, |ui| {
                let layout = app
                    .code_layout_cache
                    .entry(bubble.id)
                    .or_insert_with(|| highlight_code_job(&bubble.content, bubble.language.as_deref()))
                    .clone();
                ui.label(layout);
                if ui.button("Copy Code").clicked() {
                    ui.output_mut(|o| {
                        o.copied_text = bubble.content.clone();
                    });
                }
            });
        } else if bubble.is_thinking {
            render_collapsible_bubble(ui, "Reasoning:", bubble.id, |ui| {
                render_markdown(ui, &bubble.content);
            });
        } else {
            ui.label(egui::RichText::new(&bubble.content).color(egui::Color32::WHITE));
        }
        ui.horizontal(|ui| {
            if ui.add_sized([40.0, 20.0], egui::Button::new("Edit")).clicked() {
                app.input_text = bubble.content.clone();
                app.editing_bubble = Some(index);
            }
            if ui.add_sized([50.0, 20.0], egui::Button::new("Delete")).clicked() {
                app.chat_bubbles.remove(index);
                app.rebuild_conversation_history();
            }
        });
    });
}

impl eframe::App for ChatApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_app(ctx);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.tts_stop_flag.store(true, Ordering::Relaxed);
        if self.stt_active.compare_exchange(true, false, Ordering::Acquire, Ordering::Relaxed).is_ok() {
            self.stop_stt_recording(); 
        }
        std::thread::sleep(Duration::from_millis(200));
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let native_options = eframe::NativeOptions {
        ..Default::default()
    };
    let chat_app = ChatApp::new();
    let _ = eframe::run_native("AI Chat", native_options, Box::new(|_cc| Ok(Box::new(chat_app))));
}