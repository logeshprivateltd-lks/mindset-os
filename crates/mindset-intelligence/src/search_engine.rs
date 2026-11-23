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
}
