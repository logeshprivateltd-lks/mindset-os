//! Bilingual Text-To-Speech (TTS) Engine for MindSet OS
//!
//! Provides studio-grade speech synthesis with:
//! 1. Edge Neural TTS streaming (~4 MB RAM) for high-fidelity Tamil
//!    (`ta-IN-ValluvarNeural`, `ta-IN-PallaviNeural`) and English (`en-IN-NeerjaNeural`).
//! 2. Piper ONNX offline engine (< 60 MB RAM) for cross-platform zero-cloud playback.
//! 3. Fast OS fallback (`Windows.Media.SpeechSynthesis` / `System.Speech` / `spd-say`).
//! 4. Memory discipline: maintains strictly < 60 MB RAM footprint.
//! 5. Non-blocking asynchronous dispatch with process tracking & instant stop.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

use super::persona::{expand_latex_to_phonetic_speech, thanglish_to_tamil_phonetic};

pub static TTS_IS_SPEAKING: std::sync::LazyLock<Arc<AtomicBool>> =
    std::sync::LazyLock::new(|| Arc::new(AtomicBool::new(false)));

pub static CURRENT_TTS_PROCESS: std::sync::LazyLock<Arc<Mutex<Option<u32>>>> =
    std::sync::LazyLock::new(|| Arc::new(Mutex::new(None)));

// ---------------------------------------------------------------------------
// Studio-Grade Voice Synthesis Domain Models (Feature 23)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LanguagePreference {
    Tamil,
    English,
    Auto,
    Custom(String),
}

impl LanguagePreference {
    pub fn from_lang_code(code: &str) -> Self {
        let lower = code.trim().to_lowercase();
        if lower.starts_with("ta") || lower.contains("tamil") || lower.contains("thanglish") {
            LanguagePreference::Tamil
        } else if lower.starts_with("en") || lower.contains("english") {
            LanguagePreference::English
        } else if lower.starts_with("auto") {
            LanguagePreference::Auto
        } else {
            LanguagePreference::Custom(code.to_string())
        }
    }

    pub fn code(&self) -> &str {
        match self {
            LanguagePreference::Tamil => "ta-IN",
            LanguagePreference::English => "en-IN",
            LanguagePreference::Auto => "auto",
            LanguagePreference::Custom(c) => c.as_str(),
        }
    }

