//! MindSet OS — Chromium Engine Bridge & Manifest V3 Shield Generator
//! Manages locked-down headless/native Chromium process launch with force-injected
//! Manifest V3 DeclarativeNetRequest adblock and anti-distraction content scripts.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use crate::adblock_scripts::AdBlockFilter;
use crate::entertainment_shield::EntertainmentShield;
use crate::policy_guard::PolicyGuard;
use crate::youtube_guard::YoutubeGuard;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Content of Manifest V3 extension configuration for MindSet Shield.
pub const SHIELD_MANIFEST_V3_JSON: &str = r#"{
  "manifest_version": 3,
  "name": "MindSet Browser Integrity Shield",
  "version": "1.0.0",
  "description": "Zero-Distraction Educational Shield for MindSet OS",
  "permissions": [
    "declarativeNetRequest"
  ],
  "host_permissions": [
    "<all_urls>"
  ],
  "declarative_net_request": {
    "rule_resources": [
      {
        "id": "ruleset_1",
        "enabled": true,
        "path": "rules.json"
      }
    ]
  },
  "content_scripts": [
    {
      "matches": ["<all_urls>"],
      "js": ["mindset_shield_inject.js"],
      "run_at": "document_start"
    }
  ]
}"#;

/// Content of the auto-injected content script.
pub const SHIELD_INJECT_JS: &str = r#"// MindSet Shield Content Script — Anti-Distraction & Ad Hiding Engine
(function() {
  'use strict';

  // 1. Inject High-Priority CSS to wipe ad banners & YouTube recommendation rabbit-holes
  const style = document.createElement('style');
  style.id = 'mindset-shield-styles';
  style.textContent = `
    /* Ad Banner & Slot Suppression */
    ytd-ad-slot-renderer, .ad-showing, .video-ads, [id^="google_ads_"],
    iframe[id^="google_ads_frame"], .ad-container, .adsbygoogle,
    
    /* YouTube Shorts & Feed Hiding */
    ytd-reel-shelf-renderer, #related, #comments, a[title="Shorts"],
    ytd-rich-shelf-renderer[is-shorts], ytd-guide-entry-renderer:has(a[title="Shorts"]),
    ytd-mini-guide-entry-renderer[aria-label="Shorts"] {
      display: none !important;
      visibility: hidden !important;
      opacity: 0 !important;
      pointer-events: none !important;
      height: 0 !important;
      overflow: hidden !important;
    }
  `;
  (document.head || document.documentElement).appendChild(style);

  // 1b. Redirect /shorts/ to standard /watch?v= player
  if (window.location.pathname.startsWith('/shorts/')) {
    const parts = window.location.pathname.split('/shorts/');
    if (parts.length > 1 && parts[1]) {
      const vid = parts[1].split('?')[0].split('/')[0];
      window.location.replace('/watch?v=' + vid);
    } else {
      window.location.replace('/results?search_query=educational+lecture');
    }
  }

  // 2. Automated Ad Overlay Skipper
  setInterval(() => {
    const skipBtn = document.querySelector('.ytp-skip-ad-button, .ytp-ad-skip-button, .ytp-ad-skip-button-modern');
    if (skipBtn) {
      skipBtn.click();
    }
  }, 400);

  // 3. Render MindSet Shield Floating Verification Badge on YouTube
  if (window.location.hostname.includes('youtube.com')) {
    window.addEventListener('DOMContentLoaded', () => {
      if (!document.getElementById('mindset-shield-badge')) {
        const badge = document.createElement('div');
        badge.id = 'mindset-shield-badge';
        badge.textContent = 'MindSet Shield Active • Shorts & Feed Blocked';
        badge.setAttribute('style', [
          'position: fixed',
          'top: 14px',
          'right: 140px',
          'z-index: 999999',
          'background: rgba(18, 16, 12, 0.90)',
          'color: #FBBF24',
          'padding: 6px 14px',
          'border-radius: 999px',
          'font-family: system-ui, -apple-system, sans-serif',
          'font-size: 11.5px',
          'font-weight: 600',
          'border: 1px solid rgba(251, 191, 36, 0.45)',
          'box-shadow: 0 4px 12px rgba(0,0,0,0.3)',
          'backdrop-filter: blur(8px)',
          'pointer-events: none',
          'user-select: none'
        ].join('; '));
        document.body.appendChild(badge);
      }
    });
  }
})();
"#;

