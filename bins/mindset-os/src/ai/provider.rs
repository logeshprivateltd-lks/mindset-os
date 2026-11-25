//! Hybrid AI Provider Engine for MindSet OS
//!
//! Coordinates Bring-Your-Own-Key (BYOK) Cloud AI providers (Gemini, Groq, OpenAI,
//! OpenRouter, Custom endpoints), free local Ollama streaming, and instant
//! offline fallback via OfflineStudyHelper.

use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock};
use std::time::{Duration, Instant};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

use super::engine::StreamToken;
use super::offline_helper::OfflineStudyHelper;
use crate::storage::db::AiProviderConfigRecord;

static HTTP_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(4))
        .timeout(Duration::from_secs(60))
        .build()
        .unwrap_or_default()
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderStatus {
    pub online: bool,
    pub latency_ms: Option<u64>,
    pub message: String,
}

pub struct HybridAiEngine;

impl HybridAiEngine {
    /// Performs an active health check on the selected AI provider.
    pub async fn health_check(config: &AiProviderConfigRecord) -> Result<ProviderStatus, String> {
        let provider = config.cloud_provider.to_lowercase();
        let api_key = config.cloud_api_key.trim();

        if provider == "ollama" || (api_key.is_empty() && provider != "custom") {
            // Check local Ollama
            let start = Instant::now();
            let url = "http://127.0.0.1:11434/api/tags";
            match HTTP_CLIENT.get(url).send().await {
                Ok(resp) if resp.status().is_success() => {
                    let lat = start.elapsed().as_millis() as u64;
                    Ok(ProviderStatus {
                        online: true,
                        latency_ms: Some(lat),
                        message: format!("Local Ollama active • {}ms latency", lat),
                    })
                }
                _ => Ok(ProviderStatus {
                    online: false,
                    latency_ms: None,
                    message: "Ollama offline. Using built-in Zero-Failure Offline Study Helper."
                        .to_string(),
                }),
            }
        } else if provider == "gemini" {
            if api_key.is_empty() {
                return Err("Please enter a Google Gemini API Key.".to_string());
            }
            let start = Instant::now();
            let url = format!(
                "https://generativelanguage.googleapis.com/v1beta/models?key={}",
                api_key
            );
            match HTTP_CLIENT.get(&url).send().await {
                Ok(resp) if resp.status().is_success() => {
                    let lat = start.elapsed().as_millis() as u64;
                    Ok(ProviderStatus {
                        online: true,
                        latency_ms: Some(lat),
                        message: format!("Connected to Google Gemini • {}ms latency", lat),
                    })
                }
                Ok(resp) => Err(format!("Gemini API error: HTTP {}", resp.status())),
                Err(e) => Err(format!("Network error: {}", e)),
            }
        } else {
            // OpenAI, Groq, OpenRouter, or Custom
            let base_url = if !config.cloud_base_url.trim().is_empty() {
                config.cloud_base_url.trim().trim_end_matches('/')
            } else if provider == "groq" {
                "https://api.groq.com/openai/v1"
            } else if provider == "openrouter" {
                "https://openrouter.ai/api/v1"
            } else {
                "https://api.openai.com/v1"
            };

            let start = Instant::now();
            let url = format!("{}/models", base_url);
            let mut req = HTTP_CLIENT.get(&url);
            if !api_key.is_empty() {
                req = req.header("Authorization", format!("Bearer {}", api_key));
            }

            match req.send().await {
                Ok(resp) if resp.status().is_success() => {
                    let lat = start.elapsed().as_millis() as u64;
                    Ok(ProviderStatus {
                        online: true,
                        latency_ms: Some(lat),
                        message: format!("Connected to {} • {}ms latency", provider, lat),
                    })
                }
                Ok(resp) => Err(format!("{} error: HTTP {}", provider, resp.status())),
                Err(e) => Err(format!("Network error: {}", e)),
            }
        }
    }

