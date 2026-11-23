//! MindSet OS — Ad & Telemetry Network Blocker Subsystem
//! Fast in-memory domain/pattern matcher blocking ad servers, trackers, telemetry,
//! and generates Chromium DeclarativeNetRequest JSON rulesets.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use serde::{Deserialize, Serialize};

/// High-frequency ad networks, telemetry trackers, and behavioral analytics domains.
pub const AD_TRACKER_DOMAINS: &[&str] = &[
    "doubleclick.net",
    "googlesyndication.com",
    "adservice.google.com",
    "google-analytics.com",
    "taboola.com",
    "outbrain.com",
    "criteo.com",
    "criteo.net",
    "ads.pubmatic.com",
    "scorecardresearch.com",
    "adnxs.com",
    "rubiconproject.com",
    "advertising.com",
    "amazon-adsystem.com",
    "quantserve.com",
    "moatads.com",
    "popads.net",
    "adroll.com",
    "casale-media.com",
    "smartadserver.com",
    "openx.net",
];

/// Chromium DeclarativeNetRequest Rule representation (Manifest V3 format).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclarativeRule {
    pub id: u32,
    pub priority: u32,
    pub action: RuleAction,
    pub condition: RuleCondition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleAction {
    #[serde(rename = "type")]
    pub action_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    #[serde(rename = "urlFilter")]
    pub url_filter: String,
    #[serde(rename = "resourceTypes")]
    pub resource_types: Vec<String>,
}

/// Real-time adblock filter statistics.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdBlockStats {
    pub ads_blocked_count: u64,
    pub video_ads_skipped: u64,
    pub estimated_kb_saved: u64,
}

/// Core ad & tracking request interceptor.
#[derive(Debug, Clone, Default)]
pub struct AdBlockFilter {
    pub enabled: bool,
    pub stats: AdBlockStats,
    pub custom_filters: Vec<String>,
}

impl AdBlockFilter {
    pub fn new() -> Self {
        Self {
            enabled: true,
            stats: AdBlockStats::default(),
            custom_filters: Vec::new(),
        }
    }

    /// Evaluates whether a URL or host should be blocked as an advertisement or tracker.
    pub fn is_ad_or_tracker(&self, url: &str) -> bool {
        if !self.enabled {
            return false;
        }

        let lower = url.to_ascii_lowercase();

        // 1. Check known ad networks and telemetry servers
        for &domain in AD_TRACKER_DOMAINS {
            if lower.contains(domain) {
                return true;
            }
        }

        // 2. Check custom user-defined filters
        for custom in &self.custom_filters {
            if lower.contains(custom) {
                return true;
            }
        }

        false
    }

    /// Intercepts a network request. If it matches an ad or tracker, records telemetry
    /// and returns `true` (block the request). Otherwise returns `false`.
    pub fn should_block_request(&mut self, url: &str) -> bool {
        if self.is_ad_or_tracker(url) {
            self.stats.ads_blocked_count = self.stats.ads_blocked_count.saturating_add(1);

            let lower = url.to_ascii_lowercase();
            if lower.contains("video") || lower.contains("stream") || lower.contains("vmap") {
                self.stats.video_ads_skipped = self.stats.video_ads_skipped.saturating_add(1);
                // Video ads typically consume 1.2 MB on average
                self.stats.estimated_kb_saved = self.stats.estimated_kb_saved.saturating_add(1200);
            } else {
                // Static banner / tracker payload average: ~48 KB
                self.stats.estimated_kb_saved = self.stats.estimated_kb_saved.saturating_add(48);
            }
            true
        } else {
            false
        }
    }

    /// Appends a custom domain or substring filter.
    pub fn add_custom_filter(&mut self, filter: impl Into<String>) {
        let f = filter.into().to_ascii_lowercase();
        if !self.custom_filters.contains(&f) {
            self.custom_filters.push(f);
        }
    }

    /// Builds a valid JSON array of Chromium DeclarativeNetRequest rules.
    pub fn build_declarative_net_request_rules(&self) -> String {
        let mut rules = Vec::new();
        let mut id = 1u32;

        let resource_types = vec![
            "main_frame".to_string(),
            "sub_frame".to_string(),
            "script".to_string(),
            "image".to_string(),
            "xmlhttprequest".to_string(),
            "media".to_string(),
        ];

        for &domain in AD_TRACKER_DOMAINS {
            rules.push(DeclarativeRule {
                id,
                priority: 1,
                action: RuleAction {
                    action_type: "block".to_string(),
                },
                condition: RuleCondition {
                    url_filter: format!("||{domain}^"),
                    resource_types: resource_types.clone(),
                },
            });
            id += 1;
        }

        for custom in &self.custom_filters {
            rules.push(DeclarativeRule {
                id,
                priority: 1,
                action: RuleAction {
                    action_type: "block".to_string(),
                },
                condition: RuleCondition {
                    url_filter: format!("||{custom}^"),
                    resource_types: resource_types.clone(),
                },
            });
            id += 1;
        }

        serde_json::to_string_pretty(&rules).unwrap_or_else(|_| "[]".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adblock_filter_domain_matching() {
        let mut filter = AdBlockFilter::new();

        assert!(filter.should_block_request("https://securepubads.g.doubleclick.net/gampad/ads"));
        assert!(filter.should_block_request(
            "https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js"
        ));
        assert!(filter.should_block_request("https://www.google-analytics.com/analytics.js"));
        assert!(filter.should_block_request("https://api.taboola.com/v1/recommendations"));
        assert!(filter.should_block_request("https://static.criteo.net/js/ld/ld.js"));

        // Allowed academic & search URLs
        assert!(!filter.should_block_request("https://en.wikipedia.org/wiki/Special:Search"));
        assert!(!filter.should_block_request("https://khanacademy.org/math/algebra"));
        assert!(!filter.should_block_request("https://docs.rs/wgpu/latest/wgpu/"));
    }

    #[test]
    fn test_adblock_stats_telemetry_tracking() {
        let mut filter = AdBlockFilter::new();
        assert_eq!(filter.stats.ads_blocked_count, 0);

        filter.should_block_request("https://adservice.google.com/adsid/integrator.js");
        assert_eq!(filter.stats.ads_blocked_count, 1);
        assert_eq!(filter.stats.estimated_kb_saved, 48);

        // Video ad
        filter.should_block_request("https://doubleclick.net/video/ad_stream.mp4");
        assert_eq!(filter.stats.ads_blocked_count, 2);
        assert_eq!(filter.stats.video_ads_skipped, 1);
        assert_eq!(filter.stats.estimated_kb_saved, 48 + 1200);
    }

    #[test]
    fn test_build_declarative_net_request_rules_json_validity() {
        let mut filter = AdBlockFilter::new();
        filter.add_custom_filter("spammytracker.xyz");

        let json = filter.build_declarative_net_request_rules();
        assert!(!json.is_empty());

        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed.is_array());
        let arr = parsed.as_array().unwrap();
        assert!(arr.len() > AD_TRACKER_DOMAINS.len());

        // Verify structure of first rule
        let first = &arr[0];
        assert_eq!(first["action"]["type"], "block");
        assert!(first["condition"]["urlFilter"]
            .as_str()
            .unwrap()
            .starts_with("||"));
    }
}