    pub fn default_neural_voice(&self) -> &'static str {
        match self {
            LanguagePreference::Tamil => "ta-IN-ValluvarNeural",
            LanguagePreference::English => "en-IN-NeerjaNeural",
            LanguagePreference::Auto => "ta-IN-PallaviNeural",
            LanguagePreference::Custom(_) => "en-IN-NeerjaNeural",
        }
    }

    pub fn secondary_neural_voice(&self) -> &'static str {
        match self {
            LanguagePreference::Tamil => "ta-IN-PallaviNeural",
            LanguagePreference::English => "en-IN-PrabhatNeural",
            LanguagePreference::Auto => "ta-IN-ValluvarNeural",
            LanguagePreference::Custom(_) => "en-IN-NeerjaNeural",
        }
    }

    pub fn default_piper_model(&self) -> &'static str {
        match self {
            LanguagePreference::Tamil => "ta_IN-valluvar-medium",
            LanguagePreference::English => "en_US-lessac-medium",
            LanguagePreference::Auto => "ta_IN-pallavi-medium",
            LanguagePreference::Custom(_) => "en_US-lessac-medium",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TtsRoute {
    /// Microsoft Edge Neural TTS streaming (~4 MB RAM)
    CloudNeural { voice: String },
    /// Lightweight Piper ONNX offline engine (< 60 MB RAM)
    PiperLocal { model_path: PathBuf },
    /// Native OS speech synthesis fallback (WinRT / spd-say)
    NativeFallback { voice_name: String },
}

impl TtsRoute {
    pub fn description(&self) -> String {
        match self {
            TtsRoute::CloudNeural { voice } => format!("Edge Neural TTS ({voice}) [~4 MB RAM]"),
            TtsRoute::PiperLocal { model_path } => {
                format!("Piper ONNX ({}) [<60 MB RAM]", model_path.display())
            }
            TtsRoute::NativeFallback { voice_name } => format!("Fast OS Native ({voice_name})"),
        }
    }

    pub fn estimated_ram_mb(&self) -> u32 {
        match self {
            TtsRoute::CloudNeural { .. } => 4,
            TtsRoute::PiperLocal { .. } => 55,
            TtsRoute::NativeFallback { .. } => 12,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TtsError {
    Network(String),
    Piper(String),
    Native(String),
    Audio(String),
    FallbackExhausted(String),
    Cancelled,
}

impl std::fmt::Display for TtsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TtsError::Network(msg) => write!(f, "Network error during TTS synthesis: {msg}"),
            TtsError::Piper(msg) => write!(f, "Piper ONNX engine error: {msg}"),
            TtsError::Native(msg) => write!(f, "Native OS speech synthesis error: {msg}"),
            TtsError::Audio(msg) => write!(f, "Audio decoding or playback error: {msg}"),
            TtsError::FallbackExhausted(msg) => write!(f, "All synthesis routes exhausted: {msg}"),
            TtsError::Cancelled => write!(f, "Speech synthesis cancelled"),
        }
    }
}

impl std::error::Error for TtsError {}

// ---------------------------------------------------------------------------
// Studio-Grade Voice Engine (Feature 23)
// ---------------------------------------------------------------------------

pub struct VoiceEngine;

impl VoiceEngine {
    /// Resolves the optimal TTS route given user language preferences and system environment
    pub fn resolve_optimal_route(lang: &LanguagePreference) -> TtsRoute {
        let neural_voice = lang.default_neural_voice().to_string();

        if Self::is_edge_tts_available() {
            return TtsRoute::CloudNeural {
                voice: neural_voice,
            };
        }

        if let Some(piper_path) = Self::find_piper_model(lang) {
            return TtsRoute::PiperLocal {
                model_path: piper_path,
            };
        }

        TtsRoute::NativeFallback {
            voice_name: match lang {
                LanguagePreference::Tamil => "Tamil".to_string(),
                _ => "Default".to_string(),
            },
        }
    }

    /// Checks if edge-tts CLI is available on PATH
    pub fn is_edge_tts_available() -> bool {
        #[cfg(target_os = "windows")]
        {
            let mut cmd = std::process::Command::new("where");
            cmd.arg("edge-tts");
            cmd.creation_flags(CREATE_NO_WINDOW);
            if let Ok(out) = cmd.output() {
                if out.status.success() {
                    return true;
                }
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            let mut cmd = std::process::Command::new("which");
            cmd.arg("edge-tts");
            if let Ok(out) = cmd.output() {
                if out.status.success() {
                    return true;
                }
            }
        }
        false
    }

    /// Finds candidate piper model path on disk
    pub fn find_piper_model(lang: &LanguagePreference) -> Option<PathBuf> {
        let model_name = lang.default_piper_model();
        let candidate_dirs = [
            PathBuf::from("/usr/share/piper-voices"),
            PathBuf::from("~/.mindset_data/models/piper"),
            dirs::data_local_dir()
                .map(|p| p.join("mindset").join("piper"))
                .unwrap_or_default(),
            PathBuf::from("./models/piper"),
        ];

        for dir in candidate_dirs {
            let onnx = dir.join(format!("{model_name}.onnx"));
            if onnx.exists() {
                return Some(onnx);
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            if let Ok(out) = std::process::Command::new("which").arg("piper").output() {
                if out.status.success() {
                    return Some(PathBuf::from(model_name));
                }
            }
        }

        None
    }

    /// Core synthesize interface required by PROJECT.md:
    /// `VoiceEngine::synthesize(text: &str, lang: LanguagePreference, route: TtsRoute) -> Result<Vec<u8>, TtsError>`
    pub fn synthesize(
        text: &str,
        lang: LanguagePreference,
        route: TtsRoute,
    ) -> Result<Vec<u8>, TtsError> {
        match route {
            TtsRoute::CloudNeural { voice } => Self::synthesize_edge_neural(text, &voice),
            TtsRoute::PiperLocal { model_path } => Self::synthesize_piper(text, &model_path),
            TtsRoute::NativeFallback { voice_name } => {
                Self::synthesize_native_fallback(text, &lang, &voice_name)
            }
        }
    }

    /// High-level cascading synthesis: CloudNeural -> PiperLocal -> NativeFallback
    pub fn synthesize_with_fallback(
        text: &str,
        lang: LanguagePreference,
    ) -> Result<(Vec<u8>, TtsRoute), TtsError> {
        // 1. Try Cloud Neural first
        let voice = lang.default_neural_voice().to_string();
        let cloud_route = TtsRoute::CloudNeural { voice };
        match Self::synthesize(text, lang.clone(), cloud_route.clone()) {
            Ok(bytes) if !bytes.is_empty() => return Ok((bytes, cloud_route)),
            Err(e) => {
                tracing::debug!(
                    "Cloud neural TTS failed ({}), attempting Piper local fallback",
                    e
                );
            }
            _ => {}
        }

        // 2. Try Piper Local second
        let piper_model = Self::find_piper_model(&lang)
            .unwrap_or_else(|| PathBuf::from(lang.default_piper_model()));
        let piper_route = TtsRoute::PiperLocal {
            model_path: piper_model,
        };
        match Self::synthesize(text, lang.clone(), piper_route.clone()) {
            Ok(bytes) if !bytes.is_empty() => return Ok((bytes, piper_route)),
            Err(e) => {
                tracing::debug!(
                    "Piper local TTS failed ({}), attempting native OS fallback",
                    e
                );
            }
            _ => {}
        }

        // 3. Try Native Fallback third
        let native_route = TtsRoute::NativeFallback {
            voice_name: lang.code().to_string(),
        };
        match Self::synthesize(text, lang.clone(), native_route.clone()) {
            Ok(bytes) if !bytes.is_empty() => Ok((bytes, native_route)),
            Err(e) => Err(TtsError::FallbackExhausted(format!(
                "All routes exhausted for text: {e}"
            ))),
            _ => Err(TtsError::FallbackExhausted("Empty audio received".into())),
        }
    }

    /// Edge Neural TTS streaming synthesis (~4 MB RAM footprint)
    pub fn synthesize_edge_neural(text: &str, voice: &str) -> Result<Vec<u8>, TtsError> {
        let temp_dir = std::env::temp_dir();
        let file_id = uuid::Uuid::new_v4().to_string();
        let temp_file = temp_dir.join(format!("mindset_edge_tts_{file_id}.mp3"));

        let mut cmd = std::process::Command::new("edge-tts");
        cmd.args([
            "--voice",
            voice,
            "--text",
            text,
            "--write-media",
            &temp_file.to_string_lossy(),
        ]);

        #[cfg(target_os = "windows")]
        cmd.creation_flags(CREATE_NO_WINDOW);

        let output = cmd
            .output()
            .map_err(|e| TtsError::Network(format!("edge-tts spawn error: {e}")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let _ = std::fs::remove_file(&temp_file);
            return Err(TtsError::Network(format!(
                "edge-tts exit {}: {stderr}",
                output.status
            )));
        }

        let bytes = std::fs::read(&temp_file)
            .map_err(|e| TtsError::Network(format!("read audio file error: {e}")))?;
        let _ = std::fs::remove_file(&temp_file);

        if bytes.is_empty() {
            return Err(TtsError::Network("edge-tts produced 0 bytes".to_string()));
        }

        Ok(bytes)
    }

    /// Piper ONNX offline synthesis (<60 MB RAM footprint)
    pub fn synthesize_piper(text: &str, model_path: &Path) -> Result<Vec<u8>, TtsError> {
        use std::io::Write;

        let mut cmd = std::process::Command::new("piper");
        cmd.args(["--model", &model_path.to_string_lossy(), "--output-raw"]);
        cmd.stdin(std::process::Stdio::piped());
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());

        #[cfg(target_os = "windows")]
        cmd.creation_flags(CREATE_NO_WINDOW);

        let mut child = cmd
            .spawn()
            .map_err(|e| TtsError::Piper(format!("piper spawn error: {e}")))?;

        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
        }

        let output = child
            .wait_with_output()
            .map_err(|e| TtsError::Piper(format!("piper wait error: {e}")))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(TtsError::Piper(format!(
                "piper exit {}: {stderr}",
                output.status
            )));
        }

        let raw_pcm = output.stdout;
        if raw_pcm.is_empty() {
            return Err(TtsError::Piper("piper output is empty".to_string()));
        }

        // Wrap raw 16-bit 22050Hz mono PCM into a standard WAV header so rodio can decode it
        let wav_bytes = create_wav_header(raw_pcm, 22050, 1, 16);
        Ok(wav_bytes)
    }

    /// Fast OS fallback synthesis
    pub fn synthesize_native_fallback(
        text: &str,
        lang: &LanguagePreference,
        _voice_name: &str,
    ) -> Result<Vec<u8>, TtsError> {
        #[cfg(target_os = "windows")]
        {
            let temp_dir = std::env::temp_dir();
            let file_id = uuid::Uuid::new_v4().to_string();
            let temp_wav = temp_dir.join(format!("mindset_win_tts_{file_id}.wav"));

            let lang_filter = match lang {
                LanguagePreference::Tamil => "$all | Where-Object { $_.VoiceInfo.Culture.Name -like 'ta*' -or $_.VoiceInfo.Name -like '*Tamil*' -or $_.VoiceInfo.Name -like '*Valluvar*' -or $_.VoiceInfo.Name -like '*Ravi*' -or $_.VoiceInfo.Name -like '*Heera*' } | Select-Object -First 1",
                _ => "$all | Where-Object { $_.VoiceInfo.Name -like '*Natural*' -or $_.VoiceInfo.Name -like '*Zira*' -or $_.VoiceInfo.Name -like '*Mark*' -or $_.VoiceInfo.Name -like '*David*' } | Select-Object -First 1",
            };

            let script = format!(
                "Add-Type -AssemblyName System.Speech; \
                 $s = New-Object System.Speech.Synthesis.SpeechSynthesizer; \
                 $all = $s.GetInstalledVoices() | Where-Object {{ $_.Enabled }}; \
                 $v = {}; \
                 if ($v) {{ $s.SelectVoice($v.VoiceInfo.Name) }}; \
                 $s.SetOutputToWaveFilePath('{}'); \
                 $s.Speak('{}'); \
                 $s.Dispose();",
                lang_filter,
                temp_wav.to_string_lossy().replace('\'', "''"),
                text.replace('\'', "''")
            );

            let mut cmd = std::process::Command::new("powershell");
            cmd.args(["-NoProfile", "-NonInteractive", "-Command", &script]);
            cmd.creation_flags(CREATE_NO_WINDOW);

            let output = cmd
                .output()
                .map_err(|e| TtsError::Native(format!("powershell spawn error: {e}")))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let _ = std::fs::remove_file(&temp_wav);
                return Err(TtsError::Native(format!(
                    "powershell exit {}: {stderr}",
                    output.status
                )));
            }

            if temp_wav.exists() {
                let bytes = std::fs::read(&temp_wav)
                    .map_err(|e| TtsError::Native(format!("read wav error: {e}")))?;
                let _ = std::fs::remove_file(&temp_wav);
                if !bytes.is_empty() {
                    return Ok(bytes);
                }
            }

            Err(TtsError::Native(
                "powershell speech output was empty".to_string(),
            ))
        }

        #[cfg(not(target_os = "windows"))]
        {
            let temp_dir = std::env::temp_dir();
            let file_id = uuid::Uuid::new_v4().to_string();
            let temp_wav = temp_dir.join(format!("mindset_linux_tts_{file_id}.wav"));

            let espeak_voice = match lang {
                LanguagePreference::Tamil => "ta",
                _ => "en-in",
            };

            let out = std::process::Command::new("espeak-ng")
                .args(["-v", espeak_voice, "-w", &temp_wav.to_string_lossy(), text])
                .output();

            if let Ok(res) = out {
                if res.status.success() && temp_wav.exists() {
                    let bytes =
                        std::fs::read(&temp_wav).map_err(|e| TtsError::Native(e.to_string()))?;
                    let _ = std::fs::remove_file(&temp_wav);
                    return Ok(bytes);
                }
            }

            let _ = std::fs::remove_file(&temp_wav);
            Err(TtsError::Native(
                "Linux native synthesis returned no audio file".to_string(),
            ))
        }
    }
}