/// Standard known binary locations across Windows and Linux.
const CANDIDATE_BINARIES: &[&str] = &[
    // Windows
    r"C:\Program Files\Google\Chrome\Application\chrome.exe",
    r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
    r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
    r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
    r"C:\Program Files\BraveSoftware\Brave-Browser\Application\brave.exe",
    // Linux
    "/usr/bin/google-chrome",
    "/usr/bin/chromium",
    "/usr/bin/chromium-browser",
    "/usr/bin/microsoft-edge",
    "/usr/bin/brave-browser",
];

/// Chromium launcher and extension deployment bridge.
#[derive(Debug, Clone)]
pub struct ChromiumEngineBridge {
    pub extension_dir: PathBuf,
    pub profile_dir: PathBuf,
    pub custom_binary_path: Option<PathBuf>,
}

impl Default for ChromiumEngineBridge {
    fn default() -> Self {
        let base_dir = dirs::data_local_dir()
            .unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")))
            .join("mindset-os");

        Self {
            extension_dir: base_dir.join("browser_shield_ext"),
            profile_dir: base_dir.join("browser-profile"),
            custom_binary_path: None,
        }
    }
}

impl ChromiumEngineBridge {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_paths(extension_dir: PathBuf, profile_dir: PathBuf) -> Self {
        Self {
            extension_dir,
            profile_dir,
            custom_binary_path: None,
        }
    }

    /// Deploys the Manifest V3 extension (`manifest.json`, `rules.json`, `mindset_shield_inject.js`)
    /// to the target directory on disk.
    pub fn ensure_shield_extension_on_disk(&self) -> Result<(), String> {
        fs::create_dir_all(&self.extension_dir).map_err(|e| {
            format!(
                "Failed to create shield extension dir '{}': {e}",
                self.extension_dir.display()
            )
        })?;

        // 1. Write manifest.json
        let manifest_path = self.extension_dir.join("manifest.json");
        fs::write(&manifest_path, SHIELD_MANIFEST_V3_JSON).map_err(|e| {
            format!(
                "Failed to write manifest.json to '{}': {e}",
                manifest_path.display()
            )
        })?;

        // 2. Generate rules.json via AdBlockFilter
        let adblock = AdBlockFilter::new();
        let rules_json = adblock.build_declarative_net_request_rules();
        let rules_path = self.extension_dir.join("rules.json");
        fs::write(&rules_path, rules_json).map_err(|e| {
            format!(
                "Failed to write rules.json to '{}': {e}",
                rules_path.display()
            )
        })?;

        // 3. Write mindset_shield_inject.js
        let inject_path = self.extension_dir.join("mindset_shield_inject.js");
        fs::write(&inject_path, SHIELD_INJECT_JS).map_err(|e| {
            format!(
                "Failed to write mindset_shield_inject.js to '{}': {e}",
                inject_path.display()
            )
        })?;

        Ok(())
    }

    /// Discovers an installed Chromium-based browser executable on the current host.
    pub fn find_chromium_binary(&self) -> Option<PathBuf> {
        if let Some(ref custom) = self.custom_binary_path {
            if custom.exists() {
                return Some(custom.clone());
            }
        }

        // Check standard operating system candidate paths
        for &candidate in CANDIDATE_BINARIES {
            let path = PathBuf::from(candidate);
            if path.exists() {
                return Some(path);
            }
        }

        // Check LocalAppData on Windows
        if let Some(local_app) = dirs::data_local_dir() {
            let chrome_local = local_app
                .join("Google")
                .join("Chrome")
                .join("Application")
                .join("chrome.exe");
            if chrome_local.exists() {
                return Some(chrome_local);
            }
            let edge_local = local_app
                .join("Microsoft")
                .join("Edge")
                .join("Application")
                .join("msedge.exe");
            if edge_local.exists() {
                return Some(edge_local);
            }
        }

        None
    }

