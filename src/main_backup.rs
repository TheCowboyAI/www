use iced::Element;
use iced::widget::{button, column, container, text, scrollable, row, vertical_space, horizontal_space};

pub fn main() -> iced::Result {
    iced::application("Cowboy AI - Cognitive Orchestration Platform", App::update, App::view)
        .theme(App::theme)
        .window_size((1400.0, 900.0))
        .run()
}

#[derive(Debug, Clone)]
pub enum Message {
    NavigateTo(Page),
}

#[derive(Default)]
struct App {
    current_page: Page,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Page {
    #[default]
    Hero,
    Cognition,
    AiBreaking,
    Platform,
    Sage,
    Trust,
    Team, 
    Roadmap,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::NavigateTo(page) => {
                self.current_page = page;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let header = self.header();
        
        let content = match self.current_page {
            Page::Hero => self.hero_page(),
            Page::Cognition => self.cognition_page(),
            Page::AiBreaking => self.ai_breaking_page(),
            Page::Platform => self.platform_page(),
            Page::Sage => self.sage_page(),
            Page::Trust => self.trust_page(),
            Page::Team => self.team_page(),
            Page::Roadmap => self.roadmap_page(),
        };

        column![
            header,
            scrollable(content)
                .height(iced::Fill)
                .width(iced::Fill)
        ]
        .spacing(0)
        .into()
    }
    
    fn header(&self) -> Element<Message> {
        let logo = text("Cowboy AI")
            .size(24)
;
        
        let nav_items = row![
            button(text("Home").size(14))
                .on_press(Message::NavigateTo(Page::Hero))
                .padding([8, 16]),
            button(text("Platform").size(14))
                .on_press(Message::NavigateTo(Page::Platform))
                .padding([8, 16]),
            button(text("Technology").size(14))
                .on_press(Message::NavigateTo(Page::Cognition))
                .padding([8, 16]),
            button(text("SAGE").size(14))
                .on_press(Message::NavigateTo(Page::Sage))
                .padding([8, 16]),
            button(text("Security").size(14))
                .on_press(Message::NavigateTo(Page::Trust))
                .padding([8, 16]),
            button(text("Team").size(14))
                .on_press(Message::NavigateTo(Page::Team))
                .padding([8, 16]),
            button(text("Roadmap").size(14))
                .on_press(Message::NavigateTo(Page::Roadmap))
                .padding([8, 16]),
        ]
        .spacing(10);
        
        container(
            row![
                logo,
                horizontal_space(),
                nav_items,
            ]
            .padding(20)
            .align_y(iced::Alignment::Center)
        )
        .width(iced::Fill)
        .into()
    }

    fn hero_page(&self) -> Element<Message> {
        container(
            column![
                vertical_space().\nheight(100),
                text("Cowboy AI")
                    .size(72)
        ,
                vertical_space().\nheight(20),
                text("The Platform For Composable, Cognitive, Audit-Grade AI Swarms")
                    .size(32),
                vertical_space().\nheight(10),
                text("Your New Business Brain & Nervous System")
                    .size(20),
                vertical_space().\nheight(30),
                text("No-Code Composable Multi-AI Agent Orchestration At Scale")
                    .size(24)
        ,
                vertical_space().\nheight(60),
                container(
                    column![
                        text("Disclaimer & Warning:")
                            .size(16)
                ,
                        vertical_space().\nheight(10),
                        text("You can now ask and command the system to do anything. In plain English. And it will.")
                            .size(18),
                    ]
                    .align_x(iced::Alignment::Center)
                )
                .padding(30)
                .max_width(800),
                vertical_space().\nheight(100),
            ]
            .align_x(iced::Alignment::Center)
            .width(iced::Fill)
        )
        .padding(40)
        .center_x(iced::Fill)
        .into()
    }

    fn cognition_page(&self) -> Element<Message> {
        container(
            column![
                vertical_space().\nheight(60),
                text("No Cognition? - No AGI!")
                    .size(48)
        ,
                vertical_space().\nheight(20),
                text("Intelligence Alone Isn't Enough—Cognition Is the Missing Piece.")
                    .size(24),
                vertical_space().\nheight(60),
                text("For true AGI, systems must learn, adapt, and evolve—not just process data.")
                    .size(20),
                vertical_space().\nheight(20),
                text("Today's AI is trapped in silos, built on imperfect math, with no structure or guardrails.")
                    .size(18),
                vertical_space().\nheight(10),
                text("That's why it keeps failing.")
                    .size(18),
                vertical_space().\nheight(30),
                text("We solved it.")
                    .size(24)
        ,
                vertical_space().\nheight(20),
                text("By embedding cognition at the core, Cowboy AI creates an adaptive, composable architecture")
                    .size(18),
                text("that continuously learns, evolves, and safeguards decision-making.")
                    .size(18),
                vertical_space().\nheight(30),
                text("This is the breakthrough that makes AGI possible.")
                    .size(22)
        ,
                vertical_space().\nheight(80),
            ]
            .align_x(iced::Alignment::Center)
            .width(iced::Fill)
        )
        .padding(40)
        .into()
    }

    fn ai_breaking_page(&self) -> Element<Message> {
        container(
            column![
                vertical_space().\nheight(60),
                text("AI Is Breaking at Scale")
                    .size(48)
        ,
                vertical_space().\nheight(40),
                text("Fragmentation kills velocity")
                    .size(24)
        ,
                text("Businesses juggle many applications & tools, re-enter data, and never achieve true sync.")
                    .size(16),
                vertical_space().\nheight(20),
                text("The last 10% explodes compute")
                    .size(24)
        ,
                text("Teams reach 85-90% automation, then costs and fragility spike without the right structures.")
                    .size(16),
                vertical_space().\nheight(20),
                text("Lock-in & opacity")
                    .size(24)
        ,
                text("Cloud-centric AI → runaway spend, brittle workflows, and audits no one can pass.")
                    .size(16),
                vertical_space().\nheight(80),
            ]
            .align_x(iced::Alignment::Center)
            .width(iced::Fill)
        )
        .padding(40)
        .into()
    }

    fn platform_page(&self) -> Element<Message> {
        container(
            column![
                vertical_space().\nheight(60),
                text("Platform First — Cowboy AI CIM")
                    .size(48)
        ,
                vertical_space().\nheight(20),
                text("The platform for building, governing, and scaling your business with AI swarms.")
                    .size(24),
                vertical_space().\nheight(60),
                text("📊 Build Once, Replicate Anywhere")
                    .size(24)
        ,
                text("We're proving it in private lending/fintech first. Repeatable & Reproduceable.")
                    .size(16),
                vertical_space().\nheight(20),
                text("🔧 Open for Builders")
                    .size(24)
        ,
                text("Publish domain packs, swarms, and connectors other teams can adopt.")
                    .size(16),
                vertical_space().\nheight(20),
                text("📈 Proven Milestones")
                    .size(24)
        ,
                text("8M+ micro-transactions processed; >90% lower AI compute vs cloud-only baselines.")
                    .size(16),
                vertical_space().\nheight(20),
                text("🧩 No-code, modular agent swarms")
                    .size(24)
        ,
                text("Like little building blocks AI agents assemble and compose on the fly to execute any task.")
                    .size(16),
                vertical_space().\nheight(40),
                text("🔒 Immutable, Security First")
                    .size(28)
        ,
                text("Immutable records, ransomware-resistant architecture")
                    .size(18),
                vertical_space().\nheight(80),
            ]
            .align_x(iced::Alignment::Center)
            .width(iced::Fill)
        )
        .padding(40)
        .into()
    }

    fn sage_page(&self) -> Element<Message> {
        container(
            column![
                vertical_space().\nheight(60),
                text("SAGE")
                    .size(56)
        ,
                text("Universal Development Intelligence Platform")
                    .size(28),
                text("Orchestrating 100+ Specialized AI Experts from Silicon to Solution")
                    .size(20),
                vertical_space().\nheight(60),
                text("🎯 Complete Stack Mastery")
                    .size(24)
        ,
                text("• Nix Expert: Kernel to cloud expertise")
                    .size(16),
                text("• Resource Expert: Real-world aware optimizations")
                    .size(16),
                text("• Domain & DDD Experts: Extract business essence")
                    .size(16),
                text("• Infrastructure Experts: Network topology, NATS messaging")
                    .size(16),
                vertical_space().\nheight(20),
                text("🔬 Mathematical Foundation")
                    .size(24)
        ,
                text("• Provable Correctness: Category Theory backed decisions")
                    .size(16),
                text("• CID Chain Coherence: Cryptographically linked audit trail")
                    .size(16),
                text("• Verified State Transitions: Mathematically guaranteed")
                    .size(16),
                vertical_space().\nheight(20),
                text("🌐 Distributed Intelligence")
                    .size(24)
        ,
                text("• Zero Single Points of Failure")
                    .size(16),
                text("• Auto-Healing Architecture")
                    .size(16),
                text("• Multi-Agent Orchestration")
                    .size(16),
                text("• Event-Sourced Truth")
                    .size(16),
                vertical_space().\nheight(80),
            ]
            .align_x(iced::Alignment::Center)
            .width(iced::Fill)
        )
        .padding(40)
        .into()
    }

    fn trust_page(&self) -> Element<Message> {
        container(
            column![
                vertical_space().\nheight(60),
                text("Trust by Design")
                    .size(48)
        ,
                text("Security, Control, and Explainability at the Core")
                    .size(28),
                vertical_space().\nheight(40),
                text("🏗️ Infrastructure of Integrity")
                    .size(24)
        ,
                text("• Apple M3 Ultra + NixDarwin for deterministic environments")
                    .size(16),
                text("• Rust + Bevy Runtime for memory-safe orchestration")
                    .size(16),
                text("• NATS Messaging for real-time, fault-tolerant communication")
                    .size(16),
                vertical_space().\nheight(20),
                text("🔐 Security That Scales")
                    .size(24)
        ,
                text("• YubiKey Hardware Auth + ECC Encryption")
                    .size(16),
                text("• MinIO + Wasabi providing immutable storage")
                    .size(16),
                text("• AI Model Agnostic for privacy-preserving AI deployment")
                    .size(16),
                vertical_space().\nheight(20),
                text("🧠 Explainable Intelligence")
                    .size(24)
        ,
                text("• Neo4j Graph Engine + Event Sourcing")
                    .size(16),
                text("• Functional Programming + TDD")
                    .size(16),
                text("• DDD + Conceptual Spaces")
                    .size(16),
                vertical_space().\nheight(80),
            ]
            .align_x(iced::Alignment::Center)
            .width(iced::Fill)
        )
        .padding(40)
        .into()
    }

    fn team_page(&self) -> Element<Message> {
        container(
            column![
                vertical_space().\nheight(60),
                text("Our Leadership Team")
                    .size(48)
        ,
                vertical_space().\nheight(40),
                text("Jacob Kopilovitch - CEO")
                    .size(20)
        ,
                text("Serial entrepreneur with over 20 years of experience")
                    .size(16),
                vertical_space().\nheight(15),
                text("David Kopilovitch - COO")
                    .size(20)
        ,
                text("Operations executive with 17+ years of expertise")
                    .size(16),
                vertical_space().\nheight(15),
                text("Steele Price - CSO")
                    .size(20)
        ,
                text("Technology visionary with 40+ years of experience, Microsoft MVP")
                    .size(16),
                vertical_space().\nheight(15),
                text("Kathryn Freeman - CBO")
                    .size(20)
        ,
                text("Accomplished entrepreneur and senior advisor")
                    .size(16),
                vertical_space().\nheight(15),
                text("Mark Sonnenklar - CLO")
                    .size(20)
        ,
                text("Premier business and intellectual property attorney")
                    .size(16),
                vertical_space().\nheight(15),
                text("Ryan Plemons - Senior Architect & AI Lead")
                    .size(20)
        ,
                text("Veteran technology leader with 30+ years as a senior engineer")
                    .size(16),
                vertical_space().\nheight(80),
            ]
            .align_x(iced::Alignment::Center)
            .width(iced::Fill)
        )
        .padding(40)
        .into()
    }

    fn roadmap_page(&self) -> Element<Message> {
        container(
            column![
                vertical_space().\nheight(60),
                text("Roadmap")
                    .size(48)
        ,
                vertical_space().\nheight(60),
                text("Next 6-12 months: Building the Foundation")
                    .size(28)
        ,
                text("• Scale Private Lending Market Expansion")
                    .size(18),
                text("• Launch Partner Program")
                    .size(18),
                text("• Expand into adjacent FinTech operations")
                    .size(18),
                vertical_space().\nheight(40),
                text("12-18 months: Scaling and Expansion")
                    .size(28)
        ,
                text("• Secure Second Funding Round & Strategic Partners")
                    .size(18),
                text("• Expand Into Future Verticals: M&A, Robotics, Legal, Supply Chain")
                    .size(18),
                text("• Curate Developer Marketplace")
                    .size(18),
                vertical_space().\nheight(60),
                text("Each milestone strengthens our position to dominate")
                    .size(20),
                text("the AI orchestration market with our significant head start.")
                    .size(20),
                vertical_space().\nheight(80),
            ]
            .align_x(iced::Alignment::Center)
            .width(iced::Fill)
        )
        .padding(40)
        .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}