    /// Spawns a streaming chat session with automatic fallback chain:
    /// Cloud Provider (if key set) -> Ollama Local AI -> OfflineStudyHelper
    #[allow(clippy::too_many_arguments)]
    pub fn spawn_hybrid_stream(
        config: AiProviderConfigRecord,
        active_model: String,
        system_prompt: String,
        history: Vec<(String, String)>,
        num_ctx: u32,
        student_name: String,
        target_track: String,
        style: String,
    ) -> (UnboundedReceiver<StreamToken>, Arc<AtomicBool>) {
        let (tx, rx) = unbounded_channel();
        let cancel_flag = Arc::new(AtomicBool::new(false));
        let cancel_worker = cancel_flag.clone();

        tokio::spawn(async move {
            let provider = config.cloud_provider.to_lowercase();
            let api_key = config.cloud_api_key.trim().to_string();

            // Last user message for fallback
            let last_user_prompt = history
                .iter()
                .rev()
                .find(|(role, _)| role == "user")
                .map(|(_, content)| content.clone())
                .unwrap_or_else(|| "hello".to_string());

            // 1. Try Cloud AI if API Key is configured
            if !api_key.is_empty() && provider != "ollama" {
                let success = if provider == "gemini" {
                    Self::stream_gemini(
                        &tx,
                        &cancel_worker,
                        &api_key,
                        &config.cloud_model,
                        &system_prompt,
                        &history,
                    )
                    .await
                } else {
                    let base_url = if !config.cloud_base_url.trim().is_empty() {
                        config.cloud_base_url.trim().trim_end_matches('/')
                    } else if provider == "groq" {
                        "https://api.groq.com/openai/v1"
                    } else if provider == "openrouter" {
                        "https://openrouter.ai/api/v1"
                    } else {
                        "https://api.openai.com/v1"
                    };
                    let model_name = if !config.cloud_model.trim().is_empty() {
                        config.cloud_model.trim()
                    } else if provider == "groq" {
                        "llama-3.3-70b-versatile"
                    } else {
                        "gpt-4o-mini"
                    };
                    Self::stream_openai_compatible(
                        &tx,
                        &cancel_worker,
                        base_url,
                        &api_key,
                        model_name,
                        &system_prompt,
                        &history,
                    )
                    .await
                };

                if success {
                    let _ = tx.send(StreamToken::Done);
                    return;
                }
                // If cloud failed, proceed to local Ollama fallback
            }

            // 2. Try Local Ollama Streaming
            let ollama_success = Self::stream_ollama(
                &tx,
                &cancel_worker,
                &active_model,
                &system_prompt,
                &history,
                num_ctx,
            )
            .await;
            if ollama_success {
                let _ = tx.send(StreamToken::Done);
                return;
            }

            // Route Ollama unavailability strictly to OS Notification Bus (NEVER chat stream)
            let _ = tx.send(StreamToken::SystemAlert {
                severity: "error".to_string(),
                title: "Maindy Engine Paused".to_string(),
                body: format!(
                    "Local model '{}' is unavailable. Using built-in offline study notes; click Retry to reconnect Ollama.",
                    if active_model.is_empty() { "gemma2:2b" } else { active_model.as_str() }
                ),
            });

            // 3. Zero-Failure Offline Study Helper Fallback
            let fallback_result = OfflineStudyHelper::respond(
                &last_user_prompt,
                &style,
                &student_name,
                &target_track,
            );

            // Stream fallback response naturally in words
            let words: Vec<&str> = fallback_result.display_text.split_inclusive(' ').collect();
            for word in words {
                if cancel_worker.load(Ordering::SeqCst) {
                    break;
                }
                let _ = tx.send(StreamToken::Chunk(word.to_string()));
                tokio::time::sleep(Duration::from_millis(25)).await;
            }

            let _ = tx.send(StreamToken::Done);
        });

        (rx, cancel_flag)
    }

