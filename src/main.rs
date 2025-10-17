use iced::Element;
use iced::widget::{button, column, container, text, scrollable, row, Space};

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
    Platform,
    Team, 
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
        let header = container(
            row![
                text("Cowboy AI").size(24),
                Space::with_width(iced::Fill),
                button(text("Home").size(14))
                    .on_press(Message::NavigateTo(Page::Hero))
                    .padding([8, 16]),
                button(text("Platform").size(14))
                    .on_press(Message::NavigateTo(Page::Platform))
                    .padding([8, 16]),
                button(text("Team").size(14))
                    .on_press(Message::NavigateTo(Page::Team))
                    .padding([8, 16]),
            ]
            .padding(20)
            .align_y(iced::Alignment::Center)
        )
        .width(iced::Fill);
        
        let content = match self.current_page {
            Page::Hero => self.hero_page(),
            Page::Platform => self.platform_page(),
            Page::Team => self.team_page(),
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

    fn hero_page(&self) -> Element<Message> {
        container(
            column![
                Space::with_height(100),
                text("Cowboy AI").size(72),
                Space::with_height(20),
                text("The Platform For Composable, Cognitive, Audit-Grade AI Swarms").size(32),
                Space::with_height(10),
                text("Your New Business Brain & Nervous System").size(20),
                Space::with_height(30),
                text("No-Code Composable Multi-AI Agent Orchestration At Scale").size(24),
                Space::with_height(60),
                container(
                    column![
                        text("Disclaimer & Warning:").size(16),
                        Space::with_height(10),
                        text("You can now ask and command the system to do anything. In plain English. And it will.").size(18),
                    ]
                    .align_x(iced::Alignment::Center)
                )
                .padding(30)
                .max_width(800),
                Space::with_height(100),
            ]
            .align_x(iced::Alignment::Center)
            .width(iced::Fill)
        )
        .padding(40)
        .center_x(iced::Fill)
        .into()
    }

    fn platform_page(&self) -> Element<Message> {
        container(
            column![
                Space::with_height(60),
                text("Platform First — Cowboy AI CIM").size(48),
                Space::with_height(20),
                text("The platform for building, governing, and scaling your business with AI swarms.").size(24),
                Space::with_height(60),
                text("Build Once, Replicate Anywhere").size(24),
                text("We're proving it in private lending/fintech first. Repeatable & Reproduceable.").size(16),
                Space::with_height(20),
                text("Open for Builders").size(24),
                text("Publish domain packs, swarms, and connectors other teams can adopt.").size(16),
                Space::with_height(20),
                text("Proven Milestones").size(24),
                text("8M+ micro-transactions processed; >90% lower AI compute vs cloud-only baselines.").size(16),
                Space::with_height(20),
                text("No-code, modular agent swarms").size(24),
                text("Like little building blocks AI agents assemble and compose on the fly to execute any task.").size(16),
                Space::with_height(40),
                text("Immutable, Security First").size(28),
                text("Immutable records, ransomware-resistant architecture").size(18),
                Space::with_height(80),
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
                Space::with_height(60),
                text("Our Leadership Team").size(48),
                Space::with_height(40),
                text("Jacob Kopilovitch - CEO").size(20),
                text("Serial entrepreneur with over 20 years of experience").size(16),
                Space::with_height(15),
                text("David Kopilovitch - COO").size(20),
                text("Operations executive with 17+ years of expertise").size(16),
                Space::with_height(15),
                text("Steele Price - CSO").size(20),
                text("Technology visionary with 40+ years of experience, Microsoft MVP").size(16),
                Space::with_height(15),
                text("Kathryn Freeman - CBO").size(20),
                text("Accomplished entrepreneur and senior advisor").size(16),
                Space::with_height(80),
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