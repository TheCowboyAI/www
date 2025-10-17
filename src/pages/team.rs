use iced::{Element, Font, Length, Alignment};
use iced::widget::{column, container, text, row, vertical_space, horizontal_space};
use crate::theme;
use crate::Message;

pub fn view<'a>() -> Element<'a, Message> {
    let content = container(
        column![
            vertical_space(60),
            text("Our Leadership Team")
                .size(48)
                .font(Font::BOLD)
                .style(theme::text::heading),
            vertical_space(40),
            
            // Executive team
            row![
                team_member(
                    "Jacob Kopilovitch",
                    "CEO",
                    "Serial entrepreneur with over 20 years of experience driving strategic vision, business growth, and organizational leadership across multiple industries."
                ),
                horizontal_space(20),
                team_member(
                    "David Kopilovitch",
                    "COO",
                    "Operations executive with 17+ years of expertise in back office management, process optimization, and detailed execution of complex business operations."
                ),
            ]
            .align_y(Alignment::Start),
            vertical_space(20),
            
            row![
                team_member(
                    "Steele Price",
                    "CSO",
                    "Technology visionary with 40+ years of experience, including 10 years as Microsoft MVP and core team member for .NET language development."
                ),
                horizontal_space(20),
                team_member(
                    "Kathryn Freeman",
                    "CBO",
                    "Accomplished entrepreneur and senior advisor with 17+ years guiding fintech and lending companies."
                ),
            ]
            .align_y(Alignment::Start),
            vertical_space(20),
            
            row![
                team_member(
                    "Mark Sonnenklar",
                    "CLO",
                    "Premier business and intellectual property attorney with 25+ years of experience in corporate and private law across the West Coast."
                ),
                horizontal_space(20),
                team_member(
                    "Ryan Plemons",
                    "Senior Architect & AI Lead",
                    "Veteran technology leader with 30+ years as a senior engineer, specializing in AI implementation and product development strategy."
                ),
            ]
            .align_y(Alignment::Start),
            vertical_space(20),
            
            // Technical team
            text("Technical Team")
                .size(32)
                .font(Font::BOLD)
                .style(theme::text::heading),
            vertical_space(20),
            
            row![
                team_member(
                    "Blake",
                    "Graphic Design",
                    "Creative technologist with 12 years of experience spanning UI/UX design, Web Development, Marketing & Branding."
                ),
                horizontal_space(20),
                team_member(
                    "Daniel",
                    "Security Engineer",
                    "Cybersecurity specialist with 15 years of experience in security infrastructure, PKI implementations, and Linux/NixOS systems administration."
                ),
            ]
            .align_y(Alignment::Start),
            vertical_space(20),
            
            row![
                team_member(
                    "Jonathan Lee",
                    "DevOps Engineer",
                    "Infrastructure specialist with 11 years of experience optimizing deployment pipelines, cloud architecture, and seamless integration."
                ),
                horizontal_space(20),
                team_member(
                    "Kaleb Masterson",
                    "Security & AI Intern",
                    "Emerging talent focused on the integration of security protocols with AI systems."
                ),
            ]
            .align_y(Alignment::Start),
            vertical_space(20),
            
            row![
                team_member(
                    "Jaxson Aubert",
                    "Sales Intern",
                    "Rising talent with 3 years of software sales experience, specializing in AI functionality demonstrations."
                ),
                horizontal_space(20),
                team_member(
                    "Gal Bareket",
                    "Senior Advisor",
                    "Accomplished serial entrepreneur with multiple successful exits, serving as a strategic advisor to numerous fintech companies."
                ),
            ]
            .align_y(Alignment::Start),
            
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

fn team_member<'a>(name: &str, role: &str, bio: &str) -> Element<'a, Message> {
    container(
        column![
            text(name)
                .size(20)
                .font(Font::BOLD)
                .style(theme::text::primary),
            vertical_space(5),
            text(role)
                .size(16)
                .style(theme::text::accent),
            vertical_space(10),
            text(bio)
                .size(14)
                .style(theme::text::secondary),
        ]
    )
    .padding(25)
    .width(Length::FillPortion(1))
    .height(Length::Fixed(200.0))
    .style(theme::container::card)
    .into()
}