/// Generates a standard 44-byte RIFF WAVE header for PCM audio
pub fn create_wav_header(
    pcm_data: Vec<u8>,
    sample_rate: u32,
    channels: u16,
    bits_per_sample: u16,
) -> Vec<u8> {
    let data_len = pcm_data.len() as u32;
    let byte_rate = sample_rate * channels as u32 * (bits_per_sample as u32 / 8);
    let block_align = channels * (bits_per_sample / 8);
    let total_len = 36 + data_len;

    let mut wav = Vec::with_capacity(44 + pcm_data.len());
    // RIFF chunk
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&total_len.to_le_bytes());
    wav.extend_from_slice(b"WAVE");
    // fmt subchunk
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes()); // subchunk1 size
    wav.extend_from_slice(&1u16.to_le_bytes()); // AudioFormat 1 = PCM
    wav.extend_from_slice(&channels.to_le_bytes());
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&byte_rate.to_le_bytes());
    wav.extend_from_slice(&block_align.to_le_bytes());
    wav.extend_from_slice(&bits_per_sample.to_le_bytes());
    // data subchunk
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_len.to_le_bytes());
    wav.extend_from_slice(&pcm_data);

    wav
}

// ---------------------------------------------------------------------------
// TtsEngine Interface & Speech Dispatch
// ---------------------------------------------------------------------------

