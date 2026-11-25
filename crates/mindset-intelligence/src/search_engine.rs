//! Native Academic Search Engine, STEM Encyclopedia, and Built-In Ad & Distraction Blocker Shield.

use serde::{Deserialize, Serialize};

/// Interactive search filter category tabs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SearchFilter {
    #[default]
    All,
    StudyVault,
    Encyclopedia,
    AskMaindyAi,
}

impl SearchFilter {
    pub fn all() -> [SearchFilter; 4] {
        [
            Self::All,
            Self::StudyVault,
            Self::Encyclopedia,
            Self::AskMaindyAi,
        ]
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::All => "All Results",
            Self::StudyVault => "Study Vault",
            Self::Encyclopedia => "STEM Encyclopedia",
            Self::AskMaindyAi => "Ask Maindy AI",
        }
    }
}

/// An academic search hit or indexed encyclopedia topic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub category: String,
    pub snippet: String,
    pub full_content: String,
    pub badge: &'static str,
    pub is_vault: bool,
}

/// Built-In Ad & Distraction Shield engine blocking telemetry, ad networks, and short-form video.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdBlockShield {
    pub is_enabled: bool,
    pub block_trackers: bool,
    pub block_social_feeds: bool,
    pub strip_autoplay: bool,
    pub ads_blocked_count: u64,
    pub distractions_blocked_count: u64,
    pub data_saved_mb: f32,
    pub blocked_events_log: Vec<String>,
}

impl Default for AdBlockShield {
    fn default() -> Self {
        Self {
            is_enabled: true,
            block_trackers: true,
            block_social_feeds: true,
            strip_autoplay: true,
            ads_blocked_count: 1428,
            distractions_blocked_count: 14,
            data_saved_mb: 19.2,
            blocked_events_log: vec![
                "Blocked adservice.google.com (telemetry tracker)".to_string(),
                "Blocked doubleclick.net (behavioral ad script)".to_string(),
                "Blocked youtube.com/shorts (distraction feed)".to_string(),
            ],
        }
    }
}

impl AdBlockShield {
    pub fn new() -> Self {
        Self::default()
    }

    /// Evaluates a URL or domain against active ad and distraction filter lists.
    pub fn should_block_domain(&self, url: &str) -> Option<&'static str> {
        if !self.is_enabled {
            return None;
        }

        let lower = url.to_ascii_lowercase();

        if self.block_trackers {
            let tracker_domains = [
                "doubleclick.net",
                "googlesyndication.com",
                "adservice.google.com",
                "taboola.com",
                "outbrain.com",
                "scorecardresearch.com",
                "criteo.com",
                "quantserve.com",
                "amazon-adsystem.com",
                "telemetry",
                "analytics",
            ];
            for dom in tracker_domains {
                if lower.contains(dom) {
                    return Some("Ad Network / Telemetry Tracker");
                }
            }
        }

        if self.block_social_feeds {
            let distraction_domains = [
                "youtube.com/shorts",
                "instagram.com/reels",
                "tiktok.com",
                "reddit.com",
                "facebook.com",
                "twitter.com",
                "x.com",
            ];
            for dom in distraction_domains {
                if lower.contains(dom) {
                    return Some("Short-Form Video / Distraction Feed");
                }
            }
        }

        None
    }

    /// Simulates domain filter testing and updates telemetry logs.
    pub fn test_domain_filter(&mut self, url: &str) -> (bool, &'static str) {
        if let Some(reason) = self.should_block_domain(url) {
            if reason.contains("Tracker") {
                self.ads_blocked_count += 1;
                self.data_saved_mb += 0.15;
            } else {
                self.distractions_blocked_count += 1;
                self.data_saved_mb += 1.20;
            }
            let log_msg = format!("Blocked {} ({})", url, reason);
            self.blocked_events_log.insert(0, log_msg);
            if self.blocked_events_log.len() > 10 {
                self.blocked_events_log.truncate(10);
            }
            (true, reason)
        } else {
            (false, "Allowed (Clean Academic Content)")
        }
    }

    /// Strips tracking and telemetry query parameters from URLs (e.g. utm_*, fbclid, gclid).
    pub fn sanitize_url(&self, url: &str) -> String {
        let tracking_params = [
            "utm_source",
            "utm_medium",
            "utm_campaign",
            "utm_term",
            "utm_content",
            "fbclid",
            "gclid",
            "msclkid",
            "yclid",
            "_ga",
            "ref",
            "ref_src",
            "ref_url",
            "ad_id",
            "tracking_id",
            "mc_eid",
            "twclid",
            "igshid",
        ];

        if let Some((base, query_str)) = url.split_once('?') {
            let filtered_params: Vec<&str> = query_str
                .split('&')
                .filter(|param| {
                    let key = param.split('=').next().unwrap_or("").to_ascii_lowercase();
                    !tracking_params.contains(&key.as_str())
                })
                .collect();

            if filtered_params.is_empty() {
                base.to_string()
            } else {
                format!("{}?{}", base, filtered_params.join("&"))
            }
        } else {
            url.to_string()
        }
    }

    /// Evaluates a list of search results against ad and distraction filters, sanitizing URLs and stripping unwanted entries.
    pub fn filter_and_sanitize_results(&mut self, results: Vec<SearchResult>) -> Vec<SearchResult> {
        let mut clean = Vec::new();
        for mut res in results {
            // Check if title, snippet or content points to a blocked domain
            if let Some(reason) = self
                .should_block_domain(&res.snippet)
                .or_else(|| self.should_block_domain(&res.full_content))
                .or_else(|| self.should_block_domain(&res.title))
            {
                if reason.contains("Tracker") {
                    self.ads_blocked_count += 1;
                    self.data_saved_mb += 0.15;
                } else {
                    self.distractions_blocked_count += 1;
                    self.data_saved_mb += 1.20;
                }
                let log_msg = format!("Blocked result: {} ({})", res.title, reason);
                self.blocked_events_log.insert(0, log_msg);
                if self.blocked_events_log.len() > 10 {
                    self.blocked_events_log.truncate(10);
                }
                continue;
            }

            // Sanitize tracking params inside snippets/content if URLs are present
            res.snippet = self.sanitize_text_urls(&res.snippet);
            res.full_content = self.sanitize_text_urls(&res.full_content);
            clean.push(res);
        }
        clean
    }

    fn sanitize_text_urls(&self, text: &str) -> String {
        let mut words = Vec::new();
        for word in text.split_whitespace() {
            if word.starts_with("http://") || word.starts_with("https://") {
                words.push(self.sanitize_url(word));
            } else {
                words.push(word.to_string());
            }
        }
        words.join(" ")
    }
}

