//! Zero-Failure Multilingual Offline Study Helper
//!
//! Provides instantaneous, high-yield academic assistance and formula retrieval
//! when neither Cloud API nor local Ollama is reachable.

use super::persona::{DualTextResult, MaindyStyle};
use mindset_intelligence::agent_tools::AgentAction;

pub struct OfflineStudyHelper;

impl OfflineStudyHelper {
    /// Generates an immediate, helpful response to a student query offline.
    pub fn respond(
        prompt: &str,
        style: &str,
        student_name: &str,
        target_track: &str,
    ) -> DualTextResult {
        let name = if student_name.trim().is_empty() { "Scholar" } else { student_name.trim() };
        let mode = MaindyStyle::from_str_code(style);
        let q = prompt.to_lowercase();

        // 1. Agent Tool Actions (Deterministic execution offline)
        let (raw_display, raw_speech) = if q.contains("create capsule")
            || q.contains("make capsule")
            || q.contains("new capsule")
            || q.contains("add capsule")
        {
            Self::create_capsule_action_response(mode, name, prompt)
        } else if q.contains("list folder") || q.contains("list files") || q.contains("show files") {
            Self::list_folder_action_response(mode, name, prompt)
        } else if q.contains("read document")
            || q.contains("read file")
            || q.contains("read formula")
            || q.contains("read note")
        {
            Self::read_document_action_response(mode, name, prompt)
        } else if q.contains("open app") || q.contains("launch app") {
            Self::open_app_action_response(mode, name, prompt)
        } else if q.contains("photoelectric") || q.contains("einstein") || q.contains("work function") {
            Self::photoelectric_response(mode, name)
        } else if q.contains("faraday") || q.contains("induction") || q.contains("lenz") {
            Self::faraday_response(mode, name)
        } else if q.contains("gauss") || q.contains("flux") {
            Self::gauss_response(mode, name)
        } else if q.contains("ohm") || q.contains("resistance") || q.contains("current") {
            Self::ohm_response(mode, name)
        } else if q.contains("newton") || q.contains("force") || q.contains("momentum") {
            Self::newton_response(mode, name)
        } else if q.contains("gas") || q.contains("ideal gas") || q.contains("pv=nrt") {
            Self::ideal_gas_response(mode, name)
        } else if q.contains("nernst") || q.contains("emf") || q.contains("electrochem") {
            Self::nernst_response(mode, name)
        } else if q.contains("quadratic") || q.contains("formula") && (q.contains("root") || q.contains("equation")) {
            Self::quadratic_response(mode, name)
        } else if q.contains("matrix") || q.contains("determinant") {
            Self::matrices_response(mode, name)
        } else if q.contains("hello") || q.contains("vanakkam") || q.contains("hi") || q.contains("hey") || q.contains("thala") || q.contains("pandriga") || q.contains("pandringa") {
            Self::greeting_response(mode, name, target_track)
        } else {
            Self::general_fallback(mode, name, target_track, prompt)
        };

        let lang = match mode {
            MaindyStyle::Thanglish | MaindyStyle::Tamil => "ta-IN",
            MaindyStyle::Malayalam => "ml-IN",
            _ => "en-IN",
        };

        DualTextResult {
            display_text: raw_display,
            speech_text: raw_speech,
            primary_lang: lang.to_string(),
        }
    }

