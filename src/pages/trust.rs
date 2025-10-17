use iced::{Element, Font, Length, Alignment};
use iced::widget::{column, container, text, row, vertical_space};
use crate::theme;
use crate::Message;

pub fn view<'a>() -> Element<'a, Message> {
    let content = container(
        column![
            vertical_space(60),
            text("Trust by Design:")
                .size(48)
                .font(Font::BOLD)
                .style(theme::text::heading),
            vertical_space(10),
            text("Security, Control, and Explainability at the Core")
                .size(28)
                .style(theme::text::accent),
            vertical_space(20),
            text("Cowboy AI delivers secure-by-default automation through a modern, composable architecture engineered for financial-grade integrity and adaptability—on your terms.")
                .size(18)
                .style(theme::text::secondary),
            vertical_space(60),
            
            // Security sections
            security_section(
                "🏗️ Infrastructure of Integrity",
                vec![
                    "Apple M3 Ultra + NixDarwin for deterministic environments",
                    "Rust + Bevy Runtime for memory-safe orchestration",
                    "NATS Messaging for real-time, fault-tolerant communication",
                ]
            ),
            vertical_space(20),
            
            security_section(
                "🔐 Security That Scales",
                vec![
                    "YubiKey Hardware Auth + ECC Encryption for transaction-level trust",
                    "MinIO + Wasabi providing immutable, ransomware-resistant storage",
                    "AI Model Agnostic -Compatible for privacy-preserving AI deployment, local and cloud",
                ]
            ),
            vertical_space(20),
            
            security_section(
                "🧠 Explainable Intelligence",
                vec![
                    "Neo4j Graph Engine + Event Sourcing for auditable decisions",
                    "Functional Programming + TDD ensuring provable correctness",
                    "DDD + Conceptual Spaces aligning systems to business intent",
                ]
            ),
            vertical_space(20),
            
            security_section(
                "📐 Mathematics of Trust",
                vec![
                    "Category Theory & Graph Theory ensuring formal correctness",
                    "Gradient Descent Programming for optimized agent decisions",
                    "Trust proven line by line, decision by decision",
                ]
            ),
            
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

fn security_section<'a>(title: &str, items: Vec<&str>) -> Element<'a, Message> {
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
            .padding([5, 20])
        );
    }
    
    container(col)
        .padding(30)
        .width(Length::Fill)
        .style(theme::container::card)
        .into()
}