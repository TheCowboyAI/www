use iced::{
    alignment, executor, font, widget::{button, column, container, row, text, scrollable, Column, Container, Row, Space},
    Alignment, Application, Command, Element, Font, Length, Settings, Subscription, Theme,
    Color, Background, Border,
};
use iced::widget::svg;
use wasm_bindgen::prelude::*;

pub mod presentation_theme;
use presentation_theme::PresentationTheme;

#[wasm_bindgen]
pub fn run() {
    CowboyPresentation::run(Settings::default()).unwrap();
}

pub fn run_native() -> iced::Result {
    CowboyPresentation::run(Settings::default())
}

#[derive(Debug, Clone)]
pub enum Message {
    NextSlide,
    PreviousSlide,
    GoToSlide(usize),
}

pub struct CowboyPresentation {
    current_slide: usize,
    slides: Vec<Slide>,
}

pub struct Slide {
    title: String,
    subtitle: Option<String>,
    content: SlideContent,
    background: Background,
}

pub enum SlideContent {
    Hero {
        title: String,
        subtitle: String,
        description: String,
        warning: String,
    },
    Cognition {
        items: Vec<CircularItem>,
        description: String,
    },
    Problems {
        cards: Vec<ProblemCard>,
    },
    Platform {
        features: Vec<PlatformFeature>,
    },
    Stack {
        components: Vec<StackComponent>,
    },
    Breakthrough {
        items: Vec<BreakthroughItem>,
    },
    Trust {
        sections: Vec<TrustSection>,
    },
    Sage {
        capabilities: Vec<SageCapability>,
    },
}

pub struct CircularItem {
    number: u8,
    title: String,
    subtitle: String,
    color: Color,
}

pub struct ProblemCard {
    title: String,
    description: String,
    icon: String,
}

pub struct PlatformFeature {
    number: u8,
    title: String,
    description: String,
}

pub struct StackComponent {
    number: u8,
    title: String,
    subtitle: String,
}

pub struct BreakthroughItem {
    icon: String,
    title: String,
    description: String,
}

pub struct TrustSection {
    icon: String,
    title: String,
    items: Vec<String>,
}

pub struct SageCapability {
    emoji: String,
    title: String,
    points: Vec<String>,
}