    fn create_capsule_action_response(mode: MaindyStyle, name: &str, prompt: &str) -> (String, String) {
        let q = prompt.to_lowercase();
        let subject = if q.contains("math") || q.contains("calculus") || q.contains("algebra") {
            "Mathematics"
        } else if q.contains("chem") || q.contains("organic") || q.contains("reaction") {
            "Chemistry"
        } else if q.contains("bio") || q.contains("cell") || q.contains("botany") || q.contains("zoology") {
            "Biology"
        } else {
            "Physics"
        };

        // Extract topic if specified after "for", "on", "about", or default
        let topic = if let Some(pos) = q.find(" for ") {
            prompt[pos + 5..].trim()
        } else if let Some(pos) = q.find(" on ") {
            prompt[pos + 4..].trim()
        } else if let Some(pos) = q.find(" about ") {
            prompt[pos + 7..].trim()
        } else {
            "High-Yield Concept Mastery"
        };
        let topic_clean = if topic.is_empty() { "High-Yield Concept Mastery" } else { topic };
        let title = if topic_clean.len() > 50 {
            &topic_clean[..50]
        } else {
            topic_clean
        };

        let action = AgentAction::CreateAndShowCapsule {
            subject: subject.to_string(),
            topic: title.to_string(),
            duration_mins: 25,
            priority: "High".to_string(),
            notes_summary: format!(
                "Core syllabus capsule on {} ({}) synthesized by Maindy AI.",
                title, subject
            ),
        };

        let action_json = serde_json::to_string_pretty(&action).unwrap_or_default();

        match mode {
            MaindyStyle::Thanglish => (
                format!(
                    "Kandippa da {name}! I am creating a tailored Study Capsule on **{title}** ({subject}) right now:\n\n\
                    <os_action>\n{action_json}\n</os_action>\n\n\
                    🚀 Opening Study Session Reader and synchronizing with your 3D Odyssey trail!",
                    name = name,
                    title = title,
                    subject = subject,
                    action_json = action_json
                ),
                format!(
                    "கண்டிப்பா டா {name}! நான் இப்போதே {title} ஸ்டடி கேப்சூல் உருவாக்குகிறேன்.",
                    name = name,
                    title = title
                ),
            ),
            _ => (
                format!(
                    "Absolutely, {name}! I am generating a high-yield Study Capsule on **{title}** ({subject}) for you right now:\n\n\
                    <os_action>\n{action_json}\n</os_action>\n\n\
                    🚀 Launching the Study Session Reader and adding it to your 3D Odyssey Journey!",
                    name = name,
                    title = title,
                    subject = subject,
                    action_json = action_json
                ),
                format!(
                    "Certainly {name}! Creating your study capsule on {title} now.",
                    name = name,
                    title = title
                ),
            ),
        }
    }

    fn list_folder_action_response(mode: MaindyStyle, name: &str, prompt: &str) -> (String, String) {
        let q = prompt.to_lowercase();
        let folder = if q.contains("formula") {
            "Formula Sheets"
        } else if q.contains("capsule") {
            "Study Capsules"
        } else if q.contains("download") {
            "Downloads"
        } else {
            "Documents"
        };

        let action = AgentAction::ListFolder {
            path: folder.to_string(),
        };
        let action_json = serde_json::to_string_pretty(&action).unwrap_or_default();

        match mode {
            MaindyStyle::Thanglish => (
                format!(
                    "Sari da {name}! Un **{folder}** folder-ai inspect panna permission request anupuren. Please review the sheet on screen:\n\n\
                    <os_action>\n{action_json}\n</os_action>",
                    name = name,
                    folder = folder,
                    action_json = action_json
                ),
                format!(
                    "சரி டா {name}! உங்கள் {folder} போல்டரை பார்க்க அனுமதி கேட்கிறேன்.",
                    name = name,
                    folder = folder
                ),
            ),
            _ => (
                format!(
                    "I am ready to inspect your **{folder}** directory, {name}. Please authorize this action on the permission sheet:\n\n\
                    <os_action>\n{action_json}\n</os_action>",
                    name = name,
                    folder = folder,
                    action_json = action_json
                ),
                format!(
                    "Please authorize access so I can inspect your {folder} directory.",
                    folder = folder
                ),
            ),
        }
    }

    fn read_document_action_response(mode: MaindyStyle, name: &str, prompt: &str) -> (String, String) {
        let q = prompt.to_lowercase();
        let file = if q.contains("formula") {
            "Formula Sheets/Physics_Formula_Sheet.txt"
        } else if q.contains("readme") || q.contains("vault") {
            "Documents/README_STUDY_VAULT.txt"
        } else {
            "Documents/Quick_Study_Notes.txt"
        };

        let action = AgentAction::ReadDocument {
            file_path: file.to_string(),
        };
        let action_json = serde_json::to_string_pretty(&action).unwrap_or_default();

        match mode {
            MaindyStyle::Thanglish => (
                format!(
                    "Romba nalladhu da {name}! **{file}** document-ai read panni review panna un permission thevai:\n\n\
                    <os_action>\n{action_json}\n</os_action>",
                    name = name,
                    file = file,
                    action_json = action_json
                ),
                format!(
                    "ரொம்ப நல்லது டா {name}! ஆவணத்தைப் படிக்க அனுமதி தரவும்.",
                    name = name
                ),
            ),
            _ => (
                format!(
                    "I would like to read and summarize **{file}** for you, {name}. Please grant permission below to proceed:\n\n\
                    <os_action>\n{action_json}\n</os_action>",
                    name = name,
                    file = file,
                    action_json = action_json
                ),
                format!(
                    "Please grant permission to read your study document, {name}.",
                    name = name
                ),
            ),
        }
    }

