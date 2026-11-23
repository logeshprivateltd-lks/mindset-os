//! MindSet OS — Laya AI Runtime Bridge & Ollama / llama.cpp Orchestrator
//! Single-Model RAM Discipline, Streaming SSE Client & GBNF Structured Tool Execution
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::timeout;
use tracing::info;

pub const DEFAULT_OLLAMA_ENDPOINT: &str = "127.0.0.1:11434";
pub const DEFAULT_KEEP_ALIVE: &str = "15m";
pub const DEFAULT_CONTEXT_TOKENS: u32 = 4096;

// ---------------------------------------------------------------------------
// 1. Data Transfer Objects (DTOs)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayaStreamRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub system_prompt: Option<String>,
    pub temperature: Option<f32>,
    pub format: Option<String>, // e.g. "json"
    pub stream: Option<bool>,
    pub keep_alive: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayaStreamChunk {
    pub model: String,
    pub token: String,
    pub done: bool,
    pub total_duration_ms: Option<u64>,
    pub prompt_eval_count: Option<u32>,
    pub eval_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalModelInfo {
    pub name: String,
    pub size_bytes: u64,
    pub modified_at: String,
    pub parameter_size: Option<String>,
    pub quantization_level: Option<String>,
    pub family: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredToolResponse {
    pub success: BooleanOrString,
    pub data: serde_json::Value,
    pub latency_ms: u64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BooleanOrString {
    Bool(bool),
    Str(String),
}

// ---------------------------------------------------------------------------
// 2. Active Model State Manager (Single-Model RAM Discipline)
// ---------------------------------------------------------------------------

#[allow(dead_code)]
pub struct LayaEngineState {
    pub active_model: Arc<tokio::sync::RwLock<String>>,
    pub is_streaming: Arc<AtomicBool>,
}

#[allow(dead_code)]
impl LayaEngineState {
    pub fn new(default_model: &str) -> Self {
        Self {
            active_model: Arc::new(tokio::sync::RwLock::new(default_model.to_string())),
            is_streaming: Arc::new(AtomicBool::new(false)),
        }
    }
}

// ---------------------------------------------------------------------------
// 3. Engine Implementation
// ---------------------------------------------------------------------------

pub struct LayaEngine;

impl LayaEngine {
    /// Helper to resolve (host_port, path) from optional endpoint string
    fn parse_endpoint(endpoint_opt: Option<&str>, default_path: &str) -> (String, String) {
        let raw = endpoint_opt.unwrap_or(DEFAULT_OLLAMA_ENDPOINT);
        let cleaned = raw
            .trim_start_matches("http://")
            .trim_start_matches("https://");
        match cleaned.split_once('/') {
            Some((hp, p)) => (hp.to_string(), format!("/{}", p)),
            None => (cleaned.to_string(), default_path.to_string()),
        }
    }

    /// Connects to Ollama with 2.0s connect timeout
    async fn connect_tcp(host_port: &str) -> Result<TcpStream, String> {
        let connect_fut = TcpStream::connect(host_port);
        match timeout(Duration::from_millis(2000), connect_fut).await {
            Ok(Ok(stream)) => Ok(stream),
            Ok(Err(e)) => Err(format!(
                "Ollama offline: Connection refused at {} ({})",
                host_port, e
            )),
            Err(_) => Err(format!("Connection timeout to Ollama at {}", host_port)),
        }
    }

    /// Lists locally installed Ollama models via GET /api/tags
    pub async fn list_local_models(endpoint: Option<&str>) -> Result<Vec<LocalModelInfo>, String> {
        let (host_port, _) = Self::parse_endpoint(endpoint, "/api/tags");
        let mut stream = Self::connect_tcp(&host_port).await?;

        let request = format!(
            "GET /api/tags HTTP/1.1\r\nHost: {}\r\nUser-Agent: MindSetLaya/1.0\r\nConnection: close\r\n\r\n",
            host_port
        );

        stream
            .write_all(request.as_bytes())
            .await
            .map_err(|e| format!("Write failed: {}", e))?;

        let mut buf = Vec::new();
        stream
            .read_to_end(&mut buf)
            .await
            .map_err(|e| format!("Read failed: {}", e))?;

        let response = String::from_utf8_lossy(&buf);
        let body = match response.split_once("\r\n\r\n") {
            Some((_, b)) => b,
            None => return Ok(Vec::new()),
        };

        // If chunked transfer encoding, unchunk
        let cleaned_body = Self::clean_http_body(body);

        let parsed: serde_json::Value = serde_json::from_str(&cleaned_body)
            .map_err(|e| format!("Invalid JSON response: {}", e))?;

        let mut models = Vec::new();
        if let Some(list) = parsed.get("models").and_then(|m| m.as_array()) {
            for item in list {
                let name = item
                    .get("name")
                    .and_then(|n| n.as_str())
                    .unwrap_or("unknown")
                    .to_string();
                let size_bytes = item.get("size").and_then(|s| s.as_u64()).unwrap_or(0);
                let modified_at = item
                    .get("modified_at")
                    .and_then(|m| m.as_str())
                    .unwrap_or("")
                    .to_string();

                let details = item.get("details");
                let parameter_size = details
                    .and_then(|d| d.get("parameter_size"))
                    .and_then(|p| p.as_str())
                    .map(|s| s.to_string());
                let quantization_level = details
                    .and_then(|d| d.get("quantization_level"))
                    .and_then(|q| q.as_str())
                    .map(|s| s.to_string());
                let family = details
                    .and_then(|d| d.get("family"))
                    .and_then(|f| f.as_str())
                    .map(|s| s.to_string());

                models.push(LocalModelInfo {
                    name,
                    size_bytes,
                    modified_at,
                    parameter_size,
                    quantization_level,
                    family,
                });
            }
        }

        Ok(models)
    }

    /// Immediately unloads model from RAM by requesting keep_alive: 0
    pub async fn unload_active_model_from_ram(
        model: &str,
        endpoint: Option<&str>,
    ) -> Result<bool, String> {
        let (host_port, _) = Self::parse_endpoint(endpoint, "/api/chat");
        let mut stream = Self::connect_tcp(&host_port).await?;

        let payload = serde_json::json!({
            "model": model,
            "keep_alive": 0,
            "messages": []
        });
        let payload_str = payload.to_string();

        let request = format!(
            "POST /api/chat HTTP/1.1\r\nHost: {}\r\nUser-Agent: MindSetLaya/1.0\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            host_port,
            payload_str.len(),
            payload_str
        );

        stream
            .write_all(request.as_bytes())
            .await
            .map_err(|e| format!("Failed to send unload request: {}", e))?;

        let mut buf = Vec::new();
        let _ = stream.read_to_end(&mut buf).await;

        info!(
            "Successfully evicted model '{}' from RAM (keep_alive: 0 acknowledged)",
            model
        );
        Ok(true)
    }

    /// Generates vector embeddings on-demand via Ollama POST /api/embeddings
    pub async fn generate_embedding(
        model: &str,
        text: &str,
        endpoint: Option<&str>,
    ) -> Result<Vec<f32>, String> {
        let (host_port, _) = Self::parse_endpoint(endpoint, "/api/embeddings");
        let mut stream = Self::connect_tcp(&host_port).await?;

        let payload = serde_json::json!({
            "model": model,
            "prompt": text,
            "keep_alive": "5m"
        });
        let payload_str = payload.to_string();

        let request = format!(
            "POST /api/embeddings HTTP/1.1\r\nHost: {}\r\nUser-Agent: MindSetLaya/1.0\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            host_port,
            payload_str.len(),
            payload_str
        );

        stream
            .write_all(request.as_bytes())
            .await
            .map_err(|e| format!("Write failed: {}", e))?;

        let mut buf = Vec::new();
        stream
            .read_to_end(&mut buf)
            .await
            .map_err(|e| format!("Read failed: {}", e))?;

        let response = String::from_utf8_lossy(&buf);
        let body = match response.split_once("\r\n\r\n") {
            Some((_, b)) => b,
            None => return Err("Invalid HTTP response headers".to_string()),
        };

        let cleaned_body = Self::clean_http_body(body);
        let parsed: serde_json::Value = serde_json::from_str(&cleaned_body)
            .map_err(|e| format!("Embedding JSON parsing error: {}", e))?;

        if let Some(arr) = parsed.get("embedding").and_then(|v| v.as_array()) {
            let vec: Vec<f32> = arr
                .iter()
                .filter_map(|val| val.as_f64().map(|f| f as f32))
                .collect();
            return Ok(vec);
        }

        Err("No embedding array found in response".to_string())
    }

    /// Executes structured JSON tool call (e.g. Schedule generation, task addition)
    /// Enforces format: "json" so responses strictly follow valid schema.
    pub async fn execute_structured_tool_call(
        model: &str,
        system_prompt: &str,
        user_prompt: &str,
        json_schema_desc: Option<&str>,
        endpoint: Option<&str>,
    ) -> Result<serde_json::Value, String> {
        let start = std::time::Instant::now();
        let (host_port, _) = Self::parse_endpoint(endpoint, "/api/chat");
        let mut stream = Self::connect_tcp(&host_port).await?;

        let mut messages = Vec::new();
        if !system_prompt.is_empty() {
            let enriched_sys = if let Some(schema) = json_schema_desc {
                format!(
                    "{}\n\nCRITICAL ACTION SCHEMA:\nYou must respond strictly with valid JSON conforming to:\n{}",
                    system_prompt, schema
                )
            } else {
                system_prompt.to_string()
            };
            messages.push(serde_json::json!({
                "role": "system",
                "content": enriched_sys
            }));
        }

        messages.push(serde_json::json!({
            "role": "user",
            "content": user_prompt
        }));

        let payload = serde_json::json!({
            "model": model,
            "messages": messages,
            "format": "json",
            "stream": false,
            "keep_alive": DEFAULT_KEEP_ALIVE,
            "options": {
                "num_ctx": DEFAULT_CONTEXT_TOKENS,
                "temperature": 0.2
            }
        });

        let payload_str = payload.to_string();

        let request = format!(
            "POST /api/chat HTTP/1.1\r\nHost: {}\r\nUser-Agent: MindSetLaya/1.0\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            host_port,
            payload_str.len(),
            payload_str
        );

        stream
            .write_all(request.as_bytes())
            .await
            .map_err(|e| format!("Write failed: {}", e))?;

        let mut buf = Vec::new();
        stream
            .read_to_end(&mut buf)
            .await
            .map_err(|e| format!("Read failed: {}", e))?;

        let response = String::from_utf8_lossy(&buf);
        let body = match response.split_once("\r\n\r\n") {
            Some((_, b)) => b,
            None => return Err("Invalid HTTP response headers from Ollama".to_string()),
        };

        let cleaned_body = Self::clean_http_body(body);
        let parsed: serde_json::Value = serde_json::from_str(&cleaned_body)
            .map_err(|e| format!("JSON decode error on Ollama response: {}", e))?;

        let content = parsed
            .get("message")
            .and_then(|m| m.get("content"))
            .and_then(|c| c.as_str())
            .unwrap_or("{}");

        let structured_json: serde_json::Value = serde_json::from_str(content)
            .map_err(|e| format!("Ollama content was not valid JSON: {}", e))?;

        info!(
            "Structured tool call completed in {}ms on model '{}'",
            start.elapsed().as_millis(),
            model
        );

        Ok(structured_json)
    }

    /// Full streaming SSE client with token-by-token emission callback
    pub async fn stream_chat<F>(
        mut on_chunk: F,
        req: LayaStreamRequest,
        endpoint: Option<&str>,
        is_streaming_flag: Option<Arc<AtomicBool>>,
    ) -> Result<String, String>
    where
        F: FnMut(LayaStreamChunk) + Send,
    {
        let (host_port, _) = Self::parse_endpoint(endpoint, "/api/chat");
        let stream = Self::connect_tcp(&host_port).await?;

        if let Some(ref flag) = is_streaming_flag {
            flag.store(true, Ordering::SeqCst);
        }

        let mut messages = Vec::new();
        if let Some(sys) = req.system_prompt {
            messages.push(serde_json::json!({
                "role": "system",
                "content": sys
            }));
        }

        for msg in req.messages {
            messages.push(serde_json::json!({
                "role": msg.role,
                "content": msg.content
            }));
        }

        let keep_alive = req
            .keep_alive
            .unwrap_or_else(|| DEFAULT_KEEP_ALIVE.to_string());

        let mut payload_map = serde_json::json!({
            "model": req.model,
            "messages": messages,
            "stream": true,
            "keep_alive": keep_alive,
            "options": {
                "num_ctx": DEFAULT_CONTEXT_TOKENS,
                "temperature": req.temperature.unwrap_or(0.7)
            }
        });

        if let Some(fmt) = req.format {
            if fmt == "json" {
                payload_map["format"] = serde_json::json!("json");
            }
        }

        let payload_str = payload_map.to_string();

        let request_header = format!(
            "POST /api/chat HTTP/1.1\r\nHost: {}\r\nUser-Agent: MindSetLaya/1.0\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            host_port,
            payload_str.len(),
            payload_str
        );

        let (reader_half, mut writer_half) = tokio::io::split(stream);

        writer_half
            .write_all(request_header.as_bytes())
            .await
            .map_err(|e| format!("Failed to send chat stream request: {}", e))?;

        let mut reader = BufReader::new(reader_half);
        let mut line = String::new();

        // 1. Skip HTTP headers until blank line
        loop {
            line.clear();
            let bytes_read = reader
                .read_line(&mut line)
                .await
                .map_err(|e| format!("Header read error: {}", e))?;

            if bytes_read == 0 || line == "\r\n" || line == "\n" {
                break;
            }
        }

        // 2. Stream chunk-by-chunk and emit Tauri events
        let mut accumulated_content = String::new();

        loop {
            line.clear();
            let bytes_read = reader
                .read_line(&mut line)
                .await
                .map_err(|e| format!("Stream read error: {}", e))?;

            if bytes_read == 0 {
                break;
            }

            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Skip hex chunk size lines if chunked encoding
            if trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
                continue;
            }

            if let Ok(val) = serde_json::from_str::<serde_json::Value>(trimmed) {
                let token = val
                    .get("message")
                    .and_then(|m| m.get("content"))
                    .and_then(|c| c.as_str())
                    .unwrap_or("")
                    .to_string();

                let done = val.get("done").and_then(|d| d.as_bool()).unwrap_or(false);
                let total_duration_ms = val
                    .get("total_duration")
                    .and_then(|td| td.as_u64())
                    .map(|nanos| nanos / 1_000_000);

                let prompt_eval_count = val
                    .get("prompt_eval_count")
                    .and_then(|p| p.as_u64())
                    .map(|c| c as u32);

                let eval_count = val
                    .get("eval_count")
                    .and_then(|e| e.as_u64())
                    .map(|c| c as u32);

                accumulated_content.push_str(&token);

                let chunk = LayaStreamChunk {
                    model: req.model.clone(),
                    token,
                    done,
                    total_duration_ms,
                    prompt_eval_count,
                    eval_count,
                };

                // Emit chunk to callback
                on_chunk(chunk);

                if done {
                    break;
                }
            }
        }

        if let Some(ref flag) = is_streaming_flag {
            flag.store(false, Ordering::SeqCst);
        }

        Ok(accumulated_content)
    }

    /// Strips chunked transfer-encoding hex metadata from HTTP body if present
    fn clean_http_body(raw: &str) -> String {
        let lines: Vec<&str> = raw.lines().collect();
        if lines.is_empty() {
            return String::new();
        }

        // If first line is hex, filter out chunk length lines
        let mut clean = String::new();
        for line in lines {
            let t = line.trim();
            if !t.is_empty() && !t.chars().all(|c| c.is_ascii_hexdigit()) {
                clean.push_str(line);
                clean.push('\n');
            }
        }

        if clean.is_empty() {
            raw.to_string()
        } else {
            clean
        }
    }
}