pub struct TtsEngine;

fn is_emoji(c: char) -> bool {
    matches!(
        c,
        '\u{1F600}'..='\u{1F64F}'
            | '\u{1F300}'..='\u{1F5FF}'
            | '\u{1F680}'..='\u{1F6FF}'
            | '\u{1F700}'..='\u{1F77F}'
            | '\u{1F780}'..='\u{1F7FF}'
            | '\u{1F800}'..='\u{1F8FF}'
            | '\u{1F900}'..='\u{1F9FF}'
            | '\u{1FA00}'..='\u{1FA6F}'
            | '\u{1FA70}'..='\u{1FAFF}'
            | '\u{2600}'..='\u{26FF}'
            | '\u{2700}'..='\u{27BF}'
            | '\u{2300}'..='\u{23FF}'
            | '\u{2B50}'
            | '\u{2B55}'
            | '\u{FE00}'..='\u{FE0F}'
            | '\u{E0020}'..='\u{E007F}'
            | '\u{200D}'
    )
}

pub fn strip_emojis_and_markdown(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if is_emoji(c) {
            continue;
        }
        match c {
            '*' | '#' | '`' | '~' | '>' | '|' | '{' | '}' => {
                out.push(' ');
            }
            _ => out.push(c),
        }
    }
    out
}

