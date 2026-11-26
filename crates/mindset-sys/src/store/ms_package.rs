//! `.ms` Self-Contained Application Package Engine
//!
//! A `.ms` file is a ZIP archive containing:
//! - `manifest.ms.json` (metadata, SHA-256 digest, permissions, entrypoint)
//! - Optional `icon.svg` / `assets/` / `capsules/` / applet payload

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MsPackageKind {
    NativeApplet,
    SyllabusPack,
    WebStudyApp,
    LinuxBinary,
}

impl MsPackageKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::NativeApplet => "Native Applet",
            Self::SyllabusPack => "Syllabus Pack",
            Self::WebStudyApp => "Web Study App",
            Self::LinuxBinary => "Linux App",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub summary: String,
    pub category: String,
    pub kind: MsPackageKind,
    #[serde(default)]
    pub sha256: String,
    #[serde(default)]
    pub permissions: Vec<String>,
    pub entrypoint: String,
    #[serde(default)]
    pub default_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledMsApp {
    pub manifest: MsManifest,
    pub install_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreCatalogEntry {
    pub manifest: MsManifest,
    pub size_kb: u32,
    pub verified: bool,
}

pub struct MsPackageManager;

impl MsPackageManager {
    /// Default `~/.local/share/mindset-os/apps` directory.
    pub fn default_apps_root() -> PathBuf {
        dirs::data_local_dir()
            .unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")))
            .join("mindset-os")
            .join("apps")
    }

    /// Computes lowercase hex SHA-256 digest of arbitrary bytes.
    pub fn sha256_hex(bytes: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        format!("{:x}", hasher.finalize())
    }

    /// Verifies expected SHA-256 hex digest (if non-empty).
    pub fn verify_sha256(bytes: &[u8], expected_hex: &str) -> Result<(), String> {
        let exp = expected_hex.trim().to_lowercase();
        if exp.is_empty() {
            return Ok(());
        }
        let actual = Self::sha256_hex(bytes);
        if actual != exp {
            return Err(format!("SHA-256 mismatch: expected {exp}, got {actual}"));
        }
        Ok(())
    }

    /// Inspects `manifest.ms.json` inside a `.ms` ZIP archive without extracting it.
    pub fn inspect_ms_archive(archive_path: &Path) -> Result<MsManifest, String> {
        let file =
            File::open(archive_path).map_err(|e| format!("Failed to open .ms archive: {e}"))?;
        let mut archive =
            ZipArchive::new(file).map_err(|e| format!("Invalid .ms archive format: {e}"))?;
        let mut manifest_entry = archive
            .by_name("manifest.ms.json")
            .map_err(|_| "Missing manifest.ms.json inside .ms package".to_string())?;
        let mut json_str = String::new();
        manifest_entry
            .read_to_string(&mut json_str)
            .map_err(|e| format!("Failed to read manifest.ms.json: {e}"))?;
        let manifest: MsManifest = serde_json::from_str(&json_str)
            .map_err(|e| format!("Invalid manifest.ms.json: {e}"))?;
        if manifest.id.trim().is_empty()
            || manifest.id.contains("..")
            || manifest.id.contains('/')
            || manifest.id.contains('\\')
        {
            return Err("Invalid package id in manifest.ms.json".to_string());
        }
        Ok(manifest)
    }

    /// Safely extracts a `.ms` archive into `apps_root/<id>/` with zip-slip protection.
    pub fn install_ms_archive(
        archive_path: &Path,
        apps_root: &Path,
        expected_sha256: Option<&str>,
    ) -> Result<InstalledMsApp, String> {
        let raw_bytes =
            fs::read(archive_path).map_err(|e| format!("Failed to read .ms archive: {e}"))?;
        if let Some(expected) = expected_sha256 {
            Self::verify_sha256(&raw_bytes, expected)?;
        }

        let manifest = Self::inspect_ms_archive(archive_path)?;
        let install_dir = crate::fs::explorer::resolve_safe_path(apps_root, Some(&manifest.id))?;
        if install_dir.exists() {
            let _ = fs::remove_dir_all(&install_dir);
        }
        fs::create_dir_all(&install_dir)
            .map_err(|e| format!("Failed to create app install directory: {e}"))?;

        let file =
            File::open(archive_path).map_err(|e| format!("Failed to reopen .ms archive: {e}"))?;
        let mut archive =
            ZipArchive::new(file).map_err(|e| format!("Invalid .ms archive format: {e}"))?;

        for idx in 0..archive.len() {
            let mut entry = archive
                .by_index(idx)
                .map_err(|e| format!("Failed reading .ms entry: {e}"))?;
            let entry_name = entry.name().to_string();
            let dest = crate::fs::explorer::resolve_safe_path(&install_dir, Some(&entry_name))?;
            if entry.is_dir() {
                fs::create_dir_all(&dest)
                    .map_err(|e| format!("Failed creating dir '{}': {e}", dest.display()))?;
            } else {
                if let Some(parent) = dest.parent() {
                    fs::create_dir_all(parent)
                        .map_err(|e| format!("Failed creating parent dir: {e}"))?;
                }
                let mut out = File::create(&dest)
                    .map_err(|e| format!("Failed creating file '{}': {e}", dest.display()))?;
                std::io::copy(&mut entry, &mut out)
                    .map_err(|e| format!("Failed extracting '{}': {e}", entry_name))?;
            }
        }

        #[cfg(unix)]
        if manifest.kind == MsPackageKind::LinuxBinary {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(entry_path) =
                crate::fs::explorer::resolve_safe_path(&install_dir, Some(&manifest.entrypoint))
            {
                if entry_path.is_file() {
                    if let Ok(meta) = fs::metadata(&entry_path) {
                        let mut perms = meta.permissions();
                        perms.set_mode(0o755);
                        let _ = fs::set_permissions(&entry_path, perms);
                    }
                }
            }
        }

        Ok(InstalledMsApp {
            manifest,
            install_dir,
        })
    }

    /// Removes an installed `.ms` application directory cleanly.
    pub fn uninstall_app(app_id: &str, apps_root: &Path) -> Result<(), String> {
        let dir = crate::fs::explorer::resolve_safe_path(apps_root, Some(app_id))?;
        if dir.exists() {
            fs::remove_dir_all(&dir)
                .map_err(|e| format!("Failed removing '{}': {e}", dir.display()))?;
        }
        Ok(())
    }

    /// Creates a valid `.ms` archive on disk (used by built-in catalog & tests).
    pub fn build_ms_archive(
        output_path: &Path,
        manifest: &MsManifest,
        payload_name: &str,
        payload_bytes: &[u8],
    ) -> Result<String, String> {
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let file = File::create(output_path).map_err(|e| e.to_string())?;
        let mut zip = ZipWriter::new(file);
        let opts = SimpleFileOptions::default();

        let manifest_json = serde_json::to_string_pretty(manifest).map_err(|e| e.to_string())?;
        zip.start_file("manifest.ms.json", opts)
            .map_err(|e| e.to_string())?;
        zip.write_all(manifest_json.as_bytes())
            .map_err(|e| e.to_string())?;

        zip.start_file(payload_name, opts)
            .map_err(|e| e.to_string())?;
        zip.write_all(payload_bytes).map_err(|e| e.to_string())?;
        zip.finish().map_err(|e| e.to_string())?;

        let raw = fs::read(output_path).map_err(|e| e.to_string())?;
        Ok(Self::sha256_hex(&raw))
    }

    /// Built-in curated MindStore catalog of `.ms` study apps & packs.
    pub fn builtin_catalog() -> Vec<StoreCatalogEntry> {
        vec![
            StoreCatalogEntry {
                manifest: MsManifest {
                    id: "tn-class12-formulas".into(),
                    name: "TN Class 12 Formula Deck".into(),
                    version: "1.1.0".into(),
                    author: "L.K.S Education".into(),
                    summary: "Complete offline formula sheets for Physics, Chemistry, and Maths (590/600 target).".into(),
                    category: "Syllabus Packs".into(),
                    kind: MsPackageKind::SyllabusPack,
                    sha256: String::new(),
                    permissions: vec!["Read/Write Study Vault".into()],
                    entrypoint: "formulas.json".into(),
                    default_url: None,
                },
                size_kb: 420,
                verified: true,
            },
            StoreCatalogEntry {
                manifest: MsManifest {
                    id: "periodic-table-studio".into(),
                    name: "Periodic Table Studio".into(),
                    version: "1.1.0".into(),
                    author: "L.K.S STEM Lab".into(),
                    summary: "Interactive element explorer with electron shells, trends, and NEET/JEE quick facts.".into(),
                    category: "STEM Tools".into(),
                    kind: MsPackageKind::NativeApplet,
                    sha256: String::new(),
                    permissions: vec!["Offline Storage".into()],
                    entrypoint: "applet.json".into(),
                    default_url: None,
                },
                size_kb: 280,
                verified: true,
            },
            StoreCatalogEntry {
                manifest: MsManifest {
                    id: "geogebra-study".into(),
                    name: "GeoGebra Graphing Suite".into(),
                    version: "1.1.0".into(),
                    author: "GeoGebra / L.K.S".into(),
                    summary: "Distraction-free calculus, 3D vectors, and geometry plotter inside MS OS.".into(),
                    category: "STEM Tools".into(),
                    kind: MsPackageKind::WebStudyApp,
                    sha256: String::new(),
                    permissions: vec!["Academic Network Access".into()],
                    entrypoint: "launch.json".into(),
                    default_url: Some("https://www.geogebra.org/calculator".into()),
                },
                size_kb: 64,
                verified: true,
            },
            StoreCatalogEntry {
                manifest: MsManifest {
                    id: "wikipedia-study-hub".into(),
                    name: "Wikipedia Reference Hub".into(),
                    version: "1.1.0".into(),
                    author: "Wikimedia / L.K.S".into(),
                    summary: "Fast encyclopedia reference wrapper for UPSC, TNPSC, and science research.".into(),
                    category: "Focus Utilities".into(),
                    kind: MsPackageKind::WebStudyApp,
                    sha256: String::new(),
                    permissions: vec!["Academic Network Access".into()],
                    entrypoint: "launch.json".into(),
                    default_url: Some("https://en.wikipedia.org".into()),
                },
                size_kb: 48,
                verified: true,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ms_archive_build_verify_install_and_uninstall() {
        let tmp = std::env::temp_dir().join(format!("mindset_ms_pkg_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).expect("create tmp");

        let archive_path = tmp.join("periodic-table-studio.ms");
        let apps_root = tmp.join("apps");
        let manifest = MsManifest {
            id: "periodic-table-studio".into(),
            name: "Periodic Table Studio".into(),
            version: "1.1.0".into(),
            author: "L.K.S".into(),
            summary: "Test applet".into(),
            category: "STEM Tools".into(),
            kind: MsPackageKind::NativeApplet,
            sha256: String::new(),
            permissions: vec!["Offline Storage".into()],
            entrypoint: "applet.json".into(),
            default_url: None,
        };

        let digest = MsPackageManager::build_ms_archive(
            &archive_path,
            &manifest,
            "applet.json",
            b"{\"ok\":true}",
        )
        .expect("build .ms");
        assert_eq!(digest.len(), 64);

        // Tamper digest must fail
        assert!(MsPackageManager::install_ms_archive(
            &archive_path,
            &apps_root,
            Some("0000000000000000000000000000000000000000000000000000000000000000")
        )
        .is_err());

        // Valid digest installs cleanly
        let installed =
            MsPackageManager::install_ms_archive(&archive_path, &apps_root, Some(&digest))
                .expect("install .ms");
        assert!(installed.install_dir.join("manifest.ms.json").is_file());
        assert!(installed.install_dir.join("applet.json").is_file());

        MsPackageManager::uninstall_app("periodic-table-studio", &apps_root).expect("uninstall");
        assert!(!installed.install_dir.exists());
        let _ = fs::remove_dir_all(&tmp);
    }
}
