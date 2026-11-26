//! Clean 3-Step Student Personal Information Onboarding & Personalization Wizard
//!
//! Gathers student identity, mission, and cognitive learning style to automatically
//! personalize Maindy AI and generate tailored daily study capsules.

use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Element, Length};

use crate::storage::db::UserProfile;
use crate::theme::{
    glass_card, glass_elevated, h_space, modern_text_input, primary_button, secondary_button,
    toggle_button, v_space, ThemePalette,
};
use crate::Message;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OnboardingStep {
    Step1WhoAreYou,
    Step2YourMission,
    Step3LearningStyle,
}

#[derive(Debug, Clone)]
pub struct OnboardingData {
    pub current_step: OnboardingStep,
    pub profile: UserProfile,
    // Step 1: Who Are You?
    pub name_input: String,
    pub class_degree_input: String,
    pub selected_language: String, // "thanglish", "ta", "ml", "en"
    // Step 2: Your Mission
    pub selected_track: String,
    pub target_score_value: u32,
    pub target_score_max: u32,
    pub exam_date_input: String,
    pub daily_study_hours: u32,
    // Step 3: Learning Style & Weak Spots
    pub strongest_subject: String,
    pub toughest_subject: String,
    pub coaching_tone: String,
}

impl Default for OnboardingData {
    fn default() -> Self {
        Self::new_with_profile(UserProfile::default())
    }
}

impl OnboardingData {
    pub fn new_with_profile(profile: UserProfile) -> Self {
        let name_input = if profile.name == "Scholar" {
            String::new()
        } else {
            profile.name.clone()
        };
        let strongest = profile
            .subjects
            .first()
            .cloned()
            .unwrap_or_else(|| "Physics".into());
        let toughest = profile
            .weak_topics
            .first()
            .cloned()
            .unwrap_or_else(|| "Electrochemistry Nernst Equation".into());
        let max = if profile.target_score_max > 0 {
            profile.target_score_max
        } else {
            600
        };
        let target = profile.target_score_value.min(max);

        Self {
            current_step: OnboardingStep::Step1WhoAreYou,
            name_input,
            class_degree_input: if profile.stream_or_focus.is_empty() {
                "Class 12".to_string()
            } else {
                profile.stream_or_focus.clone()
            },
            selected_language: if profile.maindy_style.is_empty() {
                "thanglish".to_string()
            } else {
                profile.maindy_style.clone()
            },
            selected_track: profile.career_track.clone(),
            target_score_value: target,
            target_score_max: max,
            exam_date_input: profile.exam_date.clone(),
            daily_study_hours: profile.weekday_study_hours.max(2),
            strongest_subject: strongest,
            toughest_subject: toughest,
            coaching_tone: if profile.coaching_style.is_empty()
                || profile.coaching_style == "BalancedMentor"
            {
                "Calm Mentor".to_string()
            } else {
                profile.coaching_style.clone()
            },
            profile,
        }
    }

    pub fn next(&mut self) {
        match self.current_step {
            OnboardingStep::Step1WhoAreYou => {
                self.current_step = OnboardingStep::Step2YourMission;
            }
            OnboardingStep::Step2YourMission => {
                self.current_step = OnboardingStep::Step3LearningStyle;
            }
            OnboardingStep::Step3LearningStyle => {
                self.commit_to_profile();
                self.profile.onboarding_completed = true;
            }
        }
    }

    pub fn prev(&mut self) {
        match self.current_step {
            OnboardingStep::Step1WhoAreYou => {}
            OnboardingStep::Step2YourMission => {
                self.current_step = OnboardingStep::Step1WhoAreYou;
            }
            OnboardingStep::Step3LearningStyle => {
                self.current_step = OnboardingStep::Step2YourMission;
            }
        }
    }