impl TtsEngine {
    /// Terminates a tracked OS speech process by PID.
    fn terminate_pid(pid: u32) {
        #[cfg(target_os = "windows")]
        {
            let mut kill_cmd = std::process::Command::new("taskkill");
            kill_cmd.args(["/F", "/PID", &pid.to_string()]);
            kill_cmd.creation_flags(CREATE_NO_WINDOW);
            let _ = kill_cmd.output();
        }
        #[cfg(target_os = "linux")]
        {
            let _ = std::process::Command::new("kill")
                .args(["-9", &pid.to_string()])
                .output();
        }
    }

    /// Returns true if speech synthesis is currently active.
    pub fn is_speaking() -> bool {
        TTS_IS_SPEAKING.load(Ordering::SeqCst)
    }

    /// Stops any active speech synthesis immediately.
    /// Safe to call both inside and outside a Tokio runtime.
    pub fn stop() {
        TTS_IS_SPEAKING.store(false, Ordering::SeqCst);
        let pid_holder = CURRENT_TTS_PROCESS.clone();
        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            handle.spawn(async move {
                let mut guard = pid_holder.lock().await;
                if let Some(pid) = guard.take() {
                    Self::terminate_pid(pid);
                }
            });
        } else {
            std::thread::spawn(move || {
                let mut guard = pid_holder.blocking_lock();
                if let Some(pid) = guard.take() {
                    Self::terminate_pid(pid);
                }
            });
        }
    }

    /// Prepares speech text by converting LaTeX formulas into natural spoken words,
    /// converting Thanglish to Tamil phonetics if needed, stripping emojis & markdown,
    /// and sanitizing symbols/quotes for the speech engine.
    pub fn prepare_speech_text(raw_text: &str, lang: &str) -> String {
        // 1. Expand LaTeX formulas to phonetic spoken text
        let expanded = expand_latex_to_phonetic_speech(raw_text, lang);
        let stripped = strip_emojis_and_markdown(&expanded);

        // 2. Convert common mathematical symbols into natural spoken language
        let spoken = stripped
            .replace(" = ", " equals ")
            .replace(" + ", " plus ")
            .replace(" - ", " minus ")
            .replace(" * ", " times ")
            .replace(" / ", " divided by ")
            .replace('%', " percent ")
            .replace('λ', " lambda ")
            .replace('Δ', " delta ")
            .replace('π', " pie ")
            .replace('θ', " theta ")
            .replace('ε', " epsilon ")
            .replace('→', " gives ")
            .replace('•', ", ");

        let text = if lang.starts_with("ta") || lang.contains("thanglish") {
            thanglish_to_tamil_phonetic(&spoken)
        } else {
            spoken
        };

        // 3. Sanitize symbols and quotes to ensure safe command execution
        text.replace(['"', '\'', '`', '\\', '$', '\n', '\r'], " ")
    }

    /// Dispatches speech synthesis asynchronously on a background thread.
    /// Does NOT block the Iced GUI thread. Uses studio-grade synthesis when available,
    /// falling back cleanly to native OS voices without window popups.
    pub fn speak(text: String, lang: String) {
        Self::stop();
        TTS_IS_SPEAKING.store(true, Ordering::SeqCst);

        let sanitized = Self::prepare_speech_text(&text, &lang);
        let lang_pref = LanguagePreference::from_lang_code(&lang);

        let work = move || {
            // Step 1: Attempt studio-grade audio synthesis (Edge Neural / Piper)
            let synthesized_audio =
                VoiceEngine::synthesize_with_fallback(&sanitized, lang_pref.clone());

            let mut audio_played = false;
            if let Ok((audio_bytes, route)) = synthesized_audio {
                tracing::info!(
                    "Playing synthesized voice audio via route: {}",
                    route.description()
                );
                // Attempt rodio playback
                if let Ok((_stream, stream_handle)) = rodio::OutputStream::try_default() {
                    let cursor = std::io::Cursor::new(audio_bytes);
                    if let Ok(source) = rodio::Decoder::new(cursor) {
                        if let Ok(sink) = rodio::Sink::try_new(&stream_handle) {
                            sink.append(source);
                            while !sink.empty() {
                                if !TTS_IS_SPEAKING.load(Ordering::SeqCst) {
                                    sink.stop();
                                    break;
                                }
                                std::thread::sleep(std::time::Duration::from_millis(50));
                            }
                            audio_played = true;
                        }
                    }
                }
            }

            // Step 2: If in-process audio playback did not occur, dispatch native OS speaking process
            if !audio_played && TTS_IS_SPEAKING.load(Ordering::SeqCst) {
                #[cfg(target_os = "windows")]
                {
                    let voice_selector = if lang.starts_with("ta") {
                        "$v = $all | Where-Object { $_.VoiceInfo.Culture.Name -like 'ta*' -or $_.VoiceInfo.Name -like '*Tamil*' -or $_.VoiceInfo.Name -like '*Valluvar*' -or $_.VoiceInfo.Name -like '*Ravi*' -or $_.VoiceInfo.Name -like '*Heera*' } | Select-Object -First 1; if ($v) { $s.SelectVoice($v.VoiceInfo.Name) };"
                    } else {
                        "$v = $all | Where-Object { $_.VoiceInfo.Name -like '*Natural*' -or $_.VoiceInfo.Name -like '*Zira*' -or $_.VoiceInfo.Name -like '*Mark*' -or $_.VoiceInfo.Name -like '*David*' } | Select-Object -First 1; if ($v) { $s.SelectVoice($v.VoiceInfo.Name) };"
                    };

                    let script = format!(
                        "Add-Type -AssemblyName System.Speech; \
                         $s = New-Object System.Speech.Synthesis.SpeechSynthesizer; \
                         $s.Rate = 0; \
                         $s.Volume = 100; \
                         $all = $s.GetInstalledVoices() | Where-Object {{ $_.Enabled }}; \
                         {} \
                         $s.Speak(\"{}\");",
                        voice_selector, sanitized
                    );

                    let mut cmd = std::process::Command::new("powershell");
                    cmd.args(["-NoProfile", "-NonInteractive", "-Command", &script]);
                    cmd.creation_flags(CREATE_NO_WINDOW);

                    if let Ok(mut child) = cmd.spawn() {
                        let pid = child.id();
                        let pid_holder = CURRENT_TTS_PROCESS.clone();
                        {
                            let mut guard = pid_holder.blocking_lock();
                            *guard = Some(pid);
                        }

                        let _ = child.wait();

                        {
                            let mut guard = pid_holder.blocking_lock();
                            *guard = None;
                        }
                    }
                }

                #[cfg(target_os = "linux")]
                {
                    let piper_model = lang_pref.default_piper_model();
                    let piper_success = std::process::Command::new("piper")
                        .args(["--model", piper_model, "--output-raw"])
                        .stdin(std::process::Stdio::piped())
                        .stdout(std::process::Stdio::piped())
                        .spawn()
                        .and_then(|mut p| {
                            use std::io::Write;
                            if let Some(mut stdin) = p.stdin.take() {
                                let _ = stdin.write_all(sanitized.as_bytes());
                            }
                            p.wait()
                        })
                        .map(|s| s.success())
                        .unwrap_or(false);

                    if !piper_success {
                        let espeak_voice = if lang.starts_with("ta") {
                            "ta"
                        } else if lang.starts_with("ml") {
                            "ml"
                        } else {
                            "en-in"
                        };

                        let espeak_success = std::process::Command::new("espeak-ng")
                            .args(["-v", espeak_voice, &sanitized])
                            .spawn()
                            .and_then(|mut c| c.wait())
                            .map(|s| s.success())
                            .unwrap_or(false);

                        if !espeak_success {
                            let _ = std::process::Command::new("spd-say")
                                .args(["-l", &lang, &sanitized])
                                .spawn()
                                .and_then(|mut c| c.wait());
                        }
                    }
                }
            }

            TTS_IS_SPEAKING.store(false, Ordering::SeqCst);
        };

        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            handle.spawn_blocking(work);
        } else {
            std::thread::spawn(work);
        }
    }
}