    fn open_app_action_response(_mode: MaindyStyle, name: &str, prompt: &str) -> (String, String) {
        let q = prompt.to_lowercase();
        let app_id = if q.contains("vault") || q.contains("file") {
            "file_vault"
        } else if q.contains("resource") || q.contains("monitor") {
            "resource_monitor"
        } else if q.contains("pref") || q.contains("setting") {
            "preferences"
        } else if q.contains("reader") || q.contains("capsule") {
            "study_reader"
        } else if q.contains("timer") || q.contains("pomodoro") || q.contains("clock") {
            "pomodoro"
        } else {
            "file_vault"
        };

        let action = AgentAction::OpenApp {
            app_id: app_id.to_string(),
        };
        let action_json = serde_json::to_string_pretty(&action).unwrap_or_default();

        (
            format!(
                "Opening the **{}** app for you, {}!\n\n<os_action>\n{}\n</os_action>",
                app_id, name, action_json
            ),
            format!("Opening {} for you now.", app_id),
        )
    }

    fn photoelectric_response(mode: MaindyStyle, name: &str) -> (String, String) {
        match mode {
            MaindyStyle::Thanglish => (
                format!(
                    "Vanakkam da {name}! Photoelectric Effect romba high-yield topic:\n\n\
                    📌 Einstein's Equation: **hf = W + KE_max**\n\
                    • **h**: Planck's constant ($6.626 \\times 10^{{-34}}\\text{{ J}}\\cdot\\text{{s}}$)\n\
                    • **f**: Incident photon frequency ($\\nu$)\n\
                    • **W**: Work function ($W = hf_0$)\n\
                    • **KE_max**: Stopping potential energy ($e V_0$)\n\n\
                    💡 Key Exam Takeaway: Stopping potential $V_0$ frequency-ai depend pannum, intensity-ai depend pannaadhu!",
                    name = name
                ),
                format!(
                    "வணக்கம் டா {name}! Photoelectric Effect equation: h f equals W plus K E max. Stopping potential frequency-யை பொறுத்து இருக்கும், intensity-யை பொறுத்து இல்லை.",
                    name = name
                ),
            ),
            MaindyStyle::Tamil => (
                format!(
                    "வணக்கம் {name}! ஒளிமின் விளைவு (Photoelectric Effect) முக்கிய சமன்பாடு:\n\n\
                    📌 ஐன்ஸ்டீனின் சமன்பாடு: **hf = W + KE_max**\n\
                    • **hf**: விழும் ஃபோட்டானின் ஆற்றல்\n\
                    • **W**: வெளியேற்று ஆற்றல் (Work Function = $hf_0$)\n\
                    • **KE_max**: அதிகபட்ச இயக்க ஆற்றல் ($e V_0$)\n\n\
                    💡 தேர்வு குறிப்பு: நிறுத்து மின்னழுத்தம் ($V_0$) அதிர்வெண்ணைச் சார்ந்தது, ஒளிச் செறிவைச் சார்ந்ததல்ல!",
                    name = name
                ),
                format!(
                    "வணக்கம் {name}! ஒளிமின் விளைவு சமன்பாடு: h f equals W plus K E max. நிறுத்து மின்னழுத்தம் அதிர்வெண்ணைச் சார்ந்தது.",
                    name = name
                ),
            ),
            _ => (
                format!(
                    "Greetings {name}! Here is Einstein's Photoelectric Equation:\n\n\
                    📌 **hf = W + KE_max**\n\
                    • **h**: Planck's constant ($6.626 \\times 10^{{-34}}\\text{{ J}}\\cdot\\text{{s}}$)\n\
                    • **f**: Incident frequency ($\\nu$)\n\
                    • **W**: Work function ($hf_0$)\n\
                    • **KE_max**: Max kinetic energy of photoelectrons ($e V_0$)\n\n\
                    💡 Key Exam Fact: Stopping potential $V_0$ depends strictly on photon frequency, not beam intensity!",
                    name = name
                ),
                format!(
                    "Greetings {name}! Einstein's Photoelectric Equation: h f equals W plus K E max. Stopping potential depends on frequency, not intensity.",
                    name = name
                ),
            ),
        }
    }