    async fn stream_ollama(
        tx: &tokio::sync::mpsc::UnboundedSender<StreamToken>,
        cancel: &Arc<AtomicBool>,
        model: &str,
        system_prompt: &str,
        history: &[(String, String)],
        num_ctx: u32,
    ) -> bool {
        let mut messages = Vec::new();
        if !system_prompt.trim().is_empty() {
            messages.push(serde_json::json!({ "role": "system", "content": system_prompt }));
        }
        for (role, content) in history {
            messages.push(serde_json::json!({ "role": role, "content": content }));
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

        let url = "http://127.0.0.1:11434/api/chat";
        let response = match HTTP_CLIENT.post(url).json(&payload).send().await {
            Ok(r) if r.status().is_success() => r,
            _ => return false,
        };

        let mut stream = response.bytes_stream();
        let mut line_buf = String::new();

        while let Some(chunk_res) = stream.next().await {
            if cancel.load(Ordering::SeqCst) {
                return true;
            }
            if let Ok(bytes) = chunk_res {
                line_buf.push_str(&String::from_utf8_lossy(&bytes));
                while let Some(pos) = line_buf.find('\n') {
                    let line = line_buf[..pos].trim().to_string();
                    line_buf.drain(..=pos);
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
                            return true;
                        }
                    }
                }
            }
        }
        true
    }

    async fn stream_openai_compatible(
        tx: &tokio::sync::mpsc::UnboundedSender<StreamToken>,
        cancel: &Arc<AtomicBool>,
        base_url: &str,
        api_key: &str,
        model: &str,
        system_prompt: &str,
        history: &[(String, String)],
    ) -> bool {
        let mut messages = Vec::new();
        if !system_prompt.trim().is_empty() {
            messages.push(serde_json::json!({ "role": "system", "content": system_prompt }));
        }
        for (role, content) in history {
            messages.push(serde_json::json!({ "role": role, "content": content }));
        }

        let payload = serde_json::json!({
            "model": model,
            "messages": messages,
            "stream": true,
            "temperature": 0.7
        });

        let url = format!("{}/chat/completions", base_url.trim_end_matches('/'));
        let response = match HTTP_CLIENT
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&payload)
            .send()
            .await
        {
            Ok(r) if r.status().is_success() => r,
            _ => return false,
        };

        let mut stream = response.bytes_stream();
        let mut line_buf = String::new();

        while let Some(chunk_res) = stream.next().await {
            if cancel.load(Ordering::SeqCst) {
                return true;
            }
            if let Ok(bytes) = chunk_res {
                line_buf.push_str(&String::from_utf8_lossy(&bytes));
                while let Some(pos) = line_buf.find('\n') {
                    let line = line_buf[..pos].trim().to_string();
                    line_buf.drain(..=pos);
                    if line.is_empty() {
                        continue;
                    }
                    let line_content = line.strip_prefix("data: ").unwrap_or(&line);
                    if line_content == "[DONE]" {
                        return true;
                    }
                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(line_content) {
                        if let Some(token) = val
                            .get("choices")
                            .and_then(|c| c.get(0))
                            .and_then(|ch| ch.get("delta"))
                            .and_then(|d| d.get("content"))
                            .and_then(|t| t.as_str())
                        {
                            if !token.is_empty() {
                                let _ = tx.send(StreamToken::Chunk(token.to_string()));
                            }
                        }
                    }
                }
            }
        }
        true
    }

    async fn stream_gemini(
        tx: &tokio::sync::mpsc::UnboundedSender<StreamToken>,
        cancel: &Arc<AtomicBool>,
        api_key: &str,
        model: &str,
        system_prompt: &str,
        history: &[(String, String)],
    ) -> bool {
        let model_name = if model.trim().is_empty() {
            "gemini-2.5-flash"
        } else {
            model.trim()
        };
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:streamGenerateContent?key={}&alt=sse",
            model_name, api_key
        );

        let mut contents = Vec::new();
        for (role, content) in history {
            let gemini_role = if role == "assistant" { "model" } else { "user" };
            contents.push(serde_json::json!({
                "role": gemini_role,
                "parts": [{ "text": content }]
            }));
        }

        let payload = serde_json::json!({
            "systemInstruction": {
                "parts": [{ "text": system_prompt }]
            },
            "contents": contents,
            "generationConfig": {
                "temperature": 0.7
            }
        });

        let response = match HTTP_CLIENT.post(&url).json(&payload).send().await {
            Ok(r) if r.status().is_success() => r,
            _ => return false,
        };

        let mut stream = response.bytes_stream();
        let mut line_buf = String::new();

        while let Some(chunk_res) = stream.next().await {
            if cancel.load(Ordering::SeqCst) {
                return true;
            }
            if let Ok(bytes) = chunk_res {
                line_buf.push_str(&String::from_utf8_lossy(&bytes));
                while let Some(pos) = line_buf.find('\n') {
                    let line = line_buf[..pos].trim().to_string();
                    line_buf.drain(..=pos);
                    if line.is_empty() {
                        continue;
                    }
                    let line_content = line.strip_prefix("data: ").unwrap_or(&line);
                    if let Ok(val) = serde_json::from_str::<serde_json::Value>(line_content) {
                        if let Some(text) = val
                            .get("candidates")
                            .and_then(|c| c.get(0))
                            .and_then(|cand| cand.get("content"))
                            .and_then(|cnt| cnt.get("parts"))
                            .and_then(|p| p.get(0))
                            .and_then(|pt| pt.get("text"))
                            .and_then(|t| t.as_str())
                        {
                            if !text.is_empty() {
                                let _ = tx.send(StreamToken::Chunk(text.to_string()));
                            }
                        }
                    }
                }
            }
        }
        true
    }
}