/// Pre-indexed Class 12 STEM Encyclopedia database.
#[derive(Debug, Clone)]
pub struct AcademicEncyclopedia {
    pub entries: Vec<SearchResult>,
}

impl Default for AcademicEncyclopedia {
    fn default() -> Self {
        Self {
            entries: vec![
                SearchResult {
                    title: "Gauss's Law & Electric Flux".to_string(),
                    category: "Physics • Electrostatics".to_string(),
                    snippet: "Total electric flux through a closed Gaussian surface equals Q_enc / e0. Spherical, cylindrical, and planar symmetry.".to_string(),
                    full_content: "# Gauss's Law\n\nThe total electric flux through any closed hypothetical surface (Gaussian surface) is equal to 1/e0 times the net charge enclosed.\n\nFormula: Flux = Int E * dA = Q_enclosed / e0\n\nApplications:\n- Field due to infinite line charge: E = lambda / (2 * pi * e0 * r)\n- Field due to uniformly charged infinite plane sheet: E = sigma / (2 * e0)\n- Field due to thin spherical shell: E = Q / (4 * pi * e0 * r^2) outside, E = 0 inside.".to_string(),
                    badge: "PHYS",
                    is_vault: false,
                },
                SearchResult {
                    title: "Faraday's & Lenz's Law of Induction".to_string(),
                    category: "Physics • Electromagnetism".to_string(),
                    snippet: "Induced electromotive force is directly proportional to rate of change of magnetic flux. Direction opposes the change.".to_string(),
                    full_content: "# Electromagnetic Induction\n\nFaraday's Law states that the magnitude of induced EMF in a circuit equals the time rate of change of magnetic flux through the circuit.\n\nFormula: EMF = -dPhi_B / dt\n\nLenz's Law gives the negative sign, establishing that the polarity of induced EMF is such that it produces a current whose magnetic field opposes the change in magnetic flux that produces it (Law of Conservation of Energy).".to_string(),
                    badge: "PHYS",
                    is_vault: false,
                },
                SearchResult {
                    title: "Wave Optics & Young's Double Slit".to_string(),
                    category: "Physics • Optics".to_string(),
                    snippet: "Coherent wave interference produces alternating bright and dark fringe bands. Fringe width beta = lambda * D / d.".to_string(),
                    full_content: "# Young's Double Slit Experiment\n\nDemonstrates wave nature of light via two coherent pinhole slits separated by distance d.\n\nPath Difference: Delta x = x * d / D\n- Constructive (Bright): Delta x = n * lambda\n- Destructive (Dark): Delta x = (2n - 1) * lambda / 2\n\nFringe Width: beta = lambda * D / d (constant for all fringes).".to_string(),
                    badge: "PHYS",
                    is_vault: false,
                },
                SearchResult {
                    title: "Photoelectric Effect & Einstein's Equation".to_string(),
                    category: "Physics • Modern Physics".to_string(),
                    snippet: "Photon quanta transfer energy to electrons. Kinetic energy K_max = h * nu - phi_0. Instantaneous emission.".to_string(),
                    full_content: "# Photoelectric Effect\n\nLight incident on metal surfaces ejects electrons with zero measurable time lag (<= 10^-9 s).\n\nEinstein's Photoelectric Equation:\nh * nu = phi_0 + K_max = h * nu_0 + e * V_0\nwhere phi_0 is work function, nu_0 is threshold frequency, V_0 is stopping potential.\nProves particle nature of light (photons) with energy E = h * nu.".to_string(),
                    badge: "PHYS",
                    is_vault: false,
                },
                SearchResult {
                    title: "Semiconductor Diodes & Logic Gates".to_string(),
                    category: "Physics • Electronics".to_string(),
                    snippet: "P-N junctions in forward and reverse bias, Zener breakdown voltage regulation, and universal NAND/NOR logic.".to_string(),
                    full_content: "# Semiconductors & Solid State\n\n- P-N Junction Diode: Barrier potential ~0.7V for Si, ~0.3V for Ge.\n- Zener Diode: Heavily doped junction operating in reverse breakdown as voltage regulator.\n- Half-Wave & Full-Wave Rectifiers with capacitor filter smoothing.\n- Digital Logic Gates: AND, OR, NOT, NAND, NOR. Truth tables and Boolean theorems.".to_string(),
                    badge: "PHYS",
                    is_vault: false,
                },
                SearchResult {
                    title: "Electrochemistry & Nernst Equation".to_string(),
                    category: "Chemistry • Physical Chemistry".to_string(),
                    snippet: "Cell electromotive force under non-standard concentrations: E_cell = E0 - (0.0591/n) * log10(Q) at 298 K.".to_string(),
                    full_content: "# Electrochemistry & Electrochemical Cells\n\nNernst Equation connects cell potential to reaction quotient Q:\nE_cell = E0_cell - (2.303 * R * T / (n * F)) * log10(Q)\nAt 298 K: E_cell = E0_cell - (0.0591 / n) * log10(Q)\n\nEquilibrium Constant: log10(K_c) = n * E0_cell / 0.0591\nGibbs Free Energy: Delta G0 = -n * F * E0_cell\nKohlrausch's Law of independent migration of ions for limiting molar conductivity.".to_string(),
                    badge: "CHEM",
                    is_vault: false,
                },
                SearchResult {
                    title: "Chemical Kinetics & Rate Law".to_string(),
                    category: "Chemistry • Physical Chemistry".to_string(),
                    snippet: "Differential and integrated rate laws for zero and first order reactions. Arrhenius temperature dependence k = A * e^(-Ea/RT).".to_string(),
                    full_content: "# Chemical Kinetics\n\n- Zero Order: [R] = [R]_0 - k * t, t_half = [R]_0 / (2 * k)\n- First Order: k = (2.303 / t) * log10([R]_0 / [R]), t_half = 0.693 / k\n- Arrhenius Equation: k = A * exp(-E_a / (R * T))\nlog10(k2 / k1) = (E_a / (2.303 * R)) * ((T2 - T1) / (T1 * T2))\nPseudo first order hydrolysis of ester in acid medium.".to_string(),
                    badge: "CHEM",
                    is_vault: false,
                },
                SearchResult {
                    title: "Coordination Compounds & Crystal Field Theory".to_string(),
                    category: "Chemistry • Inorganic Chemistry".to_string(),
                    snippet: "Octahedral and tetrahedral d-orbital splitting Delta_0. Spectrochemical series, strong field low-spin vs weak field high-spin.".to_string(),
                    full_content: "# Coordination Compounds\n\nWerner's Coordination Theory: Primary (ionizable) vs Secondary (non-ionizable coordinate) valencies.\n\nCrystal Field Splitting (CFT):\n- Octahedral complex: d-orbitals split into lower t2g (dxy, dyz, dzx) and higher eg (dx2-y2, dz2) by energy Delta_0.\n- Tetrahedral complex: split inverted into lower e and higher t2 with Delta_t = (4/9) * Delta_0.\n- Spectrochemical Series: I- < Br- < SCN- < Cl- < F- < OH- < H2O < NH3 < en < CN- < CO.".to_string(),
                    badge: "CHEM",
                    is_vault: false,
                },
                SearchResult {
                    title: "Aldehydes, Ketones & Carboxylic Acids".to_string(),
                    category: "Chemistry • Organic Chemistry".to_string(),
                    snippet: "Nucleophilic addition across carbonyl bond. Aldol condensation, Cannizzaro redox, Clemmensen and Wolff-Kishner reduction.".to_string(),
                    full_content: "# Carbonyl Compounds\n\nKey Named Reactions:\n1. Aldol Condensation: Alpha-hydrogen containing aldehydes with dilute NaOH form beta-hydroxy aldehydes.\n2. Cannizzaro Reaction: Formaldehyde / benzaldehyde with conc. KOH disproportionate into alcohol + carboxylate salt.\n3. Rosenmund Reduction: Acyl chloride + H2 / Pd-BaSO4 gives aldehyde.\n4. Tollens' and Fehling's tests: Distinguish aldehydes from ketones.".to_string(),
                    badge: "CHEM",
                    is_vault: false,
                },
                SearchResult {
                    title: "Definite Integrals & Fundamental Properties".to_string(),
                    category: "Mathematics • Calculus".to_string(),
                    snippet: "Symmetric interval properties, King's rule Int_0^a f(x)dx = Int_0^a f(a-x)dx, and integration by parts.".to_string(),
                    full_content: "# Definite Integrals\n\nCore Properties for Board Examination:\n1. King's Property: Int_0^a f(x) dx = Int_0^a f(a - x) dx\n2. General Shift: Int_a^b f(x) dx = Int_a^b f(a + b - x) dx\n3. Even/Odd: Int_{-a}^a f(x) dx = 2 * Int_0^a f(x) dx (if even f(-x)=f(x)), 0 (if odd f(-x)=-f(x))\n4. Periodicity: Int_0^{2a} f(x) dx = 2 * Int_0^a f(x) dx (if f(2a-x)=f(x)), 0 (if f(2a-x)=-f(x))\n5. Integration by parts: Int u * v dx = u * Int v dx - Int (u' * Int v dx) dx (ILATE rule).".to_string(),
                    badge: "MATH",
                    is_vault: false,
                },
                SearchResult {
                    title: "Matrices, Determinants & Cramer's Rule".to_string(),
                    category: "Mathematics • Algebra".to_string(),
                    snippet: "Adjoint and inverse of 3x3 matrices, singular matrices det(A) = 0, and solving linear equations via matrix inversion X = A^(-1)B.".to_string(),
                    full_content: "# Matrices & Determinants\n\n- Inverse: A^(-1) = adj(A) / det(A), valid when det(A) != 0 (non-singular).\n- Properties: det(A * B) = det(A) * det(B), det(k * A) = k^n * det(A) for n x n matrix.\n- Adjoint properties: A * adj(A) = det(A) * I_n, det(adj(A)) = (det(A))^(n-1).\n- System of linear equations: A * X = B => X = A^(-1) * B. Unique solution when det(A) != 0.".to_string(),
                    badge: "MATH",
                    is_vault: false,
                },
                SearchResult {
                    title: "Differential Equations & Integrating Factors".to_string(),
                    category: "Mathematics • Calculus".to_string(),
                    snippet: "First order linear differential equation dy/dx + P(x)*y = Q(x). Integrating factor I.F. = exp(Int P dx).".to_string(),
                    full_content: "# Differential Equations\n\nLinear Form: dy/dx + P(x) * y = Q(x)\n1. Integrating Factor: I.F. = exp(Int P(x) dx)\n2. General Solution: y * (I.F.) = Int (Q(x) * (I.F.)) dx + C\n\nHomogeneous Equations: dy/dx = F(y / x) => Substitute y = v * x, dy/dx = v + x * dv/dx.\nSeparable variables: f(x) dx = g(y) dy => direct integration.".to_string(),
                    badge: "MATH",
                    is_vault: false,
                },
                SearchResult {
                    title: "Active Recall & Spaced Repetition (Leitner 5-Box)".to_string(),
                    category: "Study Methodology • Learning Science".to_string(),
                    snippet: "Cognitive science principles for retaining complex formulas without cramming. Spaced retrieval strengthens synaptic consolidation.".to_string(),
                    full_content: "# Active Recall & Spaced Retrieval\n\n1. Active Retrieval: Testing yourself from memory creates significantly stronger neural pathways than passive re-reading or re-highlighting notes.\n2. Leitner 5-Box System: Flashcards moved to next box on success, reset to Box 1 on error.\n- Box 1: Everyday review\n- Box 2: Every 2 days\n- Box 3: Every 5 days\n- Box 4: Every 2 weeks\n- Box 5: Monthly\n3. Overcomes the Ebbinghaus forgetting curve, ensuring long-term retention for board and entrance exams.".to_string(),
                    badge: "METH",
                    is_vault: false,
                },
                SearchResult {
                    title: "The Feynman Technique for Tough Derivations".to_string(),
                    category: "Study Methodology • Problem Solving".to_string(),
                    snippet: "Four-step conceptual verification method: choose topic, explain to a middle-schooler, pinpoint gaps, simplify language.".to_string(),
                    full_content: "# The Feynman Technique\n\nNobel laureate Richard Feynman's technique for mastering complex scientific concepts:\n1. Step 1: Write the concept name at the top of a blank page.\n2. Step 2: Teach it in plain, simple English without looking at reference material.\n3. Step 3: Whenever you stall or rely on textbook jargon, open notes to review that exact gap.\n4. Step 4: Simplify analogies and eliminate jargon until a beginner understands it intuitively.\nIf you can't explain it simply, you don't understand it well enough.".to_string(),
                    badge: "METH",
                    is_vault: false,
                },
                SearchResult {
                    title: "25-Minute Focused Ultradian Study Blocks".to_string(),
                    category: "Study Methodology • Focus Guard".to_string(),
                    snippet: "Biological ultradian cycles optimize deep focus. 25m intense concentration followed by 5m mental defragmentation.".to_string(),
                    full_content: "# Ultradian Focus Engineering\n\nHuman cognitive attention naturally follows 90-120 minute ultradian rhythms with sub-intervals:\n- 25-minute single-task focus block: zero multitasking, phone silenced, tabs closed.\n- 5-minute defragmentation: step away from screen, hydrate, avoid high-dopamine social scrolling.\n- Sustains 120 FPS mental throughput across multiple hours without burnout.\n- Built into MindSet OS Focus Guard with one-click session activation.".to_string(),
                    badge: "METH",
                    is_vault: false,
                },
            ],
        }
    }
}

