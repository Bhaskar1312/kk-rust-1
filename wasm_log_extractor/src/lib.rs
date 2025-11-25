use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;


#[derive(Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub severity: String, // level
    pub message: String,
}

#[wasm_bindgen]
pub fn extract_logs(log: &str) -> JsValue {
    let parts: Vec<&str> = log.splitn(3, ' ').collect();
    if parts.len() < 3 {
        return JsValue::from_str("Invalid log format");
    }
    let log_entry = LogEntry {
        timestamp: parts[0].to_string(),
        severity: parts[1].to_string(),
        message: parts[2].to_string(),
    };
    to_value(&log_entry).unwrap() // serialize logEntry struct to JsValue
}
// cargo new wasm_log_extractor --lib 
// cargo add serde serde-wasm-bindgen wasm-bindgen
// cargo install wasm-pack
// RUST_LOG=debug wasm-pack build --target web

// "2025-11-22 WARNING This is a warning message"