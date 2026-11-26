//! MindSet OS MindStore & `.ms` Package Subsystem
//!
//! Parses, SHA-256 verifies, and installs self-contained `.ms` application archives.

pub mod ms_package;

pub use ms_package::{
    InstalledMsApp, MsManifest, MsPackageKind, MsPackageManager, StoreCatalogEntry,
};