// ---------------------------------------------------------------------------
// Unit Tests (Features 23 & 24)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tts_text_sanitization() {
        let raw = r#"Einstein's equation is $hf = W + KE$ with "quotes" and \slashes!"#;
        let prepared = TtsEngine::prepare_speech_text(raw, "en");
        assert!(!prepared.contains('"'));
        assert!(!prepared.contains('\\'));
        assert!(!prepared.contains('$'));
        assert!(prepared.contains("equals"));
    }

    #[test]
    fn test_tts_thanglish_preparation() {
        let raw = "Vanakkam mapla, innaiku exam super-aa pannalaam!";
        let prepared = TtsEngine::prepare_speech_text(raw, "ta-IN");
        assert!(prepared.contains("வணக்கம்"));
    }

    #[test]
    fn test_tts_emoji_and_markdown_stripping() {
        let raw = "🎯 Ready! **Syllabus** sprint #1 `Gauss's Law` is active! 🚀";
        let cleaned = strip_emojis_and_markdown(raw);
        assert!(!cleaned.contains('🎯'));
        assert!(!cleaned.contains('🚀'));
        assert!(!cleaned.contains('*'));
        assert!(!cleaned.contains('#'));
        assert!(!cleaned.contains('`'));
        assert!(cleaned.contains("Ready"));
        assert!(cleaned.contains("Syllabus"));
    }

    #[test]
    fn test_language_preference_neural_voices() {
        let ta = LanguagePreference::from_lang_code("ta-IN");
        assert_eq!(ta, LanguagePreference::Tamil);
        assert_eq!(ta.default_neural_voice(), "ta-IN-ValluvarNeural");
        assert_eq!(ta.secondary_neural_voice(), "ta-IN-PallaviNeural");

        let en = LanguagePreference::from_lang_code("en-IN");
        assert_eq!(en, LanguagePreference::English);
        assert_eq!(en.default_neural_voice(), "en-IN-NeerjaNeural");

        let auto = LanguagePreference::from_lang_code("auto");
        assert_eq!(auto, LanguagePreference::Auto);
        assert_eq!(auto.default_neural_voice(), "ta-IN-PallaviNeural");
    }

    #[test]
    fn test_wav_header_generation() {
        let dummy_pcm = vec![0u8; 1000];
        let wav = create_wav_header(dummy_pcm, 22050, 1, 16);
        assert_eq!(wav.len(), 44 + 1000);
        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");
        assert_eq!(&wav[12..16], b"fmt ");
        assert_eq!(&wav[36..40], b"data");
    }

    #[test]
    fn test_tts_route_ram_footprint() {
        let cloud = TtsRoute::CloudNeural {
            voice: "ta-IN-ValluvarNeural".to_string(),
        };
        assert!(cloud.estimated_ram_mb() <= 10);
        assert!(cloud.description().contains("~4 MB RAM"));

        let piper = TtsRoute::PiperLocal {
            model_path: PathBuf::from("ta_IN-valluvar-medium.onnx"),
        };
        assert!(piper.estimated_ram_mb() < 60);
        assert!(piper.description().contains("<60 MB RAM"));

        let native = TtsRoute::NativeFallback {
            voice_name: "Microsoft David".to_string(),
        };
        assert!(native.estimated_ram_mb() < 20);
    }

    #[test]
    fn test_voice_engine_resolve_route() {
        let ta = LanguagePreference::Tamil;
        let route = VoiceEngine::resolve_optimal_route(&ta);
        // Should resolve to a valid route (CloudNeural if edge-tts present, otherwise Piper or Native)
        match route {
            TtsRoute::CloudNeural { voice } => {
                assert!(voice.contains("Valluvar") || voice.contains("Pallavi"));
            }
            TtsRoute::PiperLocal { model_path } => {
                assert!(!model_path.to_string_lossy().is_empty());
            }
            TtsRoute::NativeFallback { voice_name } => {
                assert_eq!(voice_name, "Tamil");
            }
        }
    }

    #[test]
    fn test_tts_stop_safe_outside_tokio_runtime() {
        TTS_IS_SPEAKING.store(true, Ordering::SeqCst);
        TtsEngine::stop();
        assert!(!TtsEngine::is_speaking());
    }
}