    fn faraday_response(mode: MaindyStyle, name: &str) -> (String, String) {
        match mode {
            MaindyStyle::Thanglish => (
                format!(
                    "Faraday's Law of Electromagnetic Induction paaru {name}:\n\n\
                    📌 **\\mathcal{{E}} = -N \\frac{{d\\Phi_B}}{{dt}}**\n\
                    • **\\mathcal{{E}}**: Induced Electromotive Force (EMF in Volts)\n\
                    • **N**: Number of coil turns\n\
                    • **\\Phi_B**: Magnetic flux ($\\Phi_B = B \\cdot A \\cos\\theta$)\n\
                    • **Negative sign (-)**: Lenz's Law (induced EMF opposes the change causing it).",
                    name = name
                ),
                "Faraday's Law of Electromagnetic Induction: E equals negative N times d phi by d t. Negative sign Lenz's Law-ஐ குறிக்கும்."
                    .to_string(),
            ),
            _ => (
                "Faraday's Law of Electromagnetic Induction:\n\n\
                📌 **\\mathcal{E} = -N \\frac{d\\Phi_B}{dt}**\n\
                • Induced EMF equals the negative rate of change of magnetic flux.\n\
                • The minus sign represents Lenz's Law: induced currents oppose the change that produces them."
                    .to_string(),
                "Faraday's Law: Induced EMF equals negative N times rate of change of magnetic flux. Minus sign represents Lenz's law."
                    .to_string(),
            ),
        }
    }

    fn gauss_response(_mode: MaindyStyle, _name: &str) -> (String, String) {
        (
            "Gauss's Law in Electrostatics:\n\n\
            📌 **\\oint \\mathbf{E} \\cdot d\\mathbf{A} = \\frac{q_{\\text{enc}}}{\\varepsilon_0}**\n\
            • Total electric flux through any closed Gaussian surface equals the net enclosed charge divided by $\\varepsilon_0$."
                .to_string(),
            "Gauss's Law: The total electric flux through a closed surface equals charge enclosed divided by epsilon nought."
                .to_string(),
        )
    }

    fn ohm_response(mode: MaindyStyle, name: &str) -> (String, String) {
        match mode {
            MaindyStyle::Thanglish => (
                format!(
                    "Ohm's Law romba basic da {name}:\n\n\
                    📌 **V = I \\cdot R**\n\
                    • **V**: Potential Difference (Volts)\n\
                    • **I**: Current (Amperes)\n\
                    • **R**: Resistance (Ohms, where $R = \\rho \\frac{{L}}{{A}}$)",
                    name = name
                ),
                "Ohm's Law: V equals I times R. Voltage equals current times resistance.".to_string(),
            ),
            _ => (
                "Ohm's Law at constant temperature:\n\n\
                📌 **V = I \\cdot R**\n\
                • **V**: Voltage (V)\n\
                • **I**: Current (A)\n\
                • **R**: Resistance ($\\Omega$, $R = \\rho L / A$)"
                    .to_string(),
                "Ohm's Law states Voltage equals Current multiplied by Resistance.".to_string(),
            ),
        }
    }

    fn newton_response(_mode: MaindyStyle, _name: &str) -> (String, String) {
        (
            "Newton's Second Law of Motion:\n\n\
            📌 **\\mathbf{F} = \\frac{d\\mathbf{p}}{dt} = m\\mathbf{a}**\n\
            • Force equals rate of change of linear momentum ($p = mv$). When mass $m$ is constant, $F = ma$."
                .to_string(),
            "Newton's Second Law: Force equals mass times acceleration.".to_string(),
        )
    }

    fn ideal_gas_response(_mode: MaindyStyle, _name: &str) -> (String, String) {
        (
            "Ideal Gas Equation:\n\n\
            📌 **PV = nRT**\n\
            • **P**: Pressure (Pa / atm)\n\
            • **V**: Volume ($m^3$ / L)\n\
            • **n**: Moles ($n = m / M$)\n\
            • **R**: Universal gas constant ($8.314\\text{ J}/(\\text{mol}\\cdot\\text{K})$)\n\
            • **T**: Absolute temperature (Kelvin)"
                .to_string(),
            "Ideal Gas Law: P V equals n R T.".to_string(),
        )
    }