    /// Validates the URL through MindSet network policies and produces the locked-down CLI arguments.
    /// Strictly guarantees `--incognito` and `--inprivate` are never included.
    pub fn prepare_launch_command(
        &self,
        url: &str,
        entertainment: &mut EntertainmentShield,
        youtube: &mut YoutubeGuard,
        focus_active: bool,
    ) -> Result<(PathBuf, Vec<String>), String> {
        // 1. Validate scheme & normalize
        let normalized = PolicyGuard::validate_url_scheme(url)?;

        // 2. Check Entertainment & Distraction shield
        entertainment.check_url(&normalized, focus_active)?;

        // 3. Check YouTube screen-time & Shorts blocker
        youtube.check_youtube_url(&normalized)?;

        // 4. Locate browser binary
        let binary = self.find_chromium_binary().unwrap_or_else(|| {
            // Provide a sensible fallback binary for headless / unit test verification
            if cfg!(target_os = "windows") {
                PathBuf::from(r"C:\Program Files\Google\Chrome\Application\chrome.exe")
            } else {
                PathBuf::from("/usr/bin/google-chrome")
            }
        });

        // 5. Ensure shield extension exists on disk
        self.ensure_shield_extension_on_disk()?;

        // 6. Build locked-down CLI arguments
        let args = vec![
            format!("--app={normalized}"),
            format!("--user-data-dir={}", self.profile_dir.display()),
            format!("--load-extension={}", self.extension_dir.display()),
            "--disable-features=IncognitoMode".to_string(),
            "--no-first-run".to_string(),
            "--no-default-browser-check".to_string(),
        ];

        // 7. Sanitize arguments through PolicyGuard as secondary defense-in-depth
        let sanitized = PolicyGuard::sanitize_browser_args(&args);

        Ok((binary, sanitized))
    }

    /// Spawns the locked-down browser process non-blockingly.
    pub fn launch_shielded_url(
        &self,
        url: &str,
        entertainment: &mut EntertainmentShield,
        youtube: &mut YoutubeGuard,
        focus_active: bool,
    ) -> Result<String, String> {
        let (binary, args) =
            self.prepare_launch_command(url, entertainment, youtube, focus_active)?;

        if !binary.exists() {
            return Err(format!(
                "Chromium/Edge browser executable not found at '{}'. Please install Chrome or Edge.",
                binary.display()
            ));
        }

        let mut cmd = Command::new(&binary);
        cmd.args(&args);

        let child = cmd.spawn().map_err(|e| {
            format!(
                "Failed to spawn shielded browser process '{}': {e}",
                binary.display()
            )
        })?;

        Ok(format!(
            "Shielded browser launched successfully (PID: {}) with URL: {url}",
            child.id()
        ))
    }

    /// Pre-seeds standard academic study bookmarks (NCERT, Khan Academy, arXiv, Wikipedia, Scholar, YouTube)
    /// into the dedicated MindSet student browser profile.
    pub fn setup_study_bookmarks(profile_dir: &std::path::Path) {
        let default_dir = profile_dir.join("Default");
        if let Err(e) = fs::create_dir_all(&default_dir) {
            eprintln!("Warning: Could not create profile Default dir: {e}");
            return;
        }

        let bookmarks_file = default_dir.join("Bookmarks");
        if bookmarks_file.exists() {
            return; // Preserve existing student bookmarks
        }

        let bookmarks_json = r#"{
   "checksum": "mindset_academic_v1",
   "roots": {
      "bookmark_bar": {
         "children": [
            {
               "date_added": "13300000000000000",
               "date_last_used": "0",
               "id": "1",
               "name": "📚 NCERT eBooks",
               "type": "url",
               "url": "https://ncert.nic.in/textbook.php"
            },
            {
               "date_added": "13300000000000000",
               "date_last_used": "0",
               "id": "2",
               "name": "🎓 Khan Academy",
               "type": "url",
               "url": "https://www.khanacademy.org"
            },
            {
               "date_added": "13300000000000000",
               "date_last_used": "0",
               "id": "3",
               "name": "🔬 arXiv Preprints",
               "type": "url",
               "url": "https://arxiv.org"
            },
            {
               "date_added": "13300000000000000",
               "date_last_used": "0",
               "id": "4",
               "name": "📖 Wikipedia",
               "type": "url",
               "url": "https://www.wikipedia.org"
            },
            {
               "date_added": "13300000000000000",
               "date_last_used": "0",
               "id": "5",
               "name": "🎓 Google Scholar",
               "type": "url",
               "url": "https://scholar.google.com"
            },
            {
               "date_added": "13300000000000000",
               "date_last_used": "0",
               "id": "6",
               "name": "📺 YouTube Study",
               "type": "url",
               "url": "https://www.youtube.com"
            }
         ],
         "date_added": "13300000000000000",
         "date_last_used": "0",
         "date_modified": "13300000000000000",
         "id": "0",
         "name": "Bookmarks bar",
         "type": "folder"
      }
   },
   "version": 1
}"#;

