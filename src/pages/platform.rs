use iced::{Element, Font, Length, Alignment};
use iced::widget::{column, container, text, row, vertical_space, horizontal_space};
use crate::theme;
use crate::Message;

pub fn view<'a>() -> Element<'a, Message> {
    let content = container(
        column![
            vertical_space(60),
            text("Platform First — Cowboy AI CIM")
                .size(48)
                .font(Font::BOLD)
                .style(theme::text::heading),
            vertical_space(20),
            text("The platform for building, governing, and scaling your business with AI swarms.")
                .size(24)
                .style(theme::text::secondary),
            vertical_space(60),
            
            // Platform features grid
            row![
                feature_card(
                    "Build Once, Replicate Anywhere",
                    "We're proving it in private lending/fintech first.\n\nRepeatable & Reproduceable.",
                    "📊"
                ),
                horizontal_space(20),
                feature_card(
                    "Open for Builders",
                    "Publish domain packs, swarms, and connectors other teams can adopt.",
                    "🔧"
                ),
            ]
            .align_y(Alignment::Start),
            vertical_space(20),
            row![
                feature_card(
                    "Proven Milestones",
                    "8M+ micro-transactions processed; hybrid runtime targeting >90% lower AI compute vs cloud-only baselines.",
                    "📈"
                ),
                horizontal_space(20),
                feature_card(
                    "No-code, modular agent swarms",
                    "Like little building blocks AI agents assemble and compose on the fly to execute any task.",
                    "🧩"
                ),
            ]
            .align_y(Alignment::Start),
            
            vertical_space(40),
            
            // Security section
            container(
                column![
                    text("🔒 Immutable, Security First")
                        .size(28)
                        .font(Font::BOLD)
                        .style(theme::text::accent),
                    vertical_space(10),
                    text("Immutable records, ransomware-resistant architecture")
                        .size(18)
                        .style(theme::text::secondary),
                ]
                .align_x(Alignment::Center)
            )
            .padding(30)
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

fn feature_card<'a>(icon: &str, title: &str, description: &str) -> Element<'a, Message> {
    container(
        column![
            text(icon)
                .size(40),
            vertical_space(15),
            text(title)
                .size(20)
                .font(Font::BOLD)
                .style(theme::text::primary),
            vertical_space(10),
            text(description)
                .size(16)
                .style(theme::text::secondary),
        ]
        .align_x(Alignment::Center)
    )
    .padding(30)
    .width(Length::FillPortion(1))
    .height(Length::Fixed(250.0))
    .style(theme::container::card)
    .into()
}