impl Application for CowboyPresentation {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let slides = vec![
            // Slide 1: Hero
            Slide {
                title: "Cowboy AI".to_string(),
                subtitle: None,
                content: SlideContent::Hero {
                    title: "Cowboy AI".to_string(),
                    subtitle: "The Platform For Composable, Cognitive, Audit‑Grade AI Swarms".to_string(),
                    description: "Your New Business Brain & Nervous System. Cognition. Evolution. Security. Trust.\nNo‑Code Composable Multi‑AI Agent Orchestration At Scale".to_string(),
                    warning: "Disclaimer & Warning: You can now ask and command the system to do anything. In plain English. And it will.".to_string(),
                },
                background: PresentationTheme::hero_gradient(),
            },
            // Slide 2: Cognition
            Slide {
                title: "No Cognition? - No AGI!".to_string(),
                subtitle: Some("Intelligence Alone Isn't Enough—Cognition Is the Missing Piece.".to_string()),
                content: SlideContent::Cognition {
                    items: vec![
                        CircularItem {
                            number: 1,
                            title: "Hardware".to_string(),
                            subtitle: "Horsepower".to_string(),
                            color: Color::from_rgb(0.06, 0.72, 0.51),
                        },
                        CircularItem {
                            number: 2,
                            title: "LLM".to_string(),
                            subtitle: "Raw tooling".to_string(),
                            color: Color::from_rgb(0.06, 0.72, 0.51),
                        },
                        CircularItem {
                            number: 3,
                            title: "Cognition".to_string(),
                            subtitle: "Awareness of tools & State".to_string(),
                            color: Color::from_rgb(0.23, 0.51, 0.96),
                        },
                        CircularItem {
                            number: 4,
                            title: "Your World - The Data".to_string(),
                            subtitle: "Clean, Holistic, Accessible, Mathematically Proven".to_string(),
                            color: Color::from_rgb(0.23, 0.51, 0.96),
                        },
                    ],
                    description: "For true AGI, systems must learn, adapt, and evolve—not just process data.\nToday's AI is trapped in silos, built on imperfect math, with no structure or guardrails.\nThat's why it keeps failing. We solved it.".to_string(),
                },
                background: PresentationTheme::cognition_gradient(),
            },
            // Slide 3: Problems
            Slide {
                title: "AI Is Breaking at Scale".to_string(),
                subtitle: None,
                content: SlideContent::Problems {
                    cards: vec![
                        ProblemCard {
                            title: "Fragmentation kills velocity".to_string(),
                            description: "Businesses juggle many applications & tools, re‑enter data, and never achieve true sync.".to_string(),
                            icon: "🚧".to_string(),
                        },
                        ProblemCard {
                            title: "The last 10% explodes compute".to_string(),
                            description: "Teams reach 85–90% automation, then costs and fragility spike without the right structures.".to_string(),
                            icon: "💣".to_string(),
                        },
                        ProblemCard {
                            title: "Lock‑in & opacity".to_string(),
                            description: "Cloud‑centric AI → runaway spend, brittle workflows, and audits no one can pass.".to_string(),
                            icon: "🔒".to_string(),
                        },
                    ],
                },
                background: PresentationTheme::problem_gradient(),
            },
            // Slide 4: Platform
            Slide {
                title: "Platform First — Cowboy AI CIM".to_string(),
                subtitle: Some("The platform for building, governing, and scaling your business with AI swarms.".to_string()),
                content: SlideContent::Platform {
                    features: vec![
                        PlatformFeature {
                            number: 1,
                            title: "Build Once, Replicate Anywhere".to_string(),
                            description: "We're proving it in private lending/fintech first. Repeatable & Reproduceable.".to_string(),
                        },
                        PlatformFeature {
                            number: 2,
                            title: "Open for Builders".to_string(),
                            description: "Publish domain packs, swarms, and connectors other teams can adopt.".to_string(),
                        },
                        PlatformFeature {
                            number: 3,
                            title: "Proven Milestones".to_string(),
                            description: "8M+ micro-transactions processed; >90% lower AI compute vs cloud-only.".to_string(),
                        },
                        PlatformFeature {
                            number: 4,
                            title: "No‑code, modular agent swarms".to_string(),
                            description: "Like little building blocks AI agents assemble on the fly.".to_string(),
                        },
                        PlatformFeature {
                            number: 5,
                            title: "Immutable, Security First".to_string(),
                            description: "Immutable records, ransomware-resistant architecture".to_string(),
                        },
                    ],
                },
                background: PresentationTheme::platform_gradient(),
            },
            // Slide 5: Stack
            Slide {
                title: "Who's Who in the Stack".to_string(),
                subtitle: None,
                content: SlideContent::Stack {
                    components: vec![
                        StackComponent {
                            number: 1,
                            title: "SAGE - Agent Orchestrator".to_string(),
                            subtitle: "The Brain - Composes and drives swarms of AIs".to_string(),
                        },
                        StackComponent {
                            number: 2,
                            title: "Alchemist Ontology".to_string(),
                            subtitle: "The Nervous System - Your real-time knowledge graph".to_string(),
                        },
                        StackComponent {
                            number: 3,
                            title: "CIM - Platform".to_string(),
                            subtitle: "The Body - Your Composable Information Machine".to_string(),
                        },
                    ],
                },
                background: PresentationTheme::cognition_gradient(),
            },
            // Slide 6: Breakthrough
            Slide {
                title: "Our Breakthrough — Swarm AI that actually scales".to_string(),
                subtitle: None,
                content: SlideContent::Breakthrough {
                    items: vec![
                        BreakthroughItem {
                            icon: "🔄".to_string(),
                            title: "Repeatable".to_string(),
                            description: "Commutative-diagram checks ensure same outcomes".to_string(),
                        },
                        BreakthroughItem {
                            icon: "📊".to_string(),
                            title: "Explainable & Auditable".to_string(),
                            description: "Every state change is an immutable event".to_string(),
                        },
                        BreakthroughItem {
                            icon: "🔄".to_string(),
                            title: "Model-Hot-Swap".to_string(),
                            description: "Switch between Claude/GPT/Llama without losing state".to_string(),
                        },
                        BreakthroughItem {
                            icon: "💰".to_string(),
                            title: "90% Cheaper".to_string(),
                            description: "Hybrid runtime + intelligent routing".to_string(),
                        },
                        BreakthroughItem {
                            icon: "🧩".to_string(),
                            title: "Modular Building Blocks".to_string(),
                            description: "Atomic functions → Extreme reuse".to_string(),
                        },
                        BreakthroughItem {
                            icon: "🚀".to_string(),
                            title: "AI Amplification".to_string(),
                            description: "Every employee gets an AI team on the fly".to_string(),
                        },
                    ],
                },
                background: PresentationTheme::breakthrough_gradient(),
            },
            // Slide 7: Trust
            Slide {
                title: "Trust by Design".to_string(),
                subtitle: Some("Security, Control, and Explainability at the Core".to_string()),
                content: SlideContent::Trust {
                    sections: vec![
                        TrustSection {
                            icon: "🏗️".to_string(),
                            title: "Infrastructure of Integrity".to_string(),
                            items: vec![
                                "Apple M3 Ultra + NixDarwin for deterministic environments".to_string(),
                                "Rust + Bevy Runtime for memory-safe orchestration".to_string(),
                                "NATS Messaging for real-time, fault-tolerant communication".to_string(),
                            ],
                        },
                        TrustSection {
                            icon: "🔐".to_string(),
                            title: "Security That Scales".to_string(),
                            items: vec![
                                "YubiKey Hardware Auth + ECC Encryption".to_string(),
                                "MinIO + Wasabi immutable storage".to_string(),
                                "AI Model Agnostic deployment".to_string(),
                            ],
                        },
                        TrustSection {
                            icon: "🧠".to_string(),
                            title: "Explainable Intelligence".to_string(),
                            items: vec![
                                "Neo4j Graph Engine + Event Sourcing".to_string(),
                                "Functional Programming + TDD".to_string(),
                                "DDD + Conceptual Spaces".to_string(),
                            ],
                        },
                        TrustSection {
                            icon: "🔢".to_string(),
                            title: "Mathematics of Trust".to_string(),
                            items: vec![
                                "Category Theory & Graph Theory".to_string(),
                                "Gradient Descent Programming".to_string(),
                                "Trust proven line by line".to_string(),
                            ],
                        },
                    ],
                },
                background: PresentationTheme::trust_gradient(),
            },
            // Slide 8: SAGE
            Slide {
                title: "SAGE".to_string(),
                subtitle: Some("Universal Development Intelligence Platform\nOrchestrating 100+ Specialized AI Experts from Silicon to Solution".to_string()),
                content: SlideContent::Sage {
                    capabilities: vec![
                        SageCapability {
                            emoji: "🎯".to_string(),
                            title: "Complete Stack Mastery".to_string(),
                            points: vec![
                                "Nix Expert: Kernel to cloud expertise".to_string(),
                                "Resource Expert: Real-world aware".to_string(),
                                "Domain & DDD Experts".to_string(),
                                "Infrastructure Experts".to_string(),
                            ],
                        },
                        SageCapability {
                            emoji: "🔬".to_string(),
                            title: "Mathematical Foundation".to_string(),
                            points: vec![
                                "Provable Correctness".to_string(),
                                "CID Chain Coherence".to_string(),
                                "Verified State Transitions".to_string(),
                                "Algebraic Message Routing".to_string(),
                            ],
                        },
                        SageCapability {
                            emoji: "🧠".to_string(),
                            title: "Distributed Intelligence".to_string(),
                            points: vec![
                                "Zero Single Points of Failure".to_string(),
                                "Auto-Healing Architecture".to_string(),
                                "Multi-Agent Orchestration".to_string(),
                                "Event-Sourced Truth".to_string(),
                            ],
                        },
                        SageCapability {
                            emoji: "🚀".to_string(),
                            title: "Development Acceleration".to_string(),
                            points: vec![
                                "Proactive Guidance".to_string(),
                                "Living Documentation".to_string(),
                                "Compliance by Design".to_string(),
                                "Context Preservation".to_string(),
                            ],
                        },
                        SageCapability {
                            emoji: "💡".to_string(),
                            title: "Key Capabilities".to_string(),
                            points: vec![
                                "Universal LLM Adapter".to_string(),
                                "Hot-Swappable Agents".to_string(),
                                "TEA-ECS Bridge".to_string(),
                                "Graph Architecture".to_string(),
                            ],
                        },
                        SageCapability {
                            emoji: "🎯".to_string(),
                            title: "Real-World Impact".to_string(),
                            points: vec![
                                "No More 'Works on My Machine'".to_string(),
                                "Resource-Optimal Solutions".to_string(),
                                "Domain Certainty".to_string(),
                                "Audit-Ready Systems".to_string(),
                            ],
                        },
                    ],
                },
                background: PresentationTheme::sage_gradient(),
            },
        ];