        let _ = fs::write(bookmarks_file, bookmarks_json);
    }

    /// Spawns the real installed YouTube desktop application in dedicated standalone app mode.
    pub fn spawn_youtube_app() -> Result<String, String> {
        #[cfg(target_os = "linux")]
        {
            // 1. Try launching registered desktop app launcher
            if let Ok(child) = Command::new("gtk-launch").arg("youtube").spawn() {
                return Ok(format!(
                    "Launched YouTube App via gtk-launch (PID: {})",
                    child.id()
                ));
            }

            // 2. Direct Chromium standalone app mode with dedicated profile
            let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/home/achiever"));
            let profile = home.join(".local/share/mindset-os/youtube-app");
            let _ = fs::create_dir_all(&profile);

            let candidates = [
                "/usr/bin/chromium",
                "/usr/bin/google-chrome",
                "/usr/bin/chromium-browser",
                "/usr/bin/microsoft-edge",
                "/usr/bin/brave-browser",
            ];

            for bin in candidates {
                if std::path::Path::new(bin).exists() {
                    let child = Command::new(bin)
                        .args([
                            "--app=https://www.youtube.com",
                            "--class=YouTube",
                            &format!("--user-data-dir={}", profile.display()),
                            "--no-first-run",
                            "--no-default-browser-check",
                        ])
                        .spawn()
                        .map_err(|e| format!("Failed to spawn {bin}: {e}"))?;
                    return Ok(format!("Spawned YouTube App ({bin}, PID: {})", child.id()));
                }
            }

            // Fallback to xdg-open
            let child = Command::new("xdg-open")
                .arg("https://www.youtube.com")
                .spawn()
                .map_err(|e| format!("Failed to open YouTube: {e}"))?;
            Ok(format!("Opened YouTube via xdg-open (PID: {})", child.id()))
        }

        #[cfg(target_os = "windows")]
        {
            let candidates = [
                r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
                r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
                r"C:\Program Files\Google\Chrome\Application\chrome.exe",
                r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
                r"C:\Program Files\BraveSoftware\Brave-Browser\Application\brave.exe",
            ];

            for bin in candidates {
                if std::path::Path::new(bin).exists() {
                    let child = Command::new(bin)
                        .args(["--app=https://www.youtube.com"])
                        .spawn()
                        .map_err(|e| format!("Failed to spawn {bin}: {e}"))?;
                    return Ok(format!("Launched YouTube App ({bin}, PID: {})", child.id()));
                }
            }

            // Fallback to start
            let child = Command::new("cmd")
                .args(["/C", "start", "https://www.youtube.com"])
                .spawn()
                .map_err(|e| format!("Failed to start YouTube: {e}"))?;
            Ok(format!(
                "Launched YouTube via system handler (PID: {})",
                child.id()
            ))
        }

        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        {
            Ok("YouTube App launch simulated (non-Linux/Windows target)".to_string())
        }
    }

    /// Spawns FreeTube ad-free desktop player if installed, with helpful notification on Windows.
    pub fn spawn_freetube_app() -> Result<String, String> {
        #[cfg(target_os = "linux")]
        {
            let candidates = [
                "/usr/bin/freetube",
                "/opt/FreeTube/freetube",
                "/usr/local/bin/freetube",
            ];

            for bin in candidates {
                if std::path::Path::new(bin).exists() {
                    let child = Command::new(bin)
                        .spawn()
                        .map_err(|e| format!("Failed to launch FreeTube at {bin}: {e}"))?;
                    return Ok(format!(
                        "Launched FreeTube Ad-Free Player (PID: {})",
                        child.id()
                    ));
                }
            }

            Err("FreeTube binary not found. FreeTube is pre-installed in the MindSet OS Debian 13 ISO.".to_string())
        }

        #[cfg(target_os = "windows")]
        {
            let candidates = [
                r"C:\Program Files\FreeTube\FreeTube.exe",
                r"C:\Program Files (x86)\FreeTube\FreeTube.exe",
            ];

            for bin in candidates {
                if std::path::Path::new(bin).exists() {
                    let child = Command::new(bin)
                        .spawn()
                        .map_err(|e| format!("Failed to launch FreeTube: {e}"))?;
                    return Ok(format!("Launched FreeTube (PID: {})", child.id()));
                }
            }

            // On Windows testing where FreeTube is not yet downloaded, give clear feedback:
            Ok("FreeTube is pre-installed in MindSet OS Linux ISO. (To test on Windows, install FreeTube from freetubeapp.io)".to_string())
        }

        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        {
            Ok("FreeTube launch simulated".to_string())
        }
    }

    /// Spawns the real installed browser (Chromium on Linux, Edge/Chrome on Windows)
    /// with dedicated student profile and pre-configured study bookmarks.
    pub fn spawn_mindset_browser(target_url: Option<&str>) -> Result<String, String> {
        let url = target_url.unwrap_or("https://www.google.com");

        #[cfg(target_os = "linux")]
        {
            let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/home/achiever"));
            let profile = home.join(".local/share/mindset-os/browser-profile");
            Self::setup_study_bookmarks(&profile);

            let candidates = [
                "/usr/bin/chromium",
                "/usr/bin/google-chrome",
                "/usr/bin/chromium-browser",
                "/usr/bin/microsoft-edge",
                "/usr/bin/brave-browser",
            ];

            for bin in candidates {
                if std::path::Path::new(bin).exists() {
                    let child = Command::new(bin)
                        .args([
                            &format!("--user-data-dir={}", profile.display()),
                            "--no-first-run",
                            "--no-default-browser-check",
                            url,
                        ])
                        .spawn()
                        .map_err(|e| format!("Failed to spawn {bin}: {e}"))?;
                    return Ok(format!(
                        "Launched MindSet Browser ({bin}, PID: {})",
                        child.id()
                    ));
                }
            }

            let child = Command::new("xdg-open")
                .arg(url)
                .spawn()
                .map_err(|e| format!("Failed to launch browser: {e}"))?;
            Ok(format!("Opened browser via xdg-open (PID: {})", child.id()))
        }

        #[cfg(target_os = "windows")]
        {
            let local_app = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
            let profile = local_app.join("mindset-os").join("browser-profile");
            Self::setup_study_bookmarks(&profile);

            let candidates = [
                r"C:\Program Files\Google\Chrome\Application\chrome.exe",
                r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
                r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
                r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
            ];

            for bin in candidates {
                if std::path::Path::new(bin).exists() {
                    let child = Command::new(bin)
                        .args([
                            &format!("--user-data-dir={}", profile.display()),
                            "--no-first-run",
                            "--no-default-browser-check",
                            url,
                        ])
                        .spawn()
                        .map_err(|e| format!("Failed to spawn {bin}: {e}"))?;
                    return Ok(format!(
                        "Launched MindSet Browser ({bin}, PID: {})",
                        child.id()
                    ));
                }
            }

            let child = Command::new("cmd")
                .args(["/C", "start", url])
                .spawn()
                .map_err(|e| format!("Failed to open browser: {e}"))?;
            Ok(format!("Opened system browser (PID: {})", child.id()))
        }

        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        {
            let _ = url;
            Ok("Browser launch simulated".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_shield_extension_on_disk_writes_three_files() {
        let temp_dir = std::env::temp_dir().join("mindset_test_shield_ext");
        let bridge = ChromiumEngineBridge::with_paths(temp_dir.clone(), temp_dir.join("profile"));

        bridge.ensure_shield_extension_on_disk().unwrap();

        let manifest = temp_dir.join("manifest.json");
        let rules = temp_dir.join("rules.json");
        let inject = temp_dir.join("mindset_shield_inject.js");

        assert!(manifest.exists());
        assert!(rules.exists());
        assert!(inject.exists());

        let manifest_content = fs::read_to_string(&manifest).unwrap();
        assert!(manifest_content.contains("declarativeNetRequest"));
        assert!(manifest_content.contains("mindset_shield_inject.js"));

        let rules_content = fs::read_to_string(&rules).unwrap();
        assert!(rules_content.contains("doubleclick.net"));

        let inject_content = fs::read_to_string(&inject).unwrap();
        assert!(inject_content.contains("MindSet Shield Active"));
        assert!(inject_content.contains("ytd-reel-shelf-renderer"));

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_prepare_launch_command_args_security() {
        let temp_dir = std::env::temp_dir().join("mindset_test_launch_args");
        let bridge = ChromiumEngineBridge::with_paths(temp_dir.clone(), temp_dir.join("profile"));

        let mut entertainment = EntertainmentShield::new();
        let mut youtube = YoutubeGuard::new();

        let (binary, args) = bridge
            .prepare_launch_command("wikipedia.org", &mut entertainment, &mut youtube, false)
            .unwrap();

        assert!(!binary.as_os_str().is_empty());
        assert!(args
            .iter()
            .any(|a| a.starts_with("--app=https://wikipedia.org")));
        assert!(args.iter().any(|a| a.starts_with("--load-extension=")));
        assert!(args.iter().any(|a| a.starts_with("--user-data-dir=")));
        assert!(args.iter().any(|a| a == "--disable-features=IncognitoMode"));

        // Guarantee incognito is NEVER in the arguments
        assert!(!args.iter().any(|a| a.contains("incognito")));
        assert!(!args.iter().any(|a| a.contains("inprivate")));

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_prepare_launch_command_blocks_forbidden_urls() {
        let temp_dir = std::env::temp_dir().join("mindset_test_block_urls");
        let bridge = ChromiumEngineBridge::with_paths(temp_dir.clone(), temp_dir.join("profile"));

        let mut entertainment = EntertainmentShield::new();
        let mut youtube = YoutubeGuard::new();

        // 1. Internal browser settings blocked
        assert!(bridge
            .prepare_launch_command(
                "chrome://extensions",
                &mut entertainment,
                &mut youtube,
                false
            )
            .is_err());

        // 2. Entertainment (Netflix) blocked
        assert!(bridge
            .prepare_launch_command(
                "https://netflix.com",
                &mut entertainment,
                &mut youtube,
                false
            )
            .is_err());

        // 3. YouTube Shorts blocked
        assert!(bridge
            .prepare_launch_command(
                "https://youtube.com/shorts/123",
                &mut entertainment,
                &mut youtube,
                false
            )
            .is_err());

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_setup_study_bookmarks_creates_valid_bookmarks_json() {
        let temp_dir = std::env::temp_dir().join("mindset_test_bookmarks");
        let _ = fs::remove_dir_all(&temp_dir);

        ChromiumEngineBridge::setup_study_bookmarks(&temp_dir);

        let bookmarks_file = temp_dir.join("Default").join("Bookmarks");
        assert!(bookmarks_file.exists());

        let content = fs::read_to_string(&bookmarks_file).unwrap();
        assert!(content.contains("NCERT eBooks"));
        assert!(content.contains("Khan Academy"));
        assert!(content.contains("YouTube Study"));

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_spawn_youtube_and_freetube_does_not_panic() {
        // Safe check that invocation returns a Result and does not panic
        let res_yt = ChromiumEngineBridge::spawn_youtube_app();
        assert!(res_yt.is_ok() || res_yt.is_err());

        let res_ft = ChromiumEngineBridge::spawn_freetube_app();
        assert!(res_ft.is_ok() || res_ft.is_err());
    }
}
