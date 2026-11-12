# MINDSET OS — SPECIFICATION 05: MINDSET FOCUS GUARD & DIGITAL WELLBEING DAEMON
**Company:** LKS PVT LTD  
**Module:** MindSet Focus Guard (System-Level Digital Wellbeing & Distraction Prevention)  
**RAM Budget:** < 15 MB RSS (Native Rust daemon + SQLite policy store)

## 1. Six Core Distraction Categories (`packages/focus-policy-engine`)
Users can add, remove, and recategorise any domain or executable across 6 categories:
1. **A. Video Platforms:** YouTube, YouTube Shorts, Netflix, Prime Video, Twitch, short-form video portals.
2. **B. Social Media:** Instagram, Snapchat, TikTok, Facebook, X/Twitter, Threads.
3. **C. Discussion Platforms:** Reddit, distracting forums, unmoderated community boards.
4. **D. Gaming:** Steam, Epic Games Launcher, Riot Client, local game binaries, browser gaming portals.
5. **E. Communication:** Discord, non-essential messaging apps, promotional notification channels.
6. **F. Custom Distractions:** User-defined URLs, executables, and unproductive browsing categories.

## 2. Multi-Layer Enforcement Architecture (`services/mindset-focus-guard`)
Network filtering is one layer of protection—never the sole enforcement mechanism.
1. **Layer 1 — Network Guard (`nftables` + `Unbound DNS`):**
   - Generates local `Unbound` RPZ (Response Policy Zone) blocklists and `nftables` IP/port rules for blocked categories.
   - Detects and blocks unauthorized DoH (DNS-over-HTTPS) bypasses during Strict Mode while preserving allowlisted educational domains.
2. **Layer 2 — Granular YouTube Educational Filter (DOM & URL Interceptor):**
   - Since DNS cannot block sub-paths, the MindSet Educational Web Viewer (and browser extension bridge) enforces separate rules for:
     - **Blocked in Study Mode:** `/shorts/*` URLs, `ytd-rich-grid-renderer` (Homepage recommendations), Subscriptions feed, and comment sections.
     - **Allowed in Study Mode:** Direct educational video playback (`/watch?v=...` for lectures) and deliberate educational search queries.
3. **Layer 3 — Application & Gaming Guard (`/proc` & `D-Bus` Process Monitor):**
   - Listens to Linux process creation and desktop launch events (with Windows process table fallback for dev testing).
   - Intercepts restricted apps/games (Steam, Epic Games, Discord, Instagram) during active focus schedules, tracks gaming time budgets, and suspends background distraction processes.
4. **Layer 4 — Notification Guard (`org.freedesktop.Notifications` Interceptor):**
   - Silences social media and promotional notifications, hides distracting previews, batches non-urgent alerts into scheduled summary digests, and always allows VIP/family/emergency contacts.

## 3. Four Intelligent Focus Profiles & Three Protection Levels
### 3.1 Focus Profiles
- **Study Mode:** Blocks Social Media, YouTube Shorts/Home feed, Games, and Entertainment; allows Educational Web, PDF Tutor, and Study apps; silences non-essential notifications.
- **Deep Work Mode:** Blocks all entertainment and messaging; allows IDEs, MindSet Terminal, documentation sites, and local AI.
- **Sleep Mode (`22:00` Default):** Restricts entertainment apps, shifts UI to calm dim mode, silences all non-emergency notifications.
- **Custom Mode:** User-configured matrix of apps, domains, schedules, and notification rules.

### 3.2 Three Protection Levels
- **Gentle Mode:** Usage reminders, intelligent warnings, screen-time analytics, zero hard blocking.
- **Balanced Mode:** Scheduled website/app blocking, daily gaming/entertainment time budgets, controlled 5-minute exception requests.
- **Strict Mode:** System-level launch interception, `nftables`/DNS enforcement, cooldown timers on exception requests, and UAC admin verification. Always preserves emergency communication access and authorised recovery.

## 4. Non-Shaming Distraction Interruption UI & Cross-Device Focus Sync
1. **Focus Guard Interruption Screen:**
   - Displays a calm `#07090D` Liquid Glass overlay showing: *Focus Guard Active*, current profile name, tabular remaining time, current planned study task, and non-judgmental reason for restriction.
   - Actions: **"Return to Previous Application"**, **"Open Current Task"**, or **"Request Controlled Exception"** (logging the user's selected reason into the **Distraction Journal**).
2. **Cross-Device Focus Sync (`MindSet Connect`):**
   - Activating a Focus Session on Desktop immediately broadcasts the encrypted policy state via `sync_oplog.sqlite` / WebSockets to `mindset-mobile`, locking distracting mobile apps and syncing the countdown timer.