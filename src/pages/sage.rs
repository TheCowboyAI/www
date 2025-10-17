use iced::{Element, Font, Length, Alignment};
use iced::widget::{column, container, text, row, vertical_space, horizontal_space};
use crate::theme;
use crate::Message;

pub fn view<'a>() -> Element<'a, Message> {
    let content = container(
        column![
            vertical_space(60),
            text("SAGE")
                .size(56)
                .font(Font::BOLD)
                .style(theme::text::heading),
            vertical_space(10),
            text("Universal Development Intelligence Platform")
                .size(28)
                .style(theme::text::accent),
            vertical_space(10),
            text("Orchestrating 100+ Specialized AI Experts from Silicon to Solution")
                .size(20)
                .style(theme::text::secondary),
            vertical_space(60),
            
            // Key features
            sage_section(
                "🎯 Complete Stack Mastery",
                vec![
                    "Nix Expert: Kernel to cloud expertise with deterministic, reproducible builds",
                    "Resource Expert: Real-world aware - optimizes for actual memory, CPU, bandwidth constraints",
                    "Domain & DDD Experts: Extract business essence through Event Storming and mathematical boundaries",
                    "Infrastructure Experts: Network topology, NATS messaging, distributed systems architecture",
                ]
            ),
            
            sage_section(
                "🔬 Mathematical Foundation",
                vec![
                    "Provable Correctness: Category Theory and Graph Theory backed decisions",
                    "CID Chain Coherence: Cryptographically linked, immutable audit trail via IPLD",
                    "Verified State Transitions: Mathematically guaranteed system invariants",
                    "Algebraic Message Routing: Subject hierarchies optimized with mathematical precision",
                ]
            ),
            
            sage_section(
                "🌐 Distributed Intelligence",
                vec![
                    "Zero Single Points of Failure: NATS JetStream based central nervous system",
                    "Auto-Healing Architecture: Lose a node or datacenter - system adapts instantly",
                    "Multi-Agent Orchestration: Symphonic coordination of specialized expertise",
                    "Event-Sourced Truth: Every decision recorded, every state reversible",
                ]
            ),
            
            sage_section(
                "🚀 Development Acceleration",
                vec![
                    "Proactive Guidance: Anticipates needs, prevents architectural mistakes",
                    "Living Documentation: BDD/TDD experts ensure specs, tests, and docs stay synchronized",
                    "Compliance by Design: HIPAA, PCI DSS, GDPR built-in, not bolted-on",
                    "Context Preservation: Seamless conversations across sessions and agents",
                ]
            ),
            
            vertical_space(40),
            
            // The SAGE Difference
            container(
                column![
                    text("🧠 The SAGE Difference")
                        .size(28)
                        .font(Font::BOLD)
                        .style(theme::text::accent),
                    vertical_space(20),
                    text("• 100+ Specialized AI Agent Experts scaling to billions")
                        .size(18)
                        .style(theme::text::primary),
                    text("• Mathematical Proofs not promises")
                        .size(18)
                        .style(theme::text::primary),
                    text("• Distributed Resilience not single points of failure")
                        .size(18)
                        .style(theme::text::primary),
                    text("• Orchestrated Intelligence not isolated tools. Like small Lego Blocks")
                        .size(18)
                        .style(theme::text::primary),
                    text("• Domain Mastery not generic solutions")
                        .size(18)
                        .style(theme::text::primary),
                    vertical_space(20),
                    text("Transform your development with orchestrated intelligence.")
                        .size(20)
                        .font(Font::BOLD)
                        .style(theme::text::secondary),
                    text("Deploy with confidence. Scale with certainty. Evolve with intelligence.")
                        .size(18)
                        .style(theme::text::secondary),
                ]
                .align_x(Alignment::Center)
            )
            .padding(40)
            .width(Length::Fill)
            .style(theme::container::glass_card),
            
            vertical_space(80),
        ]
        .align_x(Alignment::Center)
        .width(Length::Fill)
    )
    .padding(40)
    .center_x(Length::Fill);
    
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::container::dark_background)
        .into()
}

fn sage_section<'a>(title: &str, items: Vec<&str>) -> Element<'a, Message> {
    let mut col = column![
        text(title)
            .size(24)
            .font(Font::BOLD)
            .style(theme::text::primary),
        vertical_space(15),
    ];
    
    for item in items {
        col = col.push(
            container(
                text(format!("• {}", item))
                    .size(16)
                    .style(theme::text::secondary)
            )
            .padding([5, 0])
        );
    }
    
    container(col)
        .padding(30)
        .width(Length::Fill)
        .style(theme::container::card)
        .into()
}