    pub fn apply_track(&mut self, track: &str) {
        self.selected_track = track.to_string();
        match track {
            "Class12BoardsTN600" => {
                self.target_score_max = 600;
                self.target_score_value = 590;
                self.profile.target_label = "TN Class 12 Boards • 590/600 Target".to_string();
                self.profile.subjects = vec![
                    "Physics".into(),
                    "Chemistry".into(),
                    "Mathematics".into(),
                    "Biology".into(),
                ];
            }
            "NEET720" => {
                self.target_score_max = 720;
                self.target_score_value = 680;
                self.profile.target_label = "NEET-UG • 680/720 Target".to_string();
                self.profile.subjects = vec![
                    "Physics".into(),
                    "Chemistry".into(),
                    "Botany".into(),
                    "Zoology".into(),
                ];
            }
            "JEE300" => {
                self.target_score_max = 300;
                self.target_score_value = 260;
                self.profile.target_label = "JEE Main & Advanced • 260/300 Target".to_string();
                self.profile.subjects =
                    vec!["Physics".into(), "Chemistry".into(), "Mathematics".into()];
            }
            "UPSC_TNPSC" | "UPSC" => {
                self.target_score_max = 200;
                self.target_score_value = 135;
                self.profile.target_label = "UPSC Civil Services • 135/200 Target".to_string();
                self.profile.subjects = vec![
                    "Polity".into(),
                    "History".into(),
                    "Economy".into(),
                    "Geography".into(),
                ];
            }
            "TNPSC" => {
                self.target_score_max = 300;
                self.target_score_value = 240;
                self.profile.target_label = "TNPSC Group 1/2 • 240/300 Target".to_string();
                self.profile.subjects = vec![
                    "General Studies".into(),
                    "Aptitude".into(),
                    "Tamil Culture".into(),
                ];
            }
            _ => {
                self.target_score_max = 100;
                self.target_score_value = 90;
                self.profile.target_label = "College / Custom Academic Goal".to_string();
                self.profile.subjects = vec!["Core Subject 1".into(), "Core Subject 2".into()];
            }
        }
    }

    pub fn commit_to_profile(&mut self) {
        let final_name = if self.name_input.trim().is_empty() {
            "Scholar".to_string()
        } else {
            self.name_input.trim().to_string()
        };
        self.profile.name = final_name;
        self.profile.stream_or_focus = self.class_degree_input.trim().to_string();
        self.profile.career_track = self.selected_track.clone();
        self.profile.target_score_max = self.target_score_max;
        self.profile.target_score_value = self.target_score_value.min(self.target_score_max);
        if !self.exam_date_input.trim().is_empty() {
            self.profile.exam_date = self.exam_date_input.trim().to_string();
        }
        self.profile.weekday_study_hours = self.daily_study_hours;
        self.profile.maindy_style = self.selected_language.clone();
        self.profile.language_code = match self.selected_language.as_str() {
            "ta" => "ta".to_string(),
            "ml" => "ml".to_string(),
            _ => "en".to_string(),
        };
        self.profile.coaching_style = self.coaching_tone.clone();

        // Ensure strongest subject is represented in subjects
        if !self.strongest_subject.trim().is_empty()
            && !self.profile.subjects.contains(&self.strongest_subject)
        {
            self.profile
                .subjects
                .insert(0, self.strongest_subject.trim().to_string());
        }

        // Ensure toughest subject is represented in weak_topics
        if !self.toughest_subject.trim().is_empty() {
            self.profile.weak_topics = vec![
                self.toughest_subject.trim().to_string(),
                "High-Yield Numericals".to_string(),
                "Time-Pressured Recall".to_string(),
            ];
        }
    }
}

pub fn view<'a>(data: &'a OnboardingData, palette: ThemePalette) -> Element<'a, Message> {
    let (step_num, step_title) = match data.current_step {
        OnboardingStep::Step1WhoAreYou => (1, "Step 1 of 3: Who Are You?"),
        OnboardingStep::Step2YourMission => (2, "Step 2 of 3: Your Mission"),
        OnboardingStep::Step3LearningStyle => (3, "Step 3 of 3: Learning Style & Weak Spots"),
    };

    let step_body: Element<'a, Message> = match data.current_step {
        OnboardingStep::Step1WhoAreYou => render_step1(data, palette),
        OnboardingStep::Step2YourMission => render_step2(data, palette),
        OnboardingStep::Step3LearningStyle => render_step3(data, palette),
    };

    let back_btn: Element<'a, Message> = if step_num > 1 {
        button(text("← Back").size(12))
            .on_press(Message::OnboardingPrev)
            .style(secondary_button(palette))
            .padding([8, 18])
            .into()
    } else {
        iced::widget::Space::new().into()
    };

    let next_btn = if step_num == 3 {
        button(
            row![
                text("🚀").size(14),
                text("Launch MindSet OS")
                    .size(13)
                    .color(palette.accent_text),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        )
        .on_press(Message::OnboardingComplete)
        .style(primary_button(palette))
        .padding([10, 24])
    } else {
        button(
            row![text("Next Step →").size(12).color(palette.accent_text)]
                .align_y(Alignment::Center),
        )
        .on_press(Message::OnboardingNext)
        .style(primary_button(palette))
        .padding([8, 20])
    };

    let navigation = row![back_btn, h_space(), next_btn].align_y(Alignment::Center);

    let card = container(
        column![
            row![
                text("✨").size(22),
                column![
                    text("STUDENT ONBOARDING & PERSONALIZATION")
                        .size(10)
                        .color(palette.accent),
                    text(step_title).size(16).color(palette.text_primary),
                ]
                .spacing(2),
                h_space(),
                container(
                    text(format!("{}/3", step_num))
                        .size(11)
                        .color(palette.accent)
                )
                .padding([4, 10])
                .style(move |_t| glass_card(palette)),
            ]
            .align_y(Alignment::Center),
            step_body,
            navigation,
        ]
        .spacing(20)
        .max_width(740)
        .padding(26),
    )
    .style(move |_t| glass_elevated(palette));

    container(column![v_space(), card, v_space()].align_x(Alignment::Center))
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(24)
        .into()
}