        (
            Self {
                current_slide: 0,
                slides,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Cowboy AI - Presentation")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NextSlide => {
                if self.current_slide < self.slides.len() - 1 {
                    self.current_slide += 1;
                }
            }
            Message::PreviousSlide => {
                if self.current_slide > 0 {
                    self.current_slide -= 1;
                }
            }
            Message::GoToSlide(index) => {
                if index < self.slides.len() {
                    self.current_slide = index;
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let slide = &self.slides[self.current_slide];
        
        let content = match &slide.content {
            SlideContent::Hero { title, subtitle, description, warning } => {
                column![
                    text(title).size(60),
                    Space::with_height(20),
                    text(subtitle).size(30),
                    Space::with_height(20),
                    text(description).size(20),
                    Space::with_height(40),
                    container(text(warning).size(18))
                        .padding(20)
                        .style(iced::theme::Container::Box),
                ]
            }
            SlideContent::Cognition { items, description } => {
                let mut col = column![
                    text(&slide.title).size(50),
                ];
                
                if let Some(subtitle) = &slide.subtitle {
                    col = col.push(text(subtitle).size(24));
                }
                
                col = col.push(Space::with_height(30));
                
                let mut item_row = row![];
                for item in items {
                    item_row = item_row.push(
                        column![
                            text(format!("{}", item.number)).size(40),
                            text(&item.title).size(20),
                            text(&item.subtitle).size(16),
                        ]
                        .spacing(10)
                        .align_items(Alignment::Center)
                    );
                    item_row = item_row.push(Space::with_width(40));
                }
                
                col = col.push(item_row);
                col = col.push(Space::with_height(30));
                col = col.push(text(description).size(18));
                
                col
            }
            SlideContent::Problems { cards } => {
                let mut col = column![
                    text(&slide.title).size(50),
                    Space::with_height(30),
                ];
                
                for card in cards {
                    col = col.push(
                        container(
                            column![
                                row![
                                    text(&card.icon).size(30),
                                    Space::with_width(10),
                                    text(&card.title).size(22),
                                ],
                                text(&card.description).size(18),
                            ]
                            .spacing(10)
                        )
                        .padding(20)
                        .style(iced::theme::Container::Box)
                    );
                    col = col.push(Space::with_height(20));
                }
                
                col
            }
            SlideContent::Platform { features } => {
                let mut col = column![
                    text(&slide.title).size(50),
                ];
                
                if let Some(subtitle) = &slide.subtitle {
                    col = col.push(text(subtitle).size(24));
                }
                
                col = col.push(Space::with_height(30));
                
                for feature in features {
                    col = col.push(
                        row![
                            text(format!("{}", feature.number)).size(30),
                            Space::with_width(20),
                            column![
                                text(&feature.title).size(20),
                                text(&feature.description).size(16),
                            ],
                        ]
                    );
                    col = col.push(Space::with_height(15));
                }
                
                col
            }
            SlideContent::Stack { components } => {
                let mut col = column![
                    text(&slide.title).size(50),
                    Space::with_height(30),
                ];
                
                let mut comp_row = row![];
                for component in components {
                    comp_row = comp_row.push(
                        column![
                            text(format!("{}", component.number)).size(40),
                            text(&component.title).size(20),
                            text(&component.subtitle).size(16),
                        ]
                        .spacing(10)
                        .align_items(Alignment::Center)
                    );
                    comp_row = comp_row.push(Space::with_width(40));
                }
                
                col = col.push(comp_row);
                col
            }
            SlideContent::Breakthrough { items } => {
                let mut col = column![
                    text(&slide.title).size(50),
                    Space::with_height(30),
                ];
                
                let mut grid = column![];
                let mut current_row = row![];
                
                for (i, item) in items.iter().enumerate() {
                    current_row = current_row.push(
                        container(
                            column![
                                text(&item.icon).size(30),
                                text(&item.title).size(20),
                                text(&item.description).size(16),
                            ]
                            .spacing(10)
                        )
                        .padding(15)
                        .style(iced::theme::Container::Box)
                    );
                    current_row = current_row.push(Space::with_width(20));
                    
                    if (i + 1) % 3 == 0 {
                        grid = grid.push(current_row);
                        grid = grid.push(Space::with_height(20));
                        current_row = row![];
                    }
                }
                
                // Add remaining row if it has content
                grid = grid.push(current_row);
                
                col = col.push(grid);
                col
            }
            SlideContent::Trust { sections } => {
                let mut col = column![
                    text(&slide.title).size(50),
                ];
                
                if let Some(subtitle) = &slide.subtitle {
                    col = col.push(text(subtitle).size(24));
                }
                
                col = col.push(Space::with_height(30));
                
                let mut grid = column![];
                let mut current_row = row![];
                
                for (i, section) in sections.iter().enumerate() {
                    let mut section_col = column![
                        row![
                            text(&section.icon).size(25),
                            Space::with_width(10),
                            text(&section.title).size(18),
                        ],
                    ];
                    
                    for item in &section.items {
                        section_col = section_col.push(text(format!("• {}", item)).size(14));
                    }
                    
                    current_row = current_row.push(
                        container(section_col.spacing(8))
                            .padding(15)
                            .style(iced::theme::Container::Box)
                    );
                    current_row = current_row.push(Space::with_width(20));
                    
                    if (i + 1) % 2 == 0 {
                        grid = grid.push(current_row);
                        grid = grid.push(Space::with_height(20));
                        current_row = row![];
                    }
                }
                
                // Add remaining row if it has content
                grid = grid.push(current_row);
                
                col = col.push(grid);
                col
            }
            SlideContent::Sage { capabilities } => {
                let mut col = column![
                    text(&slide.title).size(60),
                ];
                
                if let Some(subtitle) = &slide.subtitle {
                    col = col.push(text(subtitle).size(20));
                }
                
                col = col.push(Space::with_height(30));
                
                let mut grid = column![];
                let mut current_row = row![];
                
                for (i, cap) in capabilities.iter().enumerate() {
                    let mut cap_col = column![
                        text(&cap.emoji).size(40),
                        text(&cap.title).size(18),
                    ];
                    
                    for point in &cap.points {
                        cap_col = cap_col.push(text(format!("• {}", point)).size(14));
                    }
                    
                    current_row = current_row.push(
                        container(cap_col.spacing(8))
                            .padding(15)
                            .style(iced::theme::Container::Box)
                            .width(Length::FillPortion(1))
                    );
                    current_row = current_row.push(Space::with_width(20));
                    
                    if (i + 1) % 3 == 0 {
                        grid = grid.push(current_row);
                        grid = grid.push(Space::with_height(20));
                        current_row = row![];
                    }
                }
                
                // Add remaining row if it has content
                grid = grid.push(current_row);
                
                col = col.push(scrollable(grid));
                col
            }
        };
        
        let navigation = row![
            button(text("Previous")).on_press(Message::PreviousSlide),
            Space::with_width(20),
            text(format!("Slide {} of {}", self.current_slide + 1, self.slides.len())).size(18),
            Space::with_width(20),
            button(text("Next")).on_press(Message::NextSlide),
        ]
        .align_items(Alignment::Center);
        
        let slide_indicators = {
            let mut indicators = row![];
            for i in 0..self.slides.len() {
                let indicator = if i == self.current_slide {
                    button(text(format!("{}", i + 1)))
                        .on_press(Message::GoToSlide(i))
                        .style(iced::theme::Button::Primary)
                } else {
                    button(text(format!("{}", i + 1)))
                        .on_press(Message::GoToSlide(i))
                        .style(iced::theme::Button::Secondary)
                };
                indicators = indicators.push(indicator);
                indicators = indicators.push(Space::with_width(5));
            }
            indicators
        };
        
        container(
            column![
                content
                    .spacing(20)
                    .align_items(Alignment::Center),
                Space::with_height(40),
                navigation,
                Space::with_height(20),
                slide_indicators,
            ]
            .spacing(10)
            .align_items(Alignment::Center)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .padding(0)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }
}