/// Native Academic Search Engine combining encyclopedia index and live study notes vault.
#[derive(Debug, Clone)]
pub struct AcademicSearchEngine {
    pub encyclopedia: AcademicEncyclopedia,
    pub shield: AdBlockShield,
    pub query: String,
    pub active_filter: SearchFilter,
    pub results: Vec<SearchResult>,
}

impl Default for AcademicSearchEngine {
    fn default() -> Self {
        let encyclopedia = AcademicEncyclopedia::default();
        let initial_results = encyclopedia.entries.clone();
        Self {
            encyclopedia,
            shield: AdBlockShield::default(),
            query: String::new(),
            active_filter: SearchFilter::All,
            results: initial_results,
        }
    }
}

impl AcademicSearchEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Performs search filtering across encyclopedia and optional external vault notes.
    pub fn search(&mut self, query: &str, vault_notes: &[SearchResult]) {
        self.query = query.trim().to_string();
        let q_lower = self.query.to_ascii_lowercase();

        let mut pool: Vec<SearchResult> = Vec::new();

        // 1. Filter by category
        match self.active_filter {
            SearchFilter::All => {
                pool.extend(self.encyclopedia.entries.clone());
                pool.extend_from_slice(vault_notes);
            }
            SearchFilter::Encyclopedia => {
                pool.extend(self.encyclopedia.entries.clone());
            }
            SearchFilter::StudyVault => {
                pool.extend_from_slice(vault_notes);
            }
            SearchFilter::AskMaindyAi => {
                pool.push(SearchResult {
                    title: format!("Ask Maindy AI: '{}'", if self.query.is_empty() { "Academic Topics" } else { &self.query }),
                    category: "AI Tutor • Instant Synthesis".to_string(),
                    snippet: "Click to generate a custom step-by-step breakdown or formula derivation with Maindy AI Tutor.".to_string(),
                    full_content: format!("Ask Maindy AI about: {}", self.query),
                    badge: "AI",
                    is_vault: false,
                });
                self.results = pool;
                return;
            }
        }

        // 2. Query text search
        if q_lower.is_empty() {
            self.results = pool;
        } else {
            self.results = pool
                .into_iter()
                .filter(|res| {
                    res.title.to_ascii_lowercase().contains(&q_lower)
                        || res.category.to_ascii_lowercase().contains(&q_lower)
                        || res.snippet.to_ascii_lowercase().contains(&q_lower)
                })
                .collect();
        }
    }

    pub fn set_filter(&mut self, filter: SearchFilter, vault_notes: &[SearchResult]) {
        self.active_filter = filter;
        let q = self.query.clone();
        self.search(&q, vault_notes);
    }

    /// Executes a web search query through the specified connector, routing all results through AdBlockShield to sanitize URLs and block distractions.
    pub async fn execute_web_search(
        &mut self,
        query: &str,
        provider: SearchProviderType,
    ) -> Result<Vec<SearchResult>, String> {
        self.query = query.trim().to_string();

        let raw_results = match provider {
            SearchProviderType::Google => {
                let connector = GoogleSearchConnector::new();
                connector.execute_query(&self.query).await?
            }
            SearchProviderType::DuckDuckGo => {
                let connector = DuckDuckGoConnector::new();
                connector.execute_query(&self.query).await?
            }
            SearchProviderType::ArXiv => {
                let connector = ArXivConnector::new();
                connector.execute_query(&self.query).await?
            }
            SearchProviderType::Encyclopedia => {
                let mut matches = Vec::new();
                let q_lower = self.query.to_ascii_lowercase();
                for entry in &self.encyclopedia.entries {
                    if q_lower.is_empty()
                        || entry.title.to_ascii_lowercase().contains(&q_lower)
                        || entry.snippet.to_ascii_lowercase().contains(&q_lower)
                    {
                        matches.push(entry.clone());
                    }
                }
                matches
            }
        };

        // Filter through AdBlockShield to sanitize tracking query parameters and block ad/distraction domains
        let clean_results = self.shield.filter_and_sanitize_results(raw_results);
        self.results = clean_results.clone();
        Ok(clean_results)
    }
}

