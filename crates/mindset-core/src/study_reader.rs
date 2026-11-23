//! MindSet OS — Study Session Reader Dataset & State (`study_reader.rs`)
//! Synchronized STEM topic dataset for high-yield board/entrance examination mastery.
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StudyTopic {
    pub tab_id: usize,
    pub title: &'static str,
    pub subject: &'static str,
    pub question: &'static str,
    pub formula: &'static str,
    pub derivation_steps: &'static str,
    pub board_exam_note: &'static str,
}

pub const STUDY_TOPICS: [StudyTopic; 3] = [
    StudyTopic {
        tab_id: 0,
        title: "[1] Gauss's Law",
        subject: "Physics — Electromagnetism",
        question: "Q: State Gauss's Law and derive the electric field for an isolated point charge using spherical symmetry.",
        formula: "∮ E · dA = Q_enclosed / ε₀",
        derivation_steps: "1. Construct a concentric spherical Gaussian surface of radius r centered on point charge +q.\n2. By spherical symmetry, electric field E is directed radially outward and has uniform magnitude across the entire surface.\n3. The flux integral simplifies:\n   Φ_E = ∮ E · dA = E ∮ dA = E (4πr²)\n4. Equate surface flux to enclosed charge over permittivity:\n   E (4πr²) = q / ε₀\n5. Solving directly for the electric field magnitude:\n   E = q / (4πε₀ r²)  [Inverse Square Law Verified]",
        board_exam_note: "Board Exam Note: Valid for any closed Gaussian surface enclosing net charge Q. Flux through surface is independent of its radius.",
    },
    StudyTopic {
        tab_id: 1,
        title: "[2] Nernst Equation",
        subject: "Chemistry — Electrochemistry",
        question: "Q: Derive the Nernst Equation for a Daniell Galvanic Cell operating at standard temperature 298 K.",
        formula: "E_cell = E°_cell - (0.0591 / n) · log₁₀(Q)",
        derivation_steps: "1. Overall Daniell cell spontaneous redox reaction:\n   Zn(s) + Cu²⁺(aq) ⇌ Zn²⁺(aq) + Cu(s)   [Number of electrons n = 2]\n2. Thermodynamic Gibbs free energy relation under non-standard conditions:\n   ΔG = ΔG° + RT · ln(Q)\n3. Substitute cell potential definitions ΔG = -nFE_cell and ΔG° = -nFE°_cell:\n   -nFE_cell = -nFE°_cell + RT · ln(Q)\n4. Divide both sides by -nF:\n   E_cell = E°_cell - (RT / nF) · ln(Q)\n5. At standard T = 298.15 K, R = 8.314 J/(mol·K), F = 96485 C/mol:\n   (2.303 · RT / F) = 0.05916 V\n6. ∴ E_cell = 1.10 V - (0.0591 / 2) · log₁₀([Zn²⁺] / [Cu²⁺])",
        board_exam_note: "Board Exam Note: Standard Daniell cell EMF E° is 1.10 V at 298 K. At dynamic equilibrium, E_cell = 0 and ΔG = 0.",
    },
    StudyTopic {
        tab_id: 2,
        title: "[3] Huygens Principle",
        subject: "Physics — Wave Optics",
        question: "Q: State Huygens Principle of secondary wavelets and rigorously prove Snell's Law of Refraction.",
        formula: "sin(i) / sin(r) = v₁ / v₂ = μ₂ / μ₁",
        derivation_steps: "1. Postulate: Every point on a wavefront behaves as a secondary source of spherical wavelets spreading in all forward directions at speed v.\n2. Consider a plane wavefront incident at angle i on a plane interface separating medium 1 (speed v₁) and medium 2 (speed v₂).\n3. In time duration τ, the wave travels distance BC = v₁τ in medium 1, while wavelet spreads distance AE = v₂τ in medium 2.\n4. From right triangle ΔABC:\n   sin(i) = BC / AC = (v₁ τ) / AC\n5. From right triangle ΔAEC:\n   sin(r) = AE / AC = (v₂ τ) / AC\n6. Dividing the two trigonometric equations eliminates common hypotenuse AC:\n   sin(i) / sin(r) = (v₁ τ) / (v₂ τ) = v₁ / v₂ = n₂ / n₁  [Snell's Law Proven]",
        board_exam_note: "Board Exam Note: Frequency remains strictly constant during optical refraction across interface. Wavelength scales with speed (λ₂ = λ₁ · v₂ / v₁).",
    },
];

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StudyReaderState {
    pub active_tab_index: usize,
}

impl StudyReaderState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_active_tab(&mut self, index: usize) {
        if index < STUDY_TOPICS.len() {
            self.active_tab_index = index;
        }
    }

    pub fn current_topic(&self) -> &'static StudyTopic {
        &STUDY_TOPICS[self.active_tab_index.min(STUDY_TOPICS.len() - 1)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_study_reader_topic_switching() {
        let mut reader = StudyReaderState::new();
        assert_eq!(reader.current_topic().title, "[1] Gauss's Law");
        reader.set_active_tab(1);
        assert_eq!(reader.current_topic().title, "[2] Nernst Equation");
        assert!(reader.current_topic().formula.contains("E_cell"));
        reader.set_active_tab(2);
        assert_eq!(reader.current_topic().title, "[3] Huygens Principle");
        assert!(reader
            .current_topic()
            .derivation_steps
            .contains("Snell's Law"));
    }
}