// -----------------------------------------------------------------------------
// STEP 1: WHO ARE YOU?
// -----------------------------------------------------------------------------
fn render_step1<'a>(data: &'a OnboardingData, palette: ThemePalette) -> Element<'a, Message> {
    // 1. Name input
    let name_field = column![
        text("Your Name or Nickname")
            .size(12)
            .color(palette.text_primary),
        text("What should Maindy AI call you? (e.g., Mapla, Arjun, Deepa)")
            .size(10)
            .color(palette.text_muted),
        container(
            text_input("Enter your name (e.g. Mapla, Arjun)", &data.name_input)
                .on_input(Message::OnboardingSetName)
                .padding(10)
                .size(13)
                .style(modern_text_input(palette))
        ),
    ]
    .spacing(6);

    // 2. Class / Academic Degree
    let classes = [
        "Class 12",
        "Class 11",
        "Undergraduate / College",
        "Competitive Exam Aspirant",
    ];
    let class_buttons = row(classes
        .into_iter()
        .map(|c| {
            let is_sel = data.class_degree_input == c;
            button(text(c).size(11).color(if is_sel {
                palette.accent_text
            } else {
                palette.text_primary
            }))
            .on_press(Message::OnboardingSetClassDegree(c.to_string()))
            .style(toggle_button(palette, is_sel))
            .padding([8, 14])
            .into()
        })
        .collect::<Vec<_>>())
    .spacing(8);

    let class_field = column![
        text("Current Class or Academic Level")
            .size(12)
            .color(palette.text_primary),
        class_buttons,
    ]
    .spacing(6);

    // 3. Preferred Language Style
    let langs = [
        (
            "thanglish",
            "Thanglish",
            "Tamil in English script • Friendly buddy feel",
        ),
        ("ta", "தமிழ் (Tamil)", "தூய தமிழ் வழிகாட்டி • High exam clarity"),
        (
            "ml",
            "മലയാളം (Malayalam)",
            "മലയാളം പഠன സഹായി • Natural clarity",
        ),
        ("en", "English", "Academic Mentor • Crisp Indian English"),
    ];

    let lang_cards = column(
        langs
            .into_iter()
            .map(|(code, title, desc)| {
                let is_sel = data.selected_language == code;
                button(
                    row![
                        column![
                            text(title).size(12).color(if is_sel {
                                palette.accent_text
                            } else {
                                palette.text_primary
                            }),
                            text(desc).size(10).color(if is_sel {
                                palette.accent_text
                            } else {
                                palette.text_muted
                            }),
                        ]
                        .spacing(2),
                        h_space(),
                        if is_sel {
                            text("✓").size(14).color(palette.accent_text)
                        } else {
                            text("").size(14)
                        },
                    ]
                    .align_y(Alignment::Center),
                )
                .on_press(Message::OnboardingSetLanguage(code.to_string()))
                .style(toggle_button(palette, is_sel))
                .padding([10, 16])
                .width(Length::Fill)
                .into()
            })
            .collect::<Vec<_>>(),
    )
    .spacing(6);

    let lang_field = column![
        text("Preferred Language for Maindy AI")
            .size(12)
            .color(palette.text_primary),
        lang_cards,
    ]
    .spacing(6);

    column![name_field, class_field, lang_field]
        .spacing(18)
        .into()
}