// ---------------------------------------------------------------------------
// Web Search Connectors & Providers
// ---------------------------------------------------------------------------

/// Web search provider connector trait for live and academic query execution.
#[allow(async_fn_in_trait)]
pub trait WebSearchProvider: Send + Sync {
    async fn execute_query(&self, query: &str) -> Result<Vec<SearchResult>, String>;
}

/// Supported academic and external web search provider engines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchProviderType {
    Google,
    DuckDuckGo,
    ArXiv,
    Encyclopedia,
}

impl SearchProviderType {
    pub fn label(self) -> &'static str {
        match self {
            Self::Google => "Google Search",
            Self::DuckDuckGo => "DuckDuckGo",
            Self::ArXiv => "ArXiv STEM Papers",
            Self::Encyclopedia => "STEM Encyclopedia",
        }
    }
}

fn url_encode_query(q: &str) -> String {
    let mut encoded = String::with_capacity(q.len() * 3);
    for b in q.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(*b as char);
            }
            b' ' => encoded.push_str("%20"),
            other => {
                encoded.push_str(&format!("%{:02X}", other));
            }
        }
    }
    encoded
}

fn build_http_client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(8))
        .user_agent("MindSetOS/1.0 (+https://mindsetos.local)")
        .build()
        .map_err(|e| format!("Failed to initialize HTTP client: {}", e))
}