    fn nernst_response(_mode: MaindyStyle, _name: &str) -> (String, String) {
        (
            "Nernst Equation (Electrochemistry at $298\\text{ K}$):\n\n\
            📌 **E_{\\text{cell}} = E^\\circ_{\\text{cell}} - \\frac{0.0591}{n} \\log_{10} Q**\n\
            • **E**: Cell potential under non-standard conditions\n\
            • **E°**: Standard reduction potential\n\
            • **n**: Number of electrons transferred\n\
            • **Q**: Reaction quotient ([Products] / [Reactants])"
                .to_string(),
            "Nernst Equation: E cell equals E nought cell minus point zero five nine one over n log Q."
                .to_string(),
        )
    }

    fn quadratic_response(_mode: MaindyStyle, _name: &str) -> (String, String) {
        (
            "Quadratic Formula for $ax^2 + bx + c = 0$:\n\n\
            📌 **x = \\frac{-b \\pm \\sqrt{b^2 - 4ac}}{2a}**\n\
            • Discriminant $D = b^2 - 4ac$\n\
            • If $D > 0$: 2 real distinct roots\n\
            • If $D = 0$: 1 real repeated root\n\
            • If $D < 0$: 2 complex conjugate roots"
                .to_string(),
            "Quadratic formula: x equals negative b plus or minus square root of b squared minus four a c, all over two a."
                .to_string(),
        )
    }

    fn matrices_response(_mode: MaindyStyle, _name: &str) -> (String, String) {
        (
            "Matrices & Determinants Key Rules:\n\n\
            📌 Inverse: **A^{-1} = \\frac{1}{|A|} \\text{adj}(A)** (exists iff $|A| \\neq 0$)\n\
            📌 Properties:\n\
            • $|AB| = |A| \\cdot |B|$\n\
            • $|kA| = k^n |A|$ for an $n \\times n$ matrix\n\
            • $A \\cdot \\text{adj}(A) = |A| \\cdot I$"
                .to_string(),
            "Matrix inverse formula: A inverse equals one over determinant A times adjoint of A."
                .to_string(),
        )
    }

    fn greeting_response(mode: MaindyStyle, name: &str, track: &str) -> (String, String) {
        match mode {
            MaindyStyle::Thanglish => (
                format!(
                    "Vanakkam {name}! Semma-ya iruken da. Innaiku unga {track} goal-ku capsule start pannalaama?\n\n\
                    Enna topic ippo padikkalaam? Physics, Chemistry, or Maths formulas edhavadhu kekkanuma?\n\n\
                    💡 *Maindy Offline Assistant is active. To enable full cloud or local LLM conversations, paste an API key or launch Ollama in Mindset Preferences.*"
                ),
                format!("வணக்கம் {name}! செம்மையா இருக்கேன் டா. இன்னைக்கு உங்க இலக்குக்கு கேப்ஸ்யூல் ஸ்டார்ட் பண்ணலாமா?"),
            ),
            MaindyStyle::Tamil => (
                format!(
                    "வணக்கம் {name}! உங்கள் படிப்பு இலக்கு: {track}.\n\n\
                    எந்த பாடத்தை நாம் இப்போது ஆரம்பிக்கலாம்? இயற்பியல், வேதியியல் அல்லது கணித சூத்திரங்கள் குறித்து என்னிடம் கேளுங்கள்!\n\n\
                    💡 *Maindy ஆஃப்லைன் உதவியாளர் செயலில் உள்ளது.*"
                ),
                format!("வணக்கம் {name}! உங்கள் படிப்பு இலக்கை நோக்கி படிக்க ஆரம்பிக்கலாம்."),
            ),
            _ => (
                format!(
                    "Greetings {name}! Welcome to MindSet OS. Target Track: {track}.\n\n\
                    What subject are we mastering today? Ask me any Physics, Chemistry, or Math formula or exam concept!\n\n\
                    💡 *Tip: To unlock full open-ended chat, connect Ollama (`ollama run qwen2.5:1.5b`) or enter an API key in Preferences.*"
                ),
                format!("Greetings {name}! Welcome to MindSet OS. What subject are we mastering today?"),
            ),
        }
    }

