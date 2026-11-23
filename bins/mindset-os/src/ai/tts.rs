//! Bilingual Text-To-Speech (TTS) Engine for MindSet OS
//!
//! Provides non-blocking Windows native TTS speech synthesis with support for
//! Tamil, Malayalam, and English phonetics, script segmentation, and Dynamic Island waveform sync.
//! Suppresses black console popups via CREATE_NO_WINDOW on Windows and routes through
//! piper -> espeak-ng -> spd-say on Debian 13 Linux.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

use super::persona::thanglish_to_tamil_phonetic;

pub static TTS_IS_SPEAKING: std::sync::LazyLock<Arc<AtomicBool>> =
    std::sync::LazyLock::new(|| Arc::new(AtomicBool::new(false)));

pub static CURRENT_TTS_PROCESS: std::sync::LazyLock<Arc<Mutex<Option<u32>>>> =
    std::sync::LazyLock::new(|| Arc::new(Mutex::new(None)));

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
    /// Returns true if speech synthesis is currently active.
    pub fn is_speaking() -> bool {
        TTS_IS_SPEAKING.load(Ordering::SeqCst)
    }

    /// Stops any active speech synthesis immediately.
    pub fn stop() {
        TTS_IS_SPEAKING.store(false, Ordering::SeqCst);
        let pid_holder = CURRENT_TTS_PROCESS.clone();
        tokio::spawn(async move {
            let mut guard = pid_holder.lock().await;
            if let Some(pid) = guard.take() {
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
        });
    }

    /// Prepares speech text by converting Thanglish to Tamil phonetics if needed,
    /// stripping emojis & markdown, expanding formulas into natural spoken words,
    /// and sanitizing symbols/quotes for the speech engine.
    pub fn prepare_speech_text(raw_text: &str, lang: &str) -> String {
        let stripped = strip_emojis_and_markdown(raw_text);

        // Convert common mathematical symbols into natural spoken language
        let spoken = stripped
            .replace(" = ", " equals ")
            .replace(" + ", " plus ")
            .replace(" - ", " minus ")
            .replace(" * ", " times ")
            .replace(" / ", " divided by ")
            .replace("%", " percent ")
            .replace('λ', " lambda ")
            .replace('Δ', " delta ")
            .replace('π', " pie ")
            .replace('θ', " theta ")
            .replace('ε', " epsilon ")
            .replace('→', " gives ")
            .replace('•', ", ");

        let text = if lang.starts_with("ta") {
            thanglish_to_tamil_phonetic(&spoken)
        } else {
            spoken
        };

        // Sanitize symbols and quotes to ensure safe command execution
        text.replace(['"', '\'', '`', '\\', '$', '\n', '\r'], " ")
    }

    /// Dispatches speech synthesis asynchronously on a background thread.
    /// Does NOT block the Iced GUI thread. Suppresses console windows on Windows.
    pub fn speak(text: String, lang: String) {
        Self::stop(); // Stop any currently playing audio
        TTS_IS_SPEAKING.store(true, Ordering::SeqCst);

        let sanitized = Self::prepare_speech_text(&text, &lang);

        tokio::task::spawn_blocking(move || {
            #[cfg(target_os = "windows")]
            {
                // Dynamic natural voice selector for Windows (picks modern natural voices for Tamil/English)
                let voice_selector = if lang.starts_with("ta") {
                    "$v = $all | Where-Object { $_.VoiceInfo.Culture.Name -like 'ta*' -or $_.VoiceInfo.Name -like '*Tamil*' -or $_.VoiceInfo.Name -like '*Valluvar*' -or $_.VoiceInfo.Name -like '*Ravi*' -or $_.VoiceInfo.Name -like '*Heera*' } | Select-Object -First 1; if ($v) { $s.SelectVoice($v.VoiceInfo.Name) };"
                } else {
                    "$v = $all | Where-Object { $_.VoiceInfo.Name -like '*Natural*' -or $_.VoiceInfo.Name -like '*Zira*' -or $_.VoiceInfo.Name -like '*Mark*' -or $_.VoiceInfo.Name -like '*Ravi*' -or $_.VoiceInfo.Name -like '*Eva*' } | Select-Object -First 1; if ($v) { $s.SelectVoice($v.VoiceInfo.Name) };"
                };

                // Invoke Windows Native SAPI / System.Speech via PowerShell with NO WINDOW
                let script = format!(
                    "Add-Type -AssemblyName System.Speech; \
                     $s = New-Object System.Speech.Synthesis.SpeechSynthesizer; \
                     $s.Rate = 0; \
                     $s.Volume = 100; \
                     $all = $s.GetInstalledVoices() | Where-Object {{ $_.Enabled }}; \
                     {} \
                     $s.Speak(\"{}\");",
                    voice_selector,
                    sanitized
                );

                let mut cmd = std::process::Command::new("powershell");
                cmd.args(["-NoProfile", "-NonInteractive", "-Command", &script]);
                cmd.creation_flags(CREATE_NO_WINDOW);

                if let Ok(mut child) = cmd.spawn() {
                    let pid = child.id();
                    let pid_holder = CURRENT_TTS_PROCESS.clone();
                    tokio::runtime::Handle::current().block_on(async {
                        let mut guard = pid_holder.lock().await;
                        *guard = Some(pid);
                    });

                    let _ = child.wait();

                    tokio::runtime::Handle::current().block_on(async {
                        let mut guard = pid_holder.lock().await;
                        *guard = None;
                    });
                }
            }

            #[cfg(target_os = "linux")]
            {
                // Debian 13 Pipeline: piper -> espeak-ng (-v ta, -v ml, -v en-in) -> spd-say
                let piper_model = if lang.starts_with("ta") {
                    "ta_IN"
                } else if lang.starts_with("ml") {
                    "ml_IN"
                } else {
                    "en_US-lessac-medium"
                };

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

            TTS_IS_SPEAKING.store(false, Ordering::SeqCst);
        });
    }
}

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
}