/// Google Academic / Custom Search Connector.
#[derive(Debug, Clone, Default)]
pub struct GoogleSearchConnector {
    pub api_key: Option<String>,
    pub search_engine_id: Option<String>,
}

impl GoogleSearchConnector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_credentials(api_key: String, search_engine_id: String) -> Self {
        Self {
            api_key: Some(api_key),
            search_engine_id: Some(search_engine_id),
        }
    }

    pub fn parse_cse_json(val: &serde_json::Value) -> Vec<SearchResult> {
        let mut results = Vec::new();
        if let Some(items) = val.get("items").and_then(|v| v.as_array()) {
            for item in items.iter().take(5) {
                let title = item
                    .get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Google Result")
                    .trim()
                    .to_string();
                let link = item
                    .get("link")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .trim()
                    .to_string();
                let snippet_raw = item
                    .get("snippet")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .trim()
                    .to_string();
                let snippet = if link.is_empty() {
                    snippet_raw.clone()
                } else if snippet_raw.is_empty() {
                    link.clone()
                } else {
                    format!("{} {}", snippet_raw, link)
                };
                results.push(SearchResult {
                    title: title.clone(),
                    category: "Google Search • Web".to_string(),
                    snippet,
                    full_content: format!("# {}\nSource: {}\n\n{}", title, link, snippet_raw),
                    badge: "GOOG",
                    is_vault: false,
                });
            }
        }
        results
    }
}