// -----------------------------------------------------------------------------
// STEP 2: YOUR MISSION
// -----------------------------------------------------------------------------
fn render_step2<'a>(data: &'a OnboardingData, palette: ThemePalette) -> Element<'a, Message> {
    let tracks = [
        (
            "Class12BoardsTN600",
            "TN Class 12 Boards",
            "Max 600 pts • Cutoff 585+",
        ),
        (
            "NEET720",
            "NEET-UG (Medical)",
            "Max 720 pts • +4/-1 Marking",
        ),
        ("JEE300", "JEE Main / Advanced", "Max 300 pts • Engineering"),
        (
            "UPSC_TNPSC",
            "UPSC Civil Services",
            "Max 200 pts • Prelims/Mains",
        ),
        ("TNPSC", "TNPSC Group 1 / 2", "Max 300 pts • Public Service"),
        (
            "CollegeCustom",
            "College / Custom",
            "Max 100 pts • GPA / Mastery",
        ),
    ];

    let track_cards = column![
        row(tracks
            .iter()
            .take(3)
            .map(|(code, title, desc)| {
                let is_sel = data.selected_track == *code;
                button(
                    column![
                        text(*title).size(12).color(if is_sel {
                            palette.accent_text
                        } else {
                            palette.text_primary
                        }),
                        text(*desc).size(10).color(if is_sel {
                            palette.accent_text
                        } else {
                            palette.text_muted
                        }),
                    ]
                    .spacing(2),
                )
                .on_press(Message::OnboardingSelectTrack(code.to_string()))
                .style(toggle_button(palette, is_sel))
                .padding([10, 14])
                .width(Length::FillPortion(1))
                .into()
            })
            .collect::<Vec<_>>())
        .spacing(8),
        row(tracks
            .iter()
            .skip(3)
            .map(|(code, title, desc)| {
                let is_sel = data.selected_track == *code;
                button(
                    column![
                        text(*title).size(12).color(if is_sel {
                            palette.accent_text
                        } else {
                            palette.text_primary
                        }),
                        text(*desc).size(10).color(if is_sel {
                            palette.accent_text
                        } else {
                            palette.text_muted
                        }),
                    ]
                    .spacing(2),
                )
                .on_press(Message::OnboardingSelectTrack(code.to_string()))
                .style(toggle_button(palette, is_sel))
                .padding([10, 14])
                .width(Length::FillPortion(1))
                .into()
            })
            .collect::<Vec<_>>())
        .spacing(8),
    ]
    .spacing(8);

    // Score adjustments with strict clamping
    let cur_score = data.target_score_value.min(data.target_score_max);
    let max_score = data.target_score_max;

    let score_controls = row![
        text(format!("Target Score: {} / {}", cur_score, max_score))
            .size(14)
            .color(palette.text_primary),
        h_space(),
        button(text("- 10").size(11))
            .on_press(Message::OnboardingSetTargetScore(
                cur_score.saturating_sub(10)
            ))
            .style(secondary_button(palette))
            .padding([4, 10]),
        button(text("- 5").size(11))
            .on_press(Message::OnboardingSetTargetScore(
                cur_score.saturating_sub(5)
            ))
            .style(secondary_button(palette))
            .padding([4, 10]),
        button(text("+ 5").size(11))
            .on_press(Message::OnboardingSetTargetScore(
                (cur_score + 5).min(max_score)
            ))
            .style(secondary_button(palette))
            .padding([4, 10]),
        button(text("+ 10").size(11))
            .on_press(Message::OnboardingSetTargetScore(
                (cur_score + 10).min(max_score)
            ))
            .style(secondary_button(palette))
            .padding([4, 10]),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    // Exam Date & Daily Study Hours
    let date_and_hours = row![
        column![
            text("Target Exam Date (YYYY-MM-DD)")
                .size(11)
                .color(palette.text_primary),
            container(
                text_input("2027-03-01", &data.exam_date_input)
                    .on_input(Message::OnboardingSetExamDate)
                    .padding(8)
                    .size(12)
                    .style(modern_text_input(palette))
            ),
        ]
        .spacing(4)
        .width(Length::FillPortion(1)),
        column![
            text("Daily Study Hours")
                .size(11)
                .color(palette.text_primary),
            row([2, 4, 6, 8, 10]
                .into_iter()
                .map(|h| {
                    let is_sel = data.daily_study_hours == h;
                    button(text(format!("{}h", h)).size(11).color(if is_sel {
                        palette.accent_text
                    } else {
                        palette.text_primary
                    }))
                    .on_press(Message::OnboardingSetWeekdayHours(h))
                    .style(toggle_button(palette, is_sel))
                    .padding([6, 12])
                    .into()
                })
                .collect::<Vec<_>>())
            .spacing(4),
        ]
        .spacing(4)
        .width(Length::FillPortion(1)),
    ]
    .spacing(16);

    column![
        column![
            text("Select Target Exam Track")
                .size(12)
                .color(palette.text_primary),
            track_cards
        ]
        .spacing(6),
        container(score_controls)
            .padding(12)
            .style(move |_t| glass_card(palette)),
        date_and_hours,
    ]
    .spacing(16)
    .into()
}

