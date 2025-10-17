use iced::{Element, Font, Length, Alignment};
use iced::widget::{column, container, text, row, vertical_space, horizontal_space};
use crate::theme;
use crate::Message;

pub fn view<'a>() -> Element<'a, Message> {
    let content = container(
        column![
            vertical_space(60),
            text("No Cognition? - No AGI!")
                .size(48)
                .font(Font::BOLD)
                .style(theme::text::heading),
            vertical_space(20),
            text("Intelligence Alone Isn't Enough—Cognition Is the Missing Piece.")
                .size(24)
                .style(theme::text::secondary),
            vertical_space(40),
            
            // Four quadrants
            row![
                quadrant("1", "Hardware", "Horsepower", theme::colors::SECONDARY),
                horizontal_space(20),
                quadrant("2", "LLM", "Raw tooling", theme::colors::SECONDARY),
            ]
            .align_y(Alignment::Center),
            vertical_space(20),
            row![
                quadrant("4", "Your World - The Data", "Clean, Holistic, Accessible, Mathematically Proven", theme::colors::PRIMARY),
                horizontal_space(20),
                quadrant("3", "Cognition", "Awareness of tools & State", theme::colors::PRIMARY),
            ]
            .align_y(Alignment::Center),
            
            vertical_space(60),
            container(
                column![
                    text("For true AGI, systems must learn, adapt, and evolve—not just process data.")
                        .size(20)
                        .style(theme::text::primary),
                    vertical_space(20),
                    text("Today's AI is trapped in silos, built on imperfect math, with no structure or guardrails.")
                        .size(18)
                        .style(theme::text::secondary),
                    vertical_space(10),
                    text("That's why it keeps failing.")
                        .size(18)
                        .style(theme::text::secondary),
                    vertical_space(30),
                    text("We solved it.")
                        .size(24)
                        .font(Font::BOLD)
                        .style(theme::text::accent),
                    vertical_space(20),
                    text("By embedding cognition at the core, Cowboy AI creates an adaptive, composable architecture that continuously learns, evolves, and safeguards decision-making.")
                        .size(18)
                        .style(theme::text::primary),
                    vertical_space(30),
                    text("This is the breakthrough that makes AGI possible.")
                        .size(22)
                        .font(Font::BOLD)
                        .style(theme::text::accent),
                ]
                .align_x(Alignment::Center)
            )
            .padding(40)
            .max_width(900)
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

fn quadrant<'a>(number: &str, title: &str, description: &str, color: iced::Color) -> Element<'a, Message> {
    container(
        column![
            text(number)
                .size(36)
                .font(Font::BOLD)
                .color(color),
            vertical_space(10),
            text(title)
                .size(22)
                .font(Font::BOLD)
                .style(theme::text::primary),
            vertical_space(5),
            text(description)
                .size(16)
                .style(theme::text::secondary),
        ]
        .align_x(Alignment::Center)
    )
    .padding(30)
    .width(Length::FillPortion(1))
    .style(theme::container::card)
    .into()
}