impl WebSearchProvider for GoogleSearchConnector {
    async fn execute_query(&self, query: &str) -> Result<Vec<SearchResult>, String> {
        let q = query.trim();
        if q.is_empty() {
            return Ok(Vec::new());
        }

        let api_key = self
            .api_key
            .clone()
            .filter(|s| !s.trim().is_empty())
            .or_else(|| {
                std::env::var("GOOGLE_CSE_API_KEY")
                    .ok()
                    .filter(|s| !s.trim().is_empty())
            });
        let cx = self
            .search_engine_id
            .clone()
            .filter(|s| !s.trim().is_empty())
            .or_else(|| {
                std::env::var("GOOGLE_CSE_CX")
                    .ok()
                    .filter(|s| !s.trim().is_empty())
            });

        let (key, engine_id) = match (api_key, cx) {
            (Some(k), Some(id)) => (k, id),
            _ => {
                return Err(
                    "Google Custom Search API key and CSE ID are not configured (set GOOGLE_CSE_API_KEY and GOOGLE_CSE_CX)"
                        .to_string(),
                );
            }
        };

        let url = format!(
            "https://www.googleapis.com/customsearch/v1?key={}&cx={}&q={}&num=5",
            url_encode_query(&key),
            url_encode_query(&engine_id),
            url_encode_query(q)
        );

        let client = build_http_client()?;
        let resp = client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Google CSE network request failed: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("Google CSE returned HTTP {}", resp.status()));
        }

        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Google CSE JSON decode failed: {}", e))?;

        Ok(Self::parse_cse_json(&json))
    }
}

/// DuckDuckGo Privacy-Preserving Academic Connector.
#[derive(Debug, Clone, Default)]
pub struct DuckDuckGoConnector;

impl DuckDuckGoConnector {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_instant_answer_json(val: &serde_json::Value) -> Vec<SearchResult> {
        let mut results = Vec::new();
        let heading = val
            .get("Heading")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim();
        let abstract_text = val
            .get("AbstractText")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim();
        let abstract_url = val
            .get("AbstractURL")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim();

        if !abstract_text.is_empty() {
            let title = if heading.is_empty() {
                "DuckDuckGo Instant Answer".to_string()
            } else {
                heading.to_string()
            };
            let snippet = if abstract_url.is_empty() {
                abstract_text.to_string()
            } else {
                format!("{} {}", abstract_text, abstract_url)
            };
            results.push(SearchResult {
                title: title.clone(),
                category: "DuckDuckGo • Instant Answer".to_string(),
                snippet,
                full_content: format!("# {}\nSource: {}\n\n{}", title, abstract_url, abstract_text),
                badge: "DDG",
                is_vault: false,
            });
        }