// -----------------------------------------------------------------------------
// STEP 3: LEARNING STYLE & WEAK SPOTS
// -----------------------------------------------------------------------------
fn render_step3<'a>(data: &'a OnboardingData, palette: ThemePalette) -> Element<'a, Message> {
    // 1. Strongest Subject
    let strong_presets = [
        "Physics",
        "Chemistry",
        "Mathematics",
        "Biology",
        "Computer Science",
    ];
    let strong_pills = row(strong_presets
        .into_iter()
        .map(|s| {
            let is_sel = data.strongest_subject == s;
            button(text(s).size(11).color(if is_sel {
                palette.accent_text
            } else {
                palette.text_primary
            }))
            .on_press(Message::OnboardingSetStrongSubject(s.to_string()))
            .style(toggle_button(palette, is_sel))
            .padding([6, 12])
            .into()
        })
        .collect::<Vec<_>>())
    .spacing(6);

    let strong_field = column![
        text("Your Strongest Subject")
            .size(12)
            .color(palette.text_primary),
        strong_pills,
        container(
            text_input("Or type strongest subject...", &data.strongest_subject)
                .on_input(Message::OnboardingSetStrongSubject)
                .padding(8)
                .size(12)
                .style(modern_text_input(palette))
        ),
    ]
    .spacing(6);

    // 2. Toughest Subject / Weak Area
    let tough_presets = [
        "Electrochemistry Nernst Eq",
        "Gauss Law & Flux",
        "Organic Mechanisms",
        "Calculus Integration",
    ];
    let tough_pills = row(tough_presets
        .into_iter()
        .map(|t| {
            let is_sel = data.toughest_subject == t;
            button(text(t).size(11).color(if is_sel {
                palette.accent_text
            } else {
                palette.text_primary
            }))
            .on_press(Message::OnboardingSetToughestSubject(t.to_string()))
            .style(toggle_button(palette, is_sel))
            .padding([6, 12])
            .into()
        })
        .collect::<Vec<_>>())
    .spacing(6);

    let tough_field = column![
        text("Toughest Topic Needing Priority Reinforcement")
            .size(12)
            .color(palette.text_primary),
        tough_pills,
        container(
            text_input(
                "e.g., Electrochemistry, Organic Reaction Mechanisms",
                &data.toughest_subject
            )
            .on_input(Message::OnboardingSetToughestSubject)
            .padding(8)
            .size(12)
            .style(modern_text_input(palette))
        ),
    ]
    .spacing(6);

    // 3. Maindy AI Tone
    let tones = [
        ("Friendly Thanglish Bro", "Casual, energetic brotherly vibe. Solra mapla, innaiku semma-ya padikrom! Zero hesitation."),
        ("Calm Mentor", "Balanced, patient, and compassionate. Methodical step-by-step clarity with zero guilt."),
        ("Strict Exam Coach", "High accountability, razor focus on speed, time pressure, and eliminating exam errors."),
    ];

    let tone_cards = column(
        tones
            .into_iter()
            .map(|(t_name, desc)| {
                let is_sel = data.coaching_tone == t_name;
                button(
                    column![
                        text(t_name).size(12).color(if is_sel {
                            palette.accent_text
                        } else {
                            palette.text_primary
                        }),
                        text(desc).size(10).color(if is_sel {
                            palette.accent_text
                        } else {
                            palette.text_muted
                        }),
                    ]
                    .spacing(2),
                )
                .on_press(Message::OnboardingSetCoachingStyle(t_name.to_string()))
                .style(toggle_button(palette, is_sel))
                .padding([10, 14])
                .width(Length::Fill)
                .into()
            })
            .collect::<Vec<_>>(),
    )
    .spacing(6);

    let tone_field = column![
        text("Maindy AI Coaching Tone")
            .size(12)
            .color(palette.text_primary),
        tone_cards,
    ]
    .spacing(6);

    column![strong_field, tough_field, tone_field]
        .spacing(16)
        .into()
}
