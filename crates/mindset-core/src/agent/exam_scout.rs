//! The Exam Scout AI Agent
//!
//! Researches official exam structures, maps syllabus units by 10-year PYQ mark weightage,
//! classifies chapters into ROI Quadrants (Goldmines, Heavyweights, Quick Wins, Traps),
//! and generates the structured Exam Intelligence Report.
//!
//! Copyright (c) 2026 LKS PVT LTD. All rights reserved.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RoiQuadrant {
    /// High mark weight + predictable / moderate difficulty (Priority 1)
    Goldmine,
    /// High mark weight + conceptually deep / tough (Priority 2)
    Heavyweight,
    /// Lower mark weight + fast to complete (Priority 3)
    QuickWin,
    /// Very low mark weight + high time cost (Priority 4)
    Trap,
}

impl RoiQuadrant {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Goldmine => "Goldmine (Priority 1)",
            Self::Heavyweight => "Heavyweight (Priority 2)",
            Self::QuickWin => "Quick Win (Priority 3)",
            Self::Trap => "Trap (Priority 4)",
        }
    }

    pub fn priority(&self) -> u8 {
        match self {
            Self::Goldmine => 1,
            Self::Heavyweight => 2,
            Self::QuickWin => 3,
            Self::Trap => 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChapterRoi {
    pub chapter_name: String,
    pub subject: String,
    pub weightage_marks: f32,
    pub effort_hours: f32,
    pub pyq_frequency_percentage: f32,
    pub quadrant: RoiQuadrant,
    pub high_yield_subtopics: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExamSection {
    pub name: String,
    pub question_count: u32,
    pub marks_per_question: f32,
    pub is_compulsory: bool,
    pub negative_marking: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubjectBreakdown {
    pub subject: String,
    pub total_marks: f32,
    pub expected_contribution: f32,
    pub chapters_count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExamProfile {
    pub id: String,
    pub name: String,
    pub category: String,
    pub total_marks: u32,
    pub default_target: u32,
    pub exam_date_label: String,
    pub duration_minutes: u32,
    pub sections: Vec<ExamSection>,
    pub chapters: Vec<ChapterRoi>,
}

impl ExamProfile {
    /// Returns the built-in TN Class 12 Board Exam profile (600 Total Marks, Target 590/600).
    pub fn tn_class_12_board() -> Self {
        let sections = vec![
            ExamSection {
                name: "Section A: 1-Mark MCQs".to_string(),
                question_count: 15,
                marks_per_question: 1.0,
                is_compulsory: true,
                negative_marking: 0.0,
            },
            ExamSection {
                name: "Section B: 2-Mark Short Answer".to_string(),
                question_count: 6,
                marks_per_question: 2.0,
                is_compulsory: false,
                negative_marking: 0.0,
            },
            ExamSection {
                name: "Section C: 3-Mark Conceptual".to_string(),
                question_count: 6,
                marks_per_question: 3.0,
                is_compulsory: false,
                negative_marking: 0.0,
            },
            ExamSection {
                name: "Section D: 5-Mark Long Problems".to_string(),
                question_count: 5,
                marks_per_question: 5.0,
                is_compulsory: false,
                negative_marking: 0.0,
            },
        ];

        let chapters = vec![
            ChapterRoi {
                chapter_name: "Electrochemistry & Nernst Equation".to_string(),
                subject: "Chemistry".to_string(),
                weightage_marks: 14.0,
                effort_hours: 6.0,
                pyq_frequency_percentage: 95.0,
                quadrant: RoiQuadrant::Goldmine,
                high_yield_subtopics: vec![
                    "Nernst equation derivation".to_string(),
                    "Kohlrausch's law of independent migration".to_string(),
                    "Standard EMF of cell calculations".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Electromagnetic Induction & AC".to_string(),
                subject: "Physics".to_string(),
                weightage_marks: 15.0,
                effort_hours: 8.0,
                pyq_frequency_percentage: 92.0,
                quadrant: RoiQuadrant::Heavyweight,
                high_yield_subtopics: vec![
                    "Faraday and Lenz's law applications".to_string(),
                    "Series LCR resonance circuit".to_string(),
                    "Transformer efficiency & losses".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Matrices & Determinants".to_string(),
                subject: "Mathematics".to_string(),
                weightage_marks: 12.0,
                effort_hours: 4.0,
                pyq_frequency_percentage: 98.0,
                quadrant: RoiQuadrant::Goldmine,
                high_yield_subtopics: vec![
                    "Inverse matrix method for linear systems".to_string(),
                    "Rank of a matrix using echelon form".to_string(),
                    "Cramer's rule application".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Genetics & Molecular Basis of Inheritance".to_string(),
                subject: "Biology".to_string(),
                weightage_marks: 18.0,
                effort_hours: 10.0,
                pyq_frequency_percentage: 96.0,
                quadrant: RoiQuadrant::Heavyweight,
                high_yield_subtopics: vec![
                    "DNA replication semi-conservative proof".to_string(),
                    "Lac operon regulation mechanism".to_string(),
                    "Genetic code degeneracy and translation".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Dual Nature of Radiation & Matter".to_string(),
                subject: "Physics".to_string(),
                weightage_marks: 8.0,
                effort_hours: 3.0,
                pyq_frequency_percentage: 88.0,
                quadrant: RoiQuadrant::QuickWin,
                high_yield_subtopics: vec![
                    "Einstein photoelectric equation".to_string(),
                    "de Broglie wavelength relation".to_string(),
                    "Davisson-Germer electron diffraction".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Surface Chemistry & Adsorption".to_string(),
                subject: "Chemistry".to_string(),
                weightage_marks: 6.0,
                effort_hours: 2.5,
                pyq_frequency_percentage: 82.0,
                quadrant: RoiQuadrant::QuickWin,
                high_yield_subtopics: vec![
                    "Freundlich adsorption isotherm".to_string(),
                    "Coagulation & Hardy-Schulze rule".to_string(),
                    "Colloids classification and Tyndall effect".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Polymer Chemistry & Biomolecules (Historical Details)".to_string(),
                subject: "Chemistry".to_string(),
                weightage_marks: 3.0,
                effort_hours: 7.0,
                pyq_frequency_percentage: 35.0,
                quadrant: RoiQuadrant::Trap,
                high_yield_subtopics: vec![
                    "Classification of biodegradable polymers".to_string(),
                    "Nylon-6,6 preparation equation".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Communication Systems (Fringe Modulations)".to_string(),
                subject: "Physics".to_string(),
                weightage_marks: 2.0,
                effort_hours: 6.0,
                pyq_frequency_percentage: 25.0,
                quadrant: RoiQuadrant::Trap,
                high_yield_subtopics: vec!["Space wave line of sight equation".to_string()],
            },
        ];

        Self {
            id: "tn_hsc_12".to_string(),
            name: "TN Class 12 Board Exam (HSC)".to_string(),
            category: "School Board".to_string(),
            total_marks: 600,
            default_target: 590,
            exam_date_label: "March 2027".to_string(),
            duration_minutes: 180,
            sections,
            chapters,
        }
    }

    /// Built-in NEET UG Profile (720 Total Marks, Target 680).
    pub fn neet_ug() -> Self {
        let sections = vec![
            ExamSection {
                name: "Section A: 35 Mandatory Questions".to_string(),
                question_count: 35,
                marks_per_question: 4.0,
                is_compulsory: true,
                negative_marking: 1.0,
            },
            ExamSection {
                name: "Section B: 15 Questions (Attempt Any 10)".to_string(),
                question_count: 10,
                marks_per_question: 4.0,
                is_compulsory: false,
                negative_marking: 1.0,
            },
        ];

        let chapters = vec![
            ChapterRoi {
                chapter_name: "Human Physiology & Neural Coordination".to_string(),
                subject: "Biology".to_string(),
                weightage_marks: 56.0,
                effort_hours: 12.0,
                pyq_frequency_percentage: 98.0,
                quadrant: RoiQuadrant::Goldmine,
                high_yield_subtopics: vec![
                    "Sliding filament theory of muscle contraction".to_string(),
                    "Synaptic transmission and action potential".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Optics: Ray and Wave".to_string(),
                subject: "Physics".to_string(),
                weightage_marks: 32.0,
                effort_hours: 14.0,
                pyq_frequency_percentage: 94.0,
                quadrant: RoiQuadrant::Heavyweight,
                high_yield_subtopics: vec![
                    "Young's double slit fringe width".to_string(),
                    "Lens maker's formula and combination".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Thermodynamics & Equilibrium".to_string(),
                subject: "Chemistry".to_string(),
                weightage_marks: 36.0,
                effort_hours: 11.0,
                pyq_frequency_percentage: 92.0,
                quadrant: RoiQuadrant::Heavyweight,
                high_yield_subtopics: vec![
                    "Gibbs free energy and spontaneity".to_string(),
                    "Buffer solution pH and Henderson equation".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Living World & Taxonomy".to_string(),
                subject: "Biology".to_string(),
                weightage_marks: 12.0,
                effort_hours: 3.0,
                pyq_frequency_percentage: 85.0,
                quadrant: RoiQuadrant::QuickWin,
                high_yield_subtopics: vec!["Taxonomic categories hierarchy".to_string()],
            },
            ChapterRoi {
                chapter_name: "Experimental Physics & Vernier Scale Details".to_string(),
                subject: "Physics".to_string(),
                weightage_marks: 4.0,
                effort_hours: 8.0,
                pyq_frequency_percentage: 30.0,
                quadrant: RoiQuadrant::Trap,
                high_yield_subtopics: vec![
                    "Screw gauge pitch and least count zero error".to_string()
                ],
            },
        ];

        Self {
            id: "neet_ug".to_string(),
            name: "NEET UG (Medical Entrance)".to_string(),
            category: "National Entrance".to_string(),
            total_marks: 720,
            default_target: 680,
            exam_date_label: "May 2027".to_string(),
            duration_minutes: 200,
            sections,
            chapters,
        }
    }

    /// Built-in JEE Main Profile (300 Total Marks, Target 250).
    /// Balanced across Physics, Chemistry, and Mathematics with all 4 ROI quadrants.
    pub fn jee_main() -> Self {
        let sections = vec![
            ExamSection {
                name: "Section A: 20 MCQs".to_string(),
                question_count: 20,
                marks_per_question: 4.0,
                is_compulsory: true,
                negative_marking: 1.0,
            },
            ExamSection {
                name: "Section B: Numerical Value Questions".to_string(),
                question_count: 5,
                marks_per_question: 4.0,
                is_compulsory: false,
                negative_marking: 1.0,
            },
        ];

        let chapters = vec![
            // Mathematics
            ChapterRoi {
                chapter_name: "Definite Integration & Areas".to_string(),
                subject: "Mathematics".to_string(),
                weightage_marks: 24.0,
                effort_hours: 10.0,
                pyq_frequency_percentage: 95.0,
                quadrant: RoiQuadrant::Goldmine,
                high_yield_subtopics: vec![
                    "King's property in definite integrals".to_string(),
                    "Area bounded between curves".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Coordinate Geometry: Conic Sections".to_string(),
                subject: "Mathematics".to_string(),
                weightage_marks: 20.0,
                effort_hours: 14.0,
                pyq_frequency_percentage: 92.0,
                quadrant: RoiQuadrant::Heavyweight,
                high_yield_subtopics: vec![
                    "Tangent and normal to parabola and ellipse".to_string(),
                    "Intersection and focal chords".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Matrices & Determinants".to_string(),
                subject: "Mathematics".to_string(),
                weightage_marks: 12.0,
                effort_hours: 4.0,
                pyq_frequency_percentage: 96.0,
                quadrant: RoiQuadrant::QuickWin,
                high_yield_subtopics: vec![
                    "Properties of determinants and Cramer's rule".to_string(),
                    "Inverse and adjoint operations".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Permutations & Combinations (Exotic Series)".to_string(),
                subject: "Mathematics".to_string(),
                weightage_marks: 4.0,
                effort_hours: 12.0,
                pyq_frequency_percentage: 30.0,
                quadrant: RoiQuadrant::Trap,
                high_yield_subtopics: vec!["Derangements and multinomial partitions".to_string()],
            },
            // Physics
            ChapterRoi {
                chapter_name: "Current Electricity & Circuits".to_string(),
                subject: "Physics".to_string(),
                weightage_marks: 20.0,
                effort_hours: 6.0,
                pyq_frequency_percentage: 95.0,
                quadrant: RoiQuadrant::Goldmine,
                high_yield_subtopics: vec![
                    "Kirchhoff's laws and nodal analysis".to_string(),
                    "Potentiometer and meter bridge comparisons".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Rotational Dynamics".to_string(),
                subject: "Physics".to_string(),
                weightage_marks: 20.0,
                effort_hours: 16.0,
                pyq_frequency_percentage: 90.0,
                quadrant: RoiQuadrant::Heavyweight,
                high_yield_subtopics: vec![
                    "Moment of inertia and parallel axis theorem".to_string(),
                    "Conservation of angular momentum in collisions".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Modern Physics: Dual Nature & Atoms".to_string(),
                subject: "Physics".to_string(),
                weightage_marks: 16.0,
                effort_hours: 5.0,
                pyq_frequency_percentage: 98.0,
                quadrant: RoiQuadrant::QuickWin,
                high_yield_subtopics: vec![
                    "Photoelectric cutoff frequencies and de Broglie relation".to_string(),
                    "Bohr radius and hydrogen emission spectra".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Rigid Body Rolling on Incline (Friction Boundary Cases)".to_string(),
                subject: "Physics".to_string(),
                weightage_marks: 4.0,
                effort_hours: 14.0,
                pyq_frequency_percentage: 25.0,
                quadrant: RoiQuadrant::Trap,
                high_yield_subtopics: vec![
                    "Instantaneous center of rotation on curved inclines".to_string()
                ],
            },
            // Chemistry
            ChapterRoi {
                chapter_name: "Electrochemistry & Chemical Kinetics".to_string(),
                subject: "Chemistry".to_string(),
                weightage_marks: 20.0,
                effort_hours: 7.0,
                pyq_frequency_percentage: 96.0,
                quadrant: RoiQuadrant::Goldmine,
                high_yield_subtopics: vec![
                    "Nernst equation and standard cell EMF".to_string(),
                    "First order rate constant and half-life kinetics".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Coordination Compounds".to_string(),
                subject: "Chemistry".to_string(),
                weightage_marks: 16.0,
                effort_hours: 10.0,
                pyq_frequency_percentage: 92.0,
                quadrant: RoiQuadrant::Heavyweight,
                high_yield_subtopics: vec![
                    "Crystal field splitting energy (CFSE) & magnetic moment".to_string(),
                    "Isomerism in octahedral complexes".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Periodic Properties & Chemical Bonding".to_string(),
                subject: "Chemistry".to_string(),
                weightage_marks: 16.0,
                effort_hours: 4.0,
                pyq_frequency_percentage: 97.0,
                quadrant: RoiQuadrant::QuickWin,
                high_yield_subtopics: vec![
                    "VSEPR geometries and hybridizations".to_string(),
                    "Ionization enthalpy and electron gain trends".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Qualitative Inorganic Salt Analysis (Rare Cation Groups)"
                    .to_string(),
                subject: "Chemistry".to_string(),
                weightage_marks: 4.0,
                effort_hours: 12.0,
                pyq_frequency_percentage: 20.0,
                quadrant: RoiQuadrant::Trap,
                high_yield_subtopics: vec![
                    "Confirmatory tests for group IV and V trace cations".to_string()
                ],
            },
        ];

        Self {
            id: "jee_main".to_string(),
            name: "JEE Main (Engineering)".to_string(),
            category: "National Entrance".to_string(),
            total_marks: 300,
            default_target: 250,
            exam_date_label: "April 2027".to_string(),
            duration_minutes: 180,
            sections,
            chapters,
        }
    }

    /// Built-in TNPSC Group 1 Preliminary Examination Profile (300 Total Marks, Target 240).
    pub fn tnpsc_group_1() -> Self {
        let sections = vec![
            ExamSection {
                name: "Section A: General Studies (Degree Standard)".to_string(),
                question_count: 175,
                marks_per_question: 1.5,
                is_compulsory: true,
                negative_marking: 0.0,
            },
            ExamSection {
                name: "Section B: Aptitude & Mental Ability (SSLC Standard)".to_string(),
                question_count: 25,
                marks_per_question: 1.5,
                is_compulsory: true,
                negative_marking: 0.0,
            },
        ];

        let chapters = vec![
            ChapterRoi {
                chapter_name: "Tamil Society, Culture & Thirukkural (Unit 8)".to_string(),
                subject: "Tamil Nadu Heritage & Culture".to_string(),
                weightage_marks: 45.0,
                effort_hours: 12.0,
                pyq_frequency_percentage: 98.0,
                quadrant: RoiQuadrant::Goldmine,
                high_yield_subtopics: vec![
                    "Thirukkural management and social justice teachings".to_string(),
                    "Archaeological discoveries at Keeladi and Korkai".to_string(),
                    "Self-respect movement and Justice Party contributions".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Development Administration in Tamil Nadu (Unit 9)".to_string(),
                subject: "Governance & Administration".to_string(),
                weightage_marks: 35.0,
                effort_hours: 8.0,
                pyq_frequency_percentage: 95.0,
                quadrant: RoiQuadrant::Goldmine,
                high_yield_subtopics: vec![
                    "Human Development Index trends across Tamil Nadu districts".to_string(),
                    "Social welfare schemes and affirmative action policies".to_string(),
                    "E-governance initiatives and industrial corridors".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Indian Polity & Constitution (Unit 5)".to_string(),
                subject: "Polity & Constitution".to_string(),
                weightage_marks: 30.0,
                effort_hours: 14.0,
                pyq_frequency_percentage: 94.0,
                quadrant: RoiQuadrant::Heavyweight,
                high_yield_subtopics: vec![
                    "Fundamental Rights, DPSP, and basic structure doctrine".to_string(),
                    "Centre-State legislative and financial relations".to_string(),
                    "Panchayati Raj 73rd and 74th Constitutional amendments".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Indian Economy & Planning (Unit 6)".to_string(),
                subject: "Economics".to_string(),
                weightage_marks: 25.0,
                effort_hours: 12.0,
                pyq_frequency_percentage: 90.0,
                quadrant: RoiQuadrant::Heavyweight,
                high_yield_subtopics: vec![
                    "Structure of Indian economy and NITI Aayog priorities".to_string(),
                    "GST Council mechanisms and fiscal federalism".to_string(),
                    "RBI monetary policy tools and inflation targeting".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Aptitude: Simplification, LCM, HCF & Percentages (Unit 10)"
                    .to_string(),
                subject: "Aptitude & Mental Ability".to_string(),
                weightage_marks: 25.0,
                effort_hours: 5.0,
                pyq_frequency_percentage: 98.0,
                quadrant: RoiQuadrant::QuickWin,
                high_yield_subtopics: vec![
                    "Ratio and proportion shortcuts".to_string(),
                    "Simple and compound interest comparisons".to_string(),
                    "Area and volume standard formulas".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Geography of Tamil Nadu & Natural Resources (Unit 3)".to_string(),
                subject: "Geography".to_string(),
                weightage_marks: 18.0,
                effort_hours: 6.0,
                pyq_frequency_percentage: 88.0,
                quadrant: RoiQuadrant::QuickWin,
                high_yield_subtopics: vec![
                    "River systems: Cauvery, Vaigai, and Thamirabarani".to_string(),
                    "Northeast monsoon rainfall distribution".to_string(),
                    "Major soil types and mineral distributions".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Pre-Historic Stone Ages & European World History".to_string(),
                subject: "History".to_string(),
                weightage_marks: 3.0,
                effort_hours: 15.0,
                pyq_frequency_percentage: 15.0,
                quadrant: RoiQuadrant::Trap,
                high_yield_subtopics: vec!["European colonial treaty timelines".to_string()],
            },
        ];

        Self {
            id: "tnpsc_group_1".to_string(),
            name: "TNPSC Group 1 Preliminary Examination".to_string(),
            category: "State Civil Services".to_string(),
            total_marks: 300,
            default_target: 240,
            exam_date_label: "July 2027".to_string(),
            duration_minutes: 180,
            sections,
            chapters,
        }
    }

    /// Built-in ICAI CA Foundation Profile (400 Total Marks, Target 320).
    pub fn ca_foundation() -> Self {
        let sections = vec![
            ExamSection {
                name: "Paper 1: Accounting (Subjective)".to_string(),
                question_count: 6,
                marks_per_question: 16.67,
                is_compulsory: true,
                negative_marking: 0.0,
            },
            ExamSection {
                name: "Paper 2: Business Laws (Subjective)".to_string(),
                question_count: 5,
                marks_per_question: 20.0,
                is_compulsory: true,
                negative_marking: 0.0,
            },
            ExamSection {
                name: "Paper 3: Quantitative Aptitude (Objective)".to_string(),
                question_count: 100,
                marks_per_question: 1.0,
                is_compulsory: true,
                negative_marking: 0.25,
            },
            ExamSection {
                name: "Paper 4: Business Economics (Objective)".to_string(),
                question_count: 100,
                marks_per_question: 1.0,
                is_compulsory: true,
                negative_marking: 0.25,
            },
        ];

        let chapters = vec![
            ChapterRoi {
                chapter_name: "Partnership & LLP Accounts".to_string(),
                subject: "Accounting".to_string(),
                weightage_marks: 25.0,
                effort_hours: 8.0,
                pyq_frequency_percentage: 96.0,
                quadrant: RoiQuadrant::Goldmine,
                high_yield_subtopics: vec![
                    "Goodwill treatment on admission and retirement".to_string(),
                    "Revaluation of assets and partner capital adjustment".to_string(),
                    "Dissolution piecemeal distribution".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Indian Contract Act, 1872".to_string(),
                subject: "Business Laws".to_string(),
                weightage_marks: 30.0,
                effort_hours: 10.0,
                pyq_frequency_percentage: 95.0,
                quadrant: RoiQuadrant::Goldmine,
                high_yield_subtopics: vec![
                    "Offer, acceptance, and lawful consideration rules".to_string(),
                    "Free consent: coercion, undue influence, and fraud".to_string(),
                    "Remedies for breach of contract".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Consignment & Bills of Exchange".to_string(),
                subject: "Accounting".to_string(),
                weightage_marks: 20.0,
                effort_hours: 12.0,
                pyq_frequency_percentage: 90.0,
                quadrant: RoiQuadrant::Heavyweight,
                high_yield_subtopics: vec![
                    "Valuation of unsold consignment stock and abnormal loss".to_string(),
                    "Accommodation bills and discounting ledger entries".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Time Value of Money & Annuities".to_string(),
                subject: "Quantitative Aptitude".to_string(),
                weightage_marks: 22.0,
                effort_hours: 6.0,
                pyq_frequency_percentage: 98.0,
                quadrant: RoiQuadrant::Goldmine,
                high_yield_subtopics: vec![
                    "Present value and future value of regular annuity".to_string(),
                    "Effective rate of interest and sinking fund calculations".to_string(),
                    "Perpetuity and compounding discount factors".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Theory of Demand & Consumer Behaviour".to_string(),
                subject: "Business Economics".to_string(),
                weightage_marks: 18.0,
                effort_hours: 4.0,
                pyq_frequency_percentage: 96.0,
                quadrant: RoiQuadrant::QuickWin,
                high_yield_subtopics: vec![
                    "Price, income, and cross elasticity of demand".to_string(),
                    "Indifference curve convex properties and consumer equilibrium".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Logical Reasoning: Number Series & Directions".to_string(),
                subject: "Quantitative Aptitude".to_string(),
                weightage_marks: 15.0,
                effort_hours: 3.0,
                pyq_frequency_percentage: 95.0,
                quadrant: RoiQuadrant::QuickWin,
                high_yield_subtopics: vec![
                    "Alpha-numeric series and odd-one-out patterns".to_string(),
                    "Blood relation family trees and directional steps".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Theoretical Framework of Accounting Trivia".to_string(),
                subject: "Accounting".to_string(),
                weightage_marks: 4.0,
                effort_hours: 10.0,
                pyq_frequency_percentage: 20.0,
                quadrant: RoiQuadrant::Trap,
                high_yield_subtopics: vec![
                    "Historical accounting standards dates and trivia".to_string()
                ],
            },
        ];

        Self {
            id: "ca_foundation".to_string(),
            name: "ICAI CA Foundation Examination".to_string(),
            category: "Professional Accounting".to_string(),
            total_marks: 400,
            default_target: 320,
            exam_date_label: "June 2027".to_string(),
            duration_minutes: 180,
            sections,
            chapters,
        }
    }

    /// Built-in UPSC CSE Prelims Profile (200 Total Marks, Target 120).
    pub fn upsc_cse_prelims() -> Self {
        let sections = vec![ExamSection {
            name: "General Studies Paper 1 (Objective MCQs)".to_string(),
            question_count: 100,
            marks_per_question: 2.0,
            is_compulsory: true,
            negative_marking: 0.66,
        }];

        let chapters = vec![
            ChapterRoi {
                chapter_name: "Modern Indian History & National Movement".to_string(),
                subject: "History".to_string(),
                weightage_marks: 28.0,
                effort_hours: 14.0,
                pyq_frequency_percentage: 96.0,
                quadrant: RoiQuadrant::Goldmine,
                high_yield_subtopics: vec![
                    "Non-Cooperation, Civil Disobedience, and Quit India phases".to_string(),
                    "Constitutional acts: 1909, 1919, and Government of India Act 1935".to_string(),
                    "Socio-religious reform movements in 19th century".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Indian Polity: Constitutional Framework & Governance".to_string(),
                subject: "Polity".to_string(),
                weightage_marks: 32.0,
                effort_hours: 12.0,
                pyq_frequency_percentage: 98.0,
                quadrant: RoiQuadrant::Goldmine,
                high_yield_subtopics: vec![
                    "Preamble, Fundamental Rights, and Judicial Review".to_string(),
                    "Parliamentary procedures, motions, and Money Bills".to_string(),
                    "Constitutional bodies: Election Commission, CAG, and UPSC".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Environment, Ecology & Climate Change".to_string(),
                subject: "Environment".to_string(),
                weightage_marks: 30.0,
                effort_hours: 16.0,
                pyq_frequency_percentage: 92.0,
                quadrant: RoiQuadrant::Heavyweight,
                high_yield_subtopics: vec![
                    "National Parks, Wildlife Sanctuaries, and Tiger Reserves".to_string(),
                    "UNFCCC COP treaties, Paris Agreement, and CBD protocols".to_string(),
                    "IUCN red list conservation statuses and biodiversity hotspots".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Macroeconomics: Monetary Policy, Fiscal Deficits & Inflation"
                    .to_string(),
                subject: "Economics".to_string(),
                weightage_marks: 26.0,
                effort_hours: 15.0,
                pyq_frequency_percentage: 90.0,
                quadrant: RoiQuadrant::Heavyweight,
                high_yield_subtopics: vec![
                    "Repo rate, reverse repo, and liquidity adjustment facility".to_string(),
                    "Balance of Payments: current vs capital account convertibility".to_string(),
                    "Direct vs indirect taxes and FRBM Act mandates".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Science & Technology: Space, AI & Biotechnology".to_string(),
                subject: "Science & Tech".to_string(),
                weightage_marks: 18.0,
                effort_hours: 6.0,
                pyq_frequency_percentage: 90.0,
                quadrant: RoiQuadrant::QuickWin,
                high_yield_subtopics: vec![
                    "ISRO launch vehicles: PSLV, GSLV Mk-III, and Gaganyaan".to_string(),
                    "CRISPR-Cas9 gene editing and mRNA vaccines".to_string(),
                    "Quantum computing and large language model architectures".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Physical Geography: Climatology & Oceanography".to_string(),
                subject: "Geography".to_string(),
                weightage_marks: 14.0,
                effort_hours: 5.0,
                pyq_frequency_percentage: 88.0,
                quadrant: RoiQuadrant::QuickWin,
                high_yield_subtopics: vec![
                    "Atmospheric circulation, Coriolis force, and pressure belts".to_string(),
                    "Ocean currents and El Niño / Southern Oscillation (ENSO)".to_string(),
                ],
            },
            ChapterRoi {
                chapter_name: "Medieval Dynastic Genealogies & Obscure Chronicles".to_string(),
                subject: "History".to_string(),
                weightage_marks: 4.0,
                effort_hours: 20.0,
                pyq_frequency_percentage: 15.0,
                quadrant: RoiQuadrant::Trap,
                high_yield_subtopics: vec![
                    "Minor feudal chieftain lineage succession records".to_string()
                ],
            },
        ];

        Self {
            id: "upsc_cse_prelims".to_string(),
            name: "UPSC Civil Services Preliminary Examination (GS-1)".to_string(),
            category: "National Civil Services".to_string(),
            total_marks: 200,
            default_target: 120,
            exam_date_label: "May 2027".to_string(),
            duration_minutes: 120,
            sections,
            chapters,
        }
    }

    /// List of all built-in profiles.
    pub fn all_builtin() -> Vec<Self> {
        vec![
            Self::tn_class_12_board(),
            Self::neet_ug(),
            Self::jee_main(),
            Self::tnpsc_group_1(),
            Self::ca_foundation(),
            Self::upsc_cse_prelims(),
        ]
    }
}

/// The structured Exam Intelligence Report produced by the Scout Agent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExamIntelligenceReport {
    pub exam_id: String,
    pub exam_name: String,
    pub total_marks: u32,
    pub target_score: u32,
    pub baseline_readiness: u32,
    pub safety_buffer_marks: f32,
    pub total_score_gap: f32,
    /// Priority-sorted top 10 preview of highest-yield chapters (Goldmines and Heavyweights first)
    pub top_roi_chapters: Vec<ChapterRoi>,
    /// Complete subject breakdown aggregated across the entire profile blueprint
    pub subjects: Vec<SubjectBreakdown>,
    pub strategic_insights: Vec<String>,
}

pub struct ExamScoutAgent;

impl ExamScoutAgent {
    /// Generates an Exam Intelligence Report for a given exam profile, target, and baseline.
    ///
    /// - `top_roi_chapters` presents a priority-sorted top-10 preview of highest-yield chapters.
    /// - `subjects` aggregates chapter counts and total marks across the entire `profile.chapters` blueprint.
    pub fn generate_report(
        profile: &ExamProfile,
        target_score: u32,
        baseline_score: u32,
    ) -> ExamIntelligenceReport {
        // Enforce target does not exceed max marks
        let effective_target = target_score.min(profile.total_marks);
        let effective_baseline = baseline_score.min(effective_target);

        // Safety buffer: 2.5% of total marks, capped so (target + buffer) <= total_marks
        let raw_buffer = (profile.total_marks as f32) * 0.025;
        let target_plus_buffer =
            (effective_target as f32 + raw_buffer).min(profile.total_marks as f32);
        let safety_buffer_marks = target_plus_buffer - effective_target as f32;
        let total_score_gap = target_plus_buffer - effective_baseline as f32;

        // Sort chapters by priority then weightage
        let mut sorted_chapters = profile.chapters.clone();
        sorted_chapters.sort_by(|a, b| {
            a.quadrant
                .priority()
                .cmp(&b.quadrant.priority())
                .then_with(|| {
                    b.weightage_marks
                        .partial_cmp(&a.weightage_marks)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
        });

        // Top 10 highest-ROI chapters
        let top_roi = sorted_chapters.into_iter().take(10).collect::<Vec<_>>();

        // Calculate subject breakdowns
        let mut subjects_map = std::collections::BTreeMap::<String, (f32, usize)>::new();
        for ch in &profile.chapters {
            let entry = subjects_map.entry(ch.subject.clone()).or_insert((0.0, 0));
            entry.0 += ch.weightage_marks;
            entry.1 += 1;
        }

        let total_chapters_marks: f32 = subjects_map.values().map(|(m, _)| *m).sum();
        let mut subjects = Vec::new();
        for (subj, (m, count)) in subjects_map {
            let expected_contrib = if total_chapters_marks > 0.0 {
                (m / total_chapters_marks) * effective_target as f32
            } else {
                0.0
            };
            subjects.push(SubjectBreakdown {
                subject: subj,
                total_marks: m,
                expected_contribution: expected_contrib,
                chapters_count: count,
            });
        }

        // Strategic Insights
        let mut strategic_insights = Vec::new();
        let goldmine_count = profile
            .chapters
            .iter()
            .filter(|c| c.quadrant == RoiQuadrant::Goldmine)
            .count();
        let trap_count = profile
            .chapters
            .iter()
            .filter(|c| c.quadrant == RoiQuadrant::Trap)
            .count();

        strategic_insights.push(format!(
            "Targeting {}/{} requires a +{:.1} mark improvement across remaining study days.",
            effective_target, profile.total_marks, total_score_gap
        ));
        strategic_insights.push(format!(
            "Identified {} Goldmine chapters giving immediate maximum marks per hour.",
            goldmine_count
        ));
        if trap_count > 0 {
            strategic_insights.push(format!(
                "Deprioritized {} low-yield Trap units to protect your core revision window.",
                trap_count
            ));
        }
        strategic_insights.push(
            "Every completed capsule adds verified marks to your Projected Exam Score.".to_string(),
        );

        ExamIntelligenceReport {
            exam_id: profile.id.clone(),
            exam_name: profile.name.clone(),
            total_marks: profile.total_marks,
            target_score: effective_target,
            baseline_readiness: effective_baseline,
            safety_buffer_marks,
            total_score_gap,
            top_roi_chapters: top_roi,
            subjects,
            strategic_insights,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tn_class_12_profile_defaults() {
        let p = ExamProfile::tn_class_12_board();
        assert_eq!(p.total_marks, 600);
        assert_eq!(p.default_target, 590);
        assert!(!p.chapters.is_empty());
    }

    #[test]
    fn test_exam_scout_report_generation() {
        let p = ExamProfile::tn_class_12_board();
        let report = ExamScoutAgent::generate_report(&p, 590, 430);

        assert_eq!(report.target_score, 590);
        assert_eq!(report.baseline_readiness, 430);
        // Gap should be (590 + buffer) - 430
        assert!(report.total_score_gap >= 160.0);
        assert!(report.safety_buffer_marks >= 0.0);
        // Verify goldmines appear before traps
        assert_eq!(report.top_roi_chapters[0].quadrant, RoiQuadrant::Goldmine);
        assert!(!report.strategic_insights.is_empty());
    }

    #[test]
    fn test_roi_quadrant_ordering() {
        assert!(RoiQuadrant::Goldmine.priority() < RoiQuadrant::Heavyweight.priority());
        assert!(RoiQuadrant::Heavyweight.priority() < RoiQuadrant::QuickWin.priority());
        assert!(RoiQuadrant::QuickWin.priority() < RoiQuadrant::Trap.priority());
    }

    #[test]
    fn test_all_builtin_profiles_exist_and_valid() {
        let profiles = ExamProfile::all_builtin();
        assert_eq!(profiles.len(), 6);

        let ids: Vec<&str> = profiles.iter().map(|p| p.id.as_str()).collect();
        assert!(ids.contains(&"tn_hsc_12"));
        assert!(ids.contains(&"neet_ug"));
        assert!(ids.contains(&"jee_main"));
        assert!(ids.contains(&"tnpsc_group_1"));
        assert!(ids.contains(&"ca_foundation"));
        assert!(ids.contains(&"upsc_cse_prelims"));

        for p in &profiles {
            assert!(!p.name.is_empty());
            assert!(p.total_marks > 0);
            assert!(p.default_target <= p.total_marks);
            assert!(!p.sections.is_empty());
            assert!(!p.chapters.is_empty());
        }
    }

    #[test]
    fn test_jee_main_balanced_subjects() {
        let p = ExamProfile::jee_main();
        let has_physics = p.chapters.iter().any(|c| c.subject == "Physics");
        let has_chemistry = p.chapters.iter().any(|c| c.subject == "Chemistry");
        let has_maths = p.chapters.iter().any(|c| c.subject == "Mathematics");

        assert!(has_physics, "JEE Main must contain Physics chapters");
        assert!(has_chemistry, "JEE Main must contain Chemistry chapters");
        assert!(has_maths, "JEE Main must contain Mathematics chapters");

        // Verify all 4 quadrants represented
        let quadrants: std::collections::HashSet<_> =
            p.chapters.iter().map(|c| c.quadrant).collect();
        assert!(quadrants.contains(&RoiQuadrant::Goldmine));
        assert!(quadrants.contains(&RoiQuadrant::Heavyweight));
        assert!(quadrants.contains(&RoiQuadrant::QuickWin));
        assert!(quadrants.contains(&RoiQuadrant::Trap));
    }

    #[test]
    fn test_subject_breakdown_matches_full_blueprint() {
        let p = ExamProfile::tnpsc_group_1();
        let report = ExamScoutAgent::generate_report(&p, 240, 180);

        let total_breakdown_chapters: usize =
            report.subjects.iter().map(|s| s.chapters_count).sum();
        assert_eq!(total_breakdown_chapters, p.chapters.len());

        let total_breakdown_marks: f32 = report.subjects.iter().map(|s| s.total_marks).sum();
        let total_blueprint_marks: f32 = p.chapters.iter().map(|c| c.weightage_marks).sum();
        assert!((total_breakdown_marks - total_blueprint_marks).abs() < 0.001);
    }
}