    fn general_fallback(mode: MaindyStyle, name: &str, track: &str, prompt: &str) -> (String, String) {
        match mode {
            MaindyStyle::Thanglish => (
                format!(
                    "Super question da {name}!\n\n\
                    You asked: *\"{prompt}\"*\n\n\
                    Target Track: **{track}**\n\n\
                    To explore this deep concept with live step-by-step guidance:\n\
                    1. **Free Local AI:** Open your terminal and run `ollama run qwen2.5:1.5b` or `ollama serve`.\n\
                    2. **Free Cloud AI:** Open **Mindset Preferences -> Maindy AI & Keys** and paste a free Gemini or Groq API key.\n\n\
                    In the meantime, feel free to ask me for any high-yield formulas (Photoelectric effect, Faraday's law, Gauss's law, Ideal gas, Quadratic formula, etc.)!",
                    name = name,
                    prompt = prompt,
                    track = track
                ),
                format!(
                    "சூப்பர் கேள்வி {name}! நீங்கள் கேட்டது: {prompt}. மேலும் விளக்கங்களுக்கு Ollama அல்லது API சாவியை இணைக்கவும்.",
                    name = name,
                    prompt = prompt
                ),
            ),
            _ => (
                format!(
                    "Great question, {name}!\n\n\
                    Regarding: *\"{prompt}\"*\n\n\
                    Target Track: **{track}**\n\n\
                    To get full step-by-step interactive tutoring on this topic:\n\
                    1. **Local AI (Free & Offline):** Launch `ollama serve` or run `ollama run qwen2.5:1.5b` in terminal.\n\
                    2. **Cloud AI (BYOK):** Enter a free Google Gemini or Groq API key in **Mindset Preferences -> Maindy AI & Keys**.\n\n\
                    You can also ask me anytime for exact formulas across Physics, Chemistry, and Mathematics!",
                    name = name,
                    prompt = prompt,
                    track = track
                ),
                format!(
                    "Great question {name}! Connect Ollama or enter a Cloud API key in Preferences for full interactive chat.",
                    name = name
                ),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_photoelectric_response_thanglish() {
        let res = OfflineStudyHelper::respond(
            "explain photoelectric effect",
            "thanglish",
            "Karthik",
            "TN Class 12",
        );
        assert!(res.display_text.contains("hf = W + KE"));
        assert!(res.speech_text.contains("வணக்கம்"));
        assert_eq!(res.primary_lang, "ta-IN");
    }

    #[test]
    fn test_formula_keyword_matching() {
        let res1 = OfflineStudyHelper::respond("Faraday induction law", "en", "Student", "JEE");
        assert!(res1.display_text.contains("d\\Phi_B"));

        let res2 = OfflineStudyHelper::respond("ideal gas equation", "en", "Student", "NEET");
        assert!(res2.display_text.contains("PV = nRT"));
    }

    #[test]
    fn test_offline_agent_actions() {
        use mindset_intelligence::agent_tools::extract_agent_action;

        // 1. Create Capsule
        let res_cap = OfflineStudyHelper::respond(
            "create capsule for Optics & Lenses",
            "en",
            "Karthik",
            "JEE",
        );
        let (action_opt, cleaned) = extract_agent_action(&res_cap.display_text);
        assert!(action_opt.is_some());
        match action_opt.unwrap() {
            AgentAction::CreateAndShowCapsule { topic, subject, .. } => {
                assert!(topic.contains("Optics"));
                assert_eq!(subject, "Physics");
            }
            _ => panic!("Expected CreateAndShowCapsule"),
        }
        assert!(!cleaned.contains("<os_action>"));

        // 2. List Folder
        let res_list = OfflineStudyHelper::respond(
            "list files in Formula Sheets",
            "en",
            "Karthik",
            "JEE",
        );
        let (action_opt2, _) = extract_agent_action(&res_list.display_text);
        assert!(action_opt2.is_some());
        match action_opt2.unwrap() {
            AgentAction::ListFolder { path } => {
                assert_eq!(path, "Formula Sheets");
            }
            _ => panic!("Expected ListFolder"),
        }

        // 3. Read Document
        let res_read = OfflineStudyHelper::respond(
            "read formula sheet for Physics",
            "en",
            "Karthik",
            "JEE",
        );
        let (action_opt3, _) = extract_agent_action(&res_read.display_text);
        assert!(action_opt3.is_some());
        match action_opt3.unwrap() {
            AgentAction::ReadDocument { file_path } => {
                assert!(file_path.contains("Physics_Formula_Sheet.txt"));
            }
            _ => panic!("Expected ReadDocument"),
        }
    }
}
