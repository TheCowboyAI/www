use iced::{Element, Font, Length, Alignment};
use iced::widget::{column, container, text, row, button, vertical_space, horizontal_space};
use crate::theme;

#[derive(Debug, Clone)]
pub enum Message {
    GetStarted,
    LearnMore,
}

#[derive(Default)]
pub struct State {
    // Hero state if needed
}

impl State {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::GetStarted => {
                // Handle get started action
            }
            Message::LearnMore => {
                // Handle learn more action
            }
        }
    }
}

pub fn view<'a>(state: &State) -> Element<'a, Message> {
    let hero_section = container(
        column![
            vertical_space(100),
            text("Cowboy AI")
                .size(72)
                .font(Font::BOLD)
                .style(theme::text::heading),
            vertical_space(20),
            text("The Platform For Composable, Cognitive, Audit-Grade AI Swarms")
                .size(32)
                .style(theme::text::accent),
            vertical_space(10),
            text("Your New Business Brain & Nervous System. Cognition. Evolution. Security. Trust.")
                .size(20)
                .style(theme::text::secondary),
            vertical_space(30),
            text("No-Code Composable Multi-AI Agent Orchestration At Scale")
                .size(24)
                .font(Font::BOLD)
                .style(theme::text::primary),
            vertical_space(40),
            row![
                button(text("Get Started").size(18))
                    .padding([15, 40])
                    .style(theme::button::primary)
                    .on_press(Message::GetStarted),
                horizontal_space(20),
                button(text("Learn More").size(18))
                    .padding([15, 40])
                    .style(theme::button::secondary)
                    .on_press(Message::LearnMore),
            ]
            .align_y(Alignment::Center),
            vertical_space(60),
            container(
                column![
                    text("Disclaimer & Warning:")
                        .size(16)
                        .font(Font::BOLD)
                        .style(theme::text::accent),
                    vertical_space(10),
                    text("You can now ask and command the system to do anything. In plain English. And it will.")
                        .size(18)
                        .style(theme::text::secondary),
                ]
                .align_x(Alignment::Center)
            )
            .padding(30)
            .style(theme::container::glass_card)
            .max_width(800),
            vertical_space(100),
        ]
        .align_x(Alignment::Center)
        .width(Length::Fill)
    )
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .padding(40);
    
    container(hero_section)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::container::dark_background)
        .into()
}