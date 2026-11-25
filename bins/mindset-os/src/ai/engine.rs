//! Maindy AI Engine & Local Ollama Streaming Client
//!
//! Features dynamic model discovery, non-blocking NDJSON token streaming,
//! RAM-safe context caps (2048/4096), keep_alive: "2m", and immediate VRAM eviction.

use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock};
use std::time::Duration;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessagePayload {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamToken {
    Chunk(String),
    Done,
    SystemAlert {
        severity: String,
        title: String,
        body: String,
    },
    Error(String),
}

static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(3))
        .build()
        .unwrap_or_default()
});

pub struct OllamaEngine;

impl OllamaEngine {
    /// Queries GET http://{host_port}/api/tags to list installed models
    pub async fn list_models(host_port: &str) -> Result<Vec<String>, String> {
        let url = if host_port.starts_with("http://") || host_port.starts_with("https://") {
            format!("{}/api/tags", host_port.trim_end_matches('/'))
        } else {
            format!("http://{}/api/tags", host_port.trim_end_matches('/'))
        };

        let response = HTTP_CLIENT
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Ollama unreachable: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Ollama returned status {}", response.status()));
        }

        let parsed: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Invalid JSON from Ollama: {}", e))?;

        let mut models = Vec::new();
        if let Some(list) = parsed.get("models").and_then(|m| m.as_array()) {
            for item in list {
                if let Some(name) = item.get("name").and_then(|n| n.as_str()) {
                    models.push(name.to_string());
                }
            }
        }

        // Sort so preferred lightweight models appear first
        models.sort_by_key(|name| {
            let n = name.to_lowercase();
            if n.contains("maindy-ai") {
                0
            } else if n.contains("gemma3:1b") {
                1
            } else if n.contains("gemma2:2b") {
                2
            } else if n.contains("qwen2.5:1.5b") || n.contains("qwen:1.8b") {
                3
            } else if n.contains("llama3.2:1b") {
                4
            } else if n.contains("gemma") {
                5
            } else {
                10
            }
        });

        Ok(models)
    }

    /// Posts keep_alive: 0 to evict active model from RAM/VRAM immediately
    pub async fn unload_model_from_ram(host_port: &str, model: &str) -> Result<(), String> {
        let url = if host_port.starts_with("http://") || host_port.starts_with("https://") {
            format!("{}/api/chat", host_port.trim_end_matches('/'))
        } else {
            format!("http://{}/api/chat", host_port.trim_end_matches('/'))
        };

        let payload = serde_json::json!({
            "model": model,
            "keep_alive": 0,
            "messages": []
        });

        let _ = HTTP_CLIENT
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Dispatches non-blocking NDJSON chat stream via tokio background task
    pub fn spawn_chat_stream(
        host_port: String,
        model: String,
        system_prompt: String,
        history: Vec<(String, String)>,
        num_ctx: u32,
    ) -> (UnboundedReceiver<StreamToken>, Arc<AtomicBool>) {
        let (tx, rx) = unbounded_channel();
        let cancel_flag = Arc::new(AtomicBool::new(false));
        let cancel_worker = cancel_flag.clone();

        tokio::spawn(async move {
            let mut messages = Vec::new();
            if !system_prompt.trim().is_empty() {
                messages.push(ChatMessagePayload {
                    role: "system".into(),
                    content: system_prompt,
                });
            }

            for (role, content) in history {
                messages.push(ChatMessagePayload { role, content });
            }

            let payload = serde_json::json!({
                "model": model,
                "messages": messages,
                "stream": true,
                "keep_alive": "2m",
                "options": {
                    "num_ctx": num_ctx,
                    "temperature": 0.7
                }
            });

            let url = if host_port.starts_with("http://") || host_port.starts_with("https://") {
                format!("{}/api/chat", host_port.trim_end_matches('/'))
            } else {
                format!("http://{}/api/chat", host_port.trim_end_matches('/'))
            };

            let response = match HTTP_CLIENT.post(&url).json(&payload).send().await {
                Ok(resp) => resp,
                Err(e) => {
                    let _ = tx.send(StreamToken::Error(format!(
                        "Ollama unreachable ({}). Click 'Start Ollama' or ensure 'ollama serve' is running.",
                        e
                    )));
                    return;
                }
            };

            if !response.status().is_success() {
                let _ = tx.send(StreamToken::Error(format!(
                    "Ollama error: HTTP {}",
                    response.status()
                )));
                return;
            }

            let mut stream = response.bytes_stream();
            let mut line_buffer = String::new();

            while let Some(chunk_res) = stream.next().await {
                if cancel_worker.load(Ordering::SeqCst) {
                    let _ = tx.send(StreamToken::Done);
                    return;
                }

                match chunk_res {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        line_buffer.push_str(&text);

                        while let Some(pos) = line_buffer.find('\n') {
                            let line = line_buffer[..pos].trim().to_string();
                            line_buffer.drain(..=pos);

                            if line.is_empty() {
                                continue;
                            }

                            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&line) {
                                if let Some(token) = val
                                    .get("message")
                                    .and_then(|m| m.get("content"))
                                    .and_then(|c| c.as_str())
                                {
                                    if !token.is_empty() {
                                        let _ = tx.send(StreamToken::Chunk(token.to_string()));
                                    }
                                }

                                if val.get("done").and_then(|d| d.as_bool()).unwrap_or(false) {
                                    let _ = tx.send(StreamToken::Done);
                                    return;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(StreamToken::Error(format!("Stream error: {}", e)));
                        return;
                    }
                }
            }

            let _ = tx.send(StreamToken::Done);
        });

        (rx, cancel_flag)
    }

    /// Spawn `ollama serve` if installed locally
    pub fn start_local_ollama() -> Result<(), String> {
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("cmd")
                .args(["/C", "start", "/B", "ollama", "serve"])
                .spawn()
                .map_err(|e| format!("Failed to launch ollama serve: {}", e))?;
        }
        #[cfg(not(target_os = "windows"))]
        {
            std::process::Command::new("nohup")
                .args(["ollama", "serve"])
                .spawn()
                .map_err(|e| format!("Failed to launch ollama serve: {}", e))?;
        }
        Ok(())
    }
}