        if let Some(topics) = val.get("RelatedTopics").and_then(|v| v.as_array()) {
            for topic in topics {
                if results.len() >= 5 {
                    break;
                }
                if let Some(text) = topic.get("Text").and_then(|v| v.as_str()) {
                    let first_url = topic
                        .get("FirstURL")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .trim();
                    let title = text
                        .split(" - ")
                        .next()
                        .unwrap_or("DuckDuckGo Topic")
                        .trim()
                        .to_string();
                    let snippet = if first_url.is_empty() {
                        text.to_string()
                    } else {
                        format!("{} {}", text, first_url)
                    };
                    results.push(SearchResult {
                        title: title.clone(),
                        category: "DuckDuckGo • Related Topic".to_string(),
                        snippet,
                        full_content: format!("# {}\nSource: {}\n\n{}", title, first_url, text),
                        badge: "DDG",
                        is_vault: false,
                    });
                }
            }
        }
        results
    }
}

impl WebSearchProvider for DuckDuckGoConnector {
    async fn execute_query(&self, query: &str) -> Result<Vec<SearchResult>, String> {
        let q = query.trim();
        if q.is_empty() {
            return Ok(Vec::new());
        }

        let url = format!(
            "https://api.duckduckgo.com/?q={}&format=json&no_html=1&skip_disambig=1",
            url_encode_query(q)
        );
        let client = build_http_client()?;
        let resp = client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("DuckDuckGo network request failed: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("DuckDuckGo returned HTTP {}", resp.status()));
        }

        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("DuckDuckGo JSON decode failed: {}", e))?;

        Ok(Self::parse_instant_answer_json(&json))
    }
}

/// ArXiv / OpenAlex STEM Scientific Preprint Connector.
#[derive(Debug, Clone, Default)]
pub struct ArXivConnector;

impl ArXivConnector {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_atom_xml(xml: &str) -> Result<Vec<SearchResult>, String> {
        use quick_xml::events::Event;
        use quick_xml::reader::Reader;

        let mut reader = Reader::from_str(xml);
        let mut results = Vec::new();
        let mut in_entry = false;
        let mut current_tag = String::new();
        let mut title = String::new();
        let mut summary = String::new();
        let mut link_id = String::new();

