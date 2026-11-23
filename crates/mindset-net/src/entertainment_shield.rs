//! MindSet OS — Entertainment & Distraction Shield Subsystem
//! Category-based blocking for Streaming/OTT, Social Media feeds, Web Games,
//! and messaging apps during focus sessions, with an unbreakable Academic Whitelist.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use serde::{Deserialize, Serialize};

/// High-yield academic and reference platforms that are ALWAYS permitted.
pub const ACADEMIC_WHITELIST: &[&str] = &[
    "wikipedia.org",
    "khanacademy.org",
    "nptel.ac.in",
    "arxiv.org",
    "github.com",
    "doc.rust-lang.org",
    "crates.io",
    "ncert.nic.in",
    "mit.edu",
    "stanford.edu",
    "jstor.org",
    "nature.com",
    "sciencedirect.com",
    "ieee.org",
    "stackoverflow.com",
];

/// OTT & video entertainment streaming platforms.
pub const OTT_STREAMING_DOMAINS: &[&str] = &[
    "netflix.com",
    "primevideo.com",
    "hotstar.com",
    "disneyplus.com",
    "twitch.tv",
    "kick.com",
    "crunchyroll.com",
    "sonyliv.com",
    "hulu.com",
    "max.com",
    "zee5.com",
    "voot.com",
];

/// Social networks and infinite short-video feed applications.
pub const SOCIAL_MEDIA_DOMAINS: &[&str] = &[
    "instagram.com",
    "tiktok.com",
    "facebook.com",
    "x.com",
    "twitter.com",
    "reddit.com",
    "snapchat.com",
    "pinterest.com",
    "threads.net",
    "tumblr.com",
    "bsky.app",
];

/// Web gaming portals, cloud gaming, and distribution stores.
pub const GAMING_DOMAINS: &[&str] = &[
    "poki.com",
    "crazygames.com",
    "roblox.com",
    "krunker.io",
    "itch.io",
    "store.steampowered.com",
    "epicgames.com",
    "miniclip.com",
    "chess.com",
    "lichess.org",
    "now.gg",
];

/// Instant messaging and real-time chat platforms.
pub const CHAT_DOMAINS: &[&str] = &[
    "web.whatsapp.com",
    "discord.com",
    "web.telegram.org",
    "slack.com",
    "teams.microsoft.com",
];

/// Entertainment and web-app shield coordinator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntertainmentShield {
    pub block_ott: bool,
    pub block_social: bool,
    pub block_games: bool,
    pub block_chat_during_focus: bool,
    pub custom_blocked_domains: Vec<String>,
    pub blocked_attempts: u64,
}

impl Default for EntertainmentShield {
    fn default() -> Self {
        Self {
            block_ott: true,
            block_social: true,
            block_games: true,
            block_chat_during_focus: true,
            custom_blocked_domains: Vec::new(),
            blocked_attempts: 0,
        }
    }
}

impl EntertainmentShield {
    pub fn new() -> Self {
        Self::default()
    }

    /// Evaluates whether a URL is in the academic and research whitelist.
    pub fn is_academic_whitelisted(url: &str) -> bool {
        let lower = url.to_ascii_lowercase();
        ACADEMIC_WHITELIST
            .iter()
            .any(|&academic| lower.contains(academic))
    }

    /// Appends a user-defined custom blocked domain.
    pub fn add_custom_domain(&mut self, domain: impl Into<String>) {
        let d = domain.into().trim().to_ascii_lowercase();
        if !d.is_empty() && !self.custom_blocked_domains.contains(&d) {
            self.custom_blocked_domains.push(d);
        }
    }

    /// Removes a user-defined custom blocked domain.
    pub fn remove_custom_domain(&mut self, domain: &str) -> bool {
        let d = domain.trim().to_ascii_lowercase();
        if let Some(pos) = self.custom_blocked_domains.iter().position(|x| x == &d) {
            self.custom_blocked_domains.remove(pos);
            true
        } else {
            false
        }
    }

    /// Inspects a navigation URL against category blocking rules and focus mode state.
    /// Academic platforms always pass through unblocked.
    pub fn check_url(&mut self, url: &str, focus_active: bool) -> Result<(), String> {
        let lower = url.trim().to_ascii_lowercase();

        // 1. Academic whitelist takes absolute priority
        if Self::is_academic_whitelisted(&lower) {
            return Ok(());
        }

        // 2. Custom user-defined domain blacklist
        for custom in &self.custom_blocked_domains {
            if lower.contains(custom) {
                self.blocked_attempts = self.blocked_attempts.saturating_add(1);
                return Err(format!(
                    "Access to '{custom}' is blocked by user security policy."
                ));
            }
        }

        // 3. OTT & Streaming Platforms
        if self.block_ott {
            for &ott in OTT_STREAMING_DOMAINS {
                if lower.contains(ott) {
                    self.blocked_attempts = self.blocked_attempts.saturating_add(1);
                    return Err(format!(
                        "Access to Streaming & OTT service '{ott}' is restricted by MindSet Integrity Policy."
                    ));
                }
            }
        }

        // 4. Social Media & Infinite Short Feeds
        if self.block_social {
            for &social in SOCIAL_MEDIA_DOMAINS {
                if lower.contains(social) {
                    self.blocked_attempts = self.blocked_attempts.saturating_add(1);
                    return Err(format!(
                        "Access to Social Media network '{social}' is restricted by MindSet Integrity Policy."
                    ));
                }
            }
        }

        // 5. Web Games & Gaming Portals
        if self.block_games {
            for &game in GAMING_DOMAINS {
                if lower.contains(game) {
                    self.blocked_attempts = self.blocked_attempts.saturating_add(1);
                    return Err(format!(
                        "Access to Gaming platform '{game}' is restricted by MindSet Integrity Policy."
                    ));
                }
            }
        }

        // 6. Messaging & Chat Platforms during Focus sessions
        if focus_active && self.block_chat_during_focus {
            for &chat in CHAT_DOMAINS {
                if lower.contains(chat) {
                    self.blocked_attempts = self.blocked_attempts.saturating_add(1);
                    return Err(format!(
                        "Messaging & Chat application '{chat}' is restricted during Mindful Focus sessions."
                    ));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entertainment_shield_blocks_categories() {
        let mut shield = EntertainmentShield::new();

        // OTT
        assert!(shield
            .check_url("https://www.netflix.com/browse", false)
            .is_err());
        assert!(shield
            .check_url("https://hotstar.com/movies", false)
            .is_err());
        assert!(shield
            .check_url("https://twitch.tv/streamer", false)
            .is_err());

        // Social Media
        assert!(shield
            .check_url("https://instagram.com/p/12345", false)
            .is_err());
        assert!(shield.check_url("https://reddit.com/r/all", false).is_err());
        assert!(shield
            .check_url("https://tiktok.com/@creator", false)
            .is_err());

        // Web Games
        assert!(shield
            .check_url("https://poki.com/en/g/subway-surfers", false)
            .is_err());
        assert!(shield.check_url("https://crazygames.com", false).is_err());
        assert!(shield
            .check_url("https://store.steampowered.com", false)
            .is_err());
    }

    #[test]
    fn test_academic_whitelist_always_allowed() {
        let mut shield = EntertainmentShield::new();
        // Even if custom blocked, whitelist wins
        shield.add_custom_domain("wikipedia.org");

        assert!(shield
            .check_url("https://en.wikipedia.org/wiki/Calculus", true)
            .is_ok());
        assert!(shield
            .check_url("https://www.khanacademy.org/math", true)
            .is_ok());
        assert!(shield
            .check_url("https://arxiv.org/pdf/2301.0001", true)
            .is_ok());
        assert!(shield
            .check_url("https://github.com/rust-lang/rust", true)
            .is_ok());
        assert!(shield
            .check_url("https://doc.rust-lang.org/book/", true)
            .is_ok());
    }

    #[test]
    fn test_chat_blocking_only_during_focus() {
        let mut shield = EntertainmentShield::new();

        // Allowed when focus is NOT active
        assert!(shield.check_url("https://web.whatsapp.com", false).is_ok());
        assert!(shield.check_url("https://discord.com/app", false).is_ok());

        // Blocked when focus IS active
        assert!(shield.check_url("https://web.whatsapp.com", true).is_err());
        assert!(shield
            .check_url("https://discord.com/channels/123", true)
            .is_err());
    }

    #[test]
    fn test_custom_domain_addition_and_removal() {
        let mut shield = EntertainmentShield::new();
        shield.add_custom_domain("distracting-blog.io");

        assert!(shield
            .check_url("https://distracting-blog.io/post", false)
            .is_err());
        assert!(shield.remove_custom_domain("distracting-blog.io"));
        assert!(shield
            .check_url("https://distracting-blog.io/post", false)
            .is_ok());
    }
}