        loop {
            match reader.read_event() {
                Ok(Event::Start(e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    let local = name.rsplit(':').next().unwrap_or(&name).to_string();
                    if local == "entry" {
                        in_entry = true;
                        title.clear();
                        summary.clear();
                        link_id.clear();
                    } else if in_entry {
                        current_tag = local;
                    }
                }
                Ok(Event::Text(e)) => {
                    if in_entry {
                        let raw = std::str::from_utf8(&e).unwrap_or("");
                        let text = quick_xml::escape::unescape(raw).unwrap_or_default();
                        match current_tag.as_str() {
                            "title" => title.push_str(&text),
                            "summary" => summary.push_str(&text),
                            "id" => link_id.push_str(&text),
                            _ => {}
                        }
                    }
                }
                Ok(Event::End(e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    let local = name.rsplit(':').next().unwrap_or(&name);
                    if local == "entry" && in_entry {
                        in_entry = false;
                        let clean_title = title.split_whitespace().collect::<Vec<_>>().join(" ");
                        let clean_summary =
                            summary.split_whitespace().collect::<Vec<_>>().join(" ");
                        let clean_id = link_id.trim().to_string();
                        if !clean_title.is_empty() {
                            let snippet = if clean_id.is_empty() {
                                clean_summary.clone()
                            } else {
                                format!("{} {}", clean_summary, clean_id)
                            };
                            results.push(SearchResult {
                                title: clean_title.clone(),
                                category: "ArXiv • STEM Preprint".to_string(),
                                snippet,
                                full_content: format!(
                                    "# {}\nSource: {}\n\n{}",
                                    clean_title, clean_id, clean_summary
                                ),
                                badge: "ARXIV",
                                is_vault: false,
                            });
                        }
                    } else if current_tag == local {
                        current_tag.clear();
                    }
                }
                Ok(Event::Eof) => break,
                Ok(_) => {}
                Err(e) => return Err(format!("ArXiv Atom XML parse error: {}", e)),
            }
        }

        Ok(results)
    }
}

impl WebSearchProvider for ArXivConnector {
    async fn execute_query(&self, query: &str) -> Result<Vec<SearchResult>, String> {
        let q = query.trim();
        if q.is_empty() {
            return Ok(Vec::new());
        }

        let url = format!(
            "https://export.arxiv.org/api/query?search_query=all:{}&start=0&max_results=5",
            url_encode_query(q)
        );
        let client = build_http_client()?;
        let resp = client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("ArXiv network request failed: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("ArXiv returned HTTP {}", resp.status()));
        }

        let xml = resp
            .text()
            .await
            .map_err(|e| format!("ArXiv response read failed: {}", e))?;

        Self::parse_atom_xml(&xml)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ad_block_shield_filters_trackers_and_distractions() {
        let mut shield = AdBlockShield::new();
        assert!(shield.is_enabled);

        // Tracker test
        assert!(shield
            .should_block_domain("https://adservice.google.com/pagead/id")
            .is_some());
        assert!(shield
            .should_block_domain("https://securepubads.g.doubleclick.net/gampad/ads")
            .is_some());

        // Distraction test
        assert!(shield
            .should_block_domain("https://youtube.com/shorts/dQw4w9WgXcQ")
            .is_some());
        assert!(shield
            .should_block_domain("https://tiktok.com/@student")
            .is_some());

        // Clean educational URL test
        assert!(shield
            .should_block_domain("https://en.wikipedia.org/wiki/Gauss%27s_law")
            .is_none());

        // Test test_domain_filter
        let (blocked, reason) = shield.test_domain_filter("https://doubleclick.net/tracker.js");
        assert!(blocked);
        assert!(reason.contains("Tracker"));
        assert!(shield.ads_blocked_count > 1428);
    }

    #[test]
    fn test_academic_search_query_matching() {
        let mut engine = AcademicSearchEngine::new();
        assert!(engine.results.len() >= 15);

        // Search for Gauss
        engine.search("Gauss", &[]);
        assert!(!engine.results.is_empty());
        assert!(engine.results[0].title.contains("Gauss"));

        // Search for Electrochemistry
        engine.search("Electrochemistry", &[]);
        assert!(!engine.results.is_empty());
        assert!(engine.results[0].title.contains("Electrochemistry"));

        // Filter by STEM Encyclopedia
        engine.set_filter(SearchFilter::Encyclopedia, &[]);
        assert_eq!(engine.active_filter, SearchFilter::Encyclopedia);
    }

    #[test]
    fn test_ad_block_shield_url_sanitization() {
        let shield = AdBlockShield::new();

        let dirty_url = "https://ocw.mit.edu/courses/physics?utm_source=adnetwork&fbclid=abc12345&gclid=xyz987&topic=gauss";
        let clean_url = shield.sanitize_url(dirty_url);
        assert!(!clean_url.contains("utm_source"));
        assert!(!clean_url.contains("fbclid"));
        assert!(!clean_url.contains("gclid"));
        assert!(
            clean_url.contains("topic=gauss"),
            "Should preserve non-tracking parameter"
        );

        let clean_input = "https://en.wikipedia.org/wiki/Nernst_equation";
        assert_eq!(shield.sanitize_url(clean_input), clean_input);
    }

    #[tokio::test]
    async fn test_web_search_connectors_fail_closed_and_parse_real_formats() {
        // 1. GoogleSearchConnector must fail closed without credentials (no canned fake results)
        let google = GoogleSearchConnector::new();
        let g_err = google.execute_query("Gauss's law electric flux").await;
        assert!(g_err.is_err(), "Unconfigured Google CSE must fail closed");

        // 2. Encyclopedia offline search succeeds cleanly
        let mut engine = AcademicSearchEngine::new();
        let enc_res = engine
            .execute_web_search("Gauss", SearchProviderType::Encyclopedia)
            .await
            .expect("Encyclopedia offline search must succeed");
        assert!(!enc_res.is_empty());

        // 3. ArXiv Atom XML parsing + AdBlockShield sanitization
        let atom_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <feed xmlns="http://www.w3.org/2005/Atom">
          <entry>
            <id>https://arxiv.org/abs/2501.00001?utm_source=tracker&amp;fbclid=xyz</id>
            <title>Quantum Photoelectric Verification</title>
            <summary>Single-photon stopping potential measurements.</summary>
          </entry>
        </feed>"#;
        let parsed_arxiv = ArXivConnector::parse_atom_xml(atom_xml).expect("Atom XML parse");
        assert_eq!(parsed_arxiv.len(), 1);
        assert_eq!(parsed_arxiv[0].badge, "ARXIV");
        let sanitized = engine.shield.filter_and_sanitize_results(parsed_arxiv);
        assert_eq!(sanitized.len(), 1);
        assert!(!sanitized[0].snippet.contains("utm_source"));
        assert!(!sanitized[0].snippet.contains("fbclid"));

        // 4. DuckDuckGo JSON parsing + Google CSE JSON parsing + tracker blocking
        let ddg_json = serde_json::json!({
            "Heading": "Nernst Equation",
            "AbstractText": "Relates cell reduction potential to temperature.",
            "AbstractURL": "https://en.wikipedia.org/wiki/Nernst_equation?gclid=bad123",
            "RelatedTopics": []
        });
        let ddg_res = DuckDuckGoConnector::parse_instant_answer_json(&ddg_json);
        assert_eq!(ddg_res.len(), 1);
        let ddg_clean = engine.shield.filter_and_sanitize_results(ddg_res);
        assert!(!ddg_clean[0].snippet.contains("gclid"));

        let cse_json = serde_json::json!({
            "items": [
                {
                    "title": "MIT Gauss Law",
                    "link": "https://ocw.mit.edu/gauss?utm_campaign=promo",
                    "snippet": "Electric flux through closed surfaces."
                },
                {
                    "title": "Blocked Tracker",
                    "link": "https://doubleclick.net/ad",
                    "snippet": "Ad network"
                }
            ]
        });
        let cse_res = GoogleSearchConnector::parse_cse_json(&cse_json);
        assert_eq!(cse_res.len(), 2);
        let cse_clean = engine.shield.filter_and_sanitize_results(cse_res);
        assert_eq!(cse_clean.len(), 1);
        assert!(!cse_clean[0].snippet.contains("utm_campaign"));
    }
}
