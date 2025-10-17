use iced::{Element, Font, Length, Alignment};
use iced::widget::{column, container, text, row, vertical_space, horizontal_space};
use crate::theme;
use crate::Message;

pub fn view<'a>() -> Element<'a, Message> {
    let content = container(
        column![
            vertical_space(60),
            text("Roadmap")
                .size(48)
                .font(Font::BOLD)
                .style(theme::text::heading),
            vertical_space(60),
            
            // Next 6-12 months
            roadmap_section(
                "Next 6-12 months: Building the Foundation",
                vec![
                    "Scale Private Lending Market Expansion",
                    "Launch Partner Program",
                    "Expand into adjacent FinTech operations",
                ]
            ),
            vertical_space(40),
            
            // 12-18 months
            roadmap_section(
                "12-18 months: Scaling and Expansion",
                vec![
                    "Secure Second Funding Round & Cultivate Strategic Vertical Partners",
                    "Expand Into Future Verticals: M&A, Robotics, Legal, Supply Chain",
                    "Curate Developer Marketplace",
                ]
            ),
            vertical_space(60),
            
            text("Each milestone strengthens our position to dominate the AI orchestration market with our significant head start.")
                .size(20)
                .style(theme::text::secondary),
            vertical_space(60),
            
            // Strategic Growth
            text("Strategic Growth Roadmap")
                .size(36)
                .font(Font::BOLD)
                .style(theme::text::heading),
            vertical_space(30),
            
            growth_section(
                "👑 Dominate Private Lending Market",
                "Establish market leadership within 12-18 months by onboarding 25 strategic customers and positioning for a major vertical partnership or exit in growth stage"
            ),
            vertical_space(20),
            
            growth_section(
                "💰 Generate Early Investor Returns",
                "Create liquidity events to reward initial investors while strategically reinvesting capital to fuel cross-vertical platform expansion"
            ),
            vertical_space(20),
            
            growth_section(
                "📈 Scale Across High-Growth Verticals",
                ""
            ),
            vertical_space(10),
            
            // Verticals list
            vertical_list(),
            
            vertical_space(40),
            
            container(
                text("Our vision is to become the definitive platform for AI Agent Orchestration, delivering transformative solutions wherever fragmented systems and repetitive processes are inhibiting innovation and growth.")
                    .size(18)
                    .style(theme::text::primary)
            )
            .padding(30)
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

fn roadmap_section<'a>(title: &str, items: Vec<&str>) -> Element<'a, Message> {
    let mut col = column![
        text(title)
            .size(28)
            .font(Font::BOLD)
            .style(theme::text::primary),
        vertical_space(20),
    ];
    
    for item in items {
        col = col.push(
            container(
                text(format!("• {}", item))
                    .size(18)
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

fn growth_section<'a>(title: &str, description: &str) -> Element<'a, Message> {
    container(
        column![
            text(title)
                .size(24)
                .font(Font::BOLD)
                .style(theme::text::primary),
            vertical_space(10),
            text(description)
                .size(16)
                .style(theme::text::secondary),
        ]
    )
    .padding(25)
    .width(Length::Fill)
    .style(theme::container::card)
    .into()
}

fn vertical_list<'a>() -> Element<'a, Message> {
    let verticals = vec![
        ("Real Estate & Tokenized Securities", "$215.2B market (2024) growing to $16T by 2030"),
        ("Industrial Robotics", "$16.5B market (2023) expanding to $163B by 2030"),
        ("Personal Robotics", "$28B market (2023) reaching $100B by 2032"),
        ("M&A Integration Solutions", "$8B market (2023) growing to $22B by 2031"),
        ("Social Media Advertising", "$80-95B projected ad spend by 2025"),
        ("Retail Automation", "Streamlining operations, supply chain management and enhancing customer experience"),
        ("Legal Process Optimization", "Transforming document management, case analysis and predictive legal tools"),
    ];
    
    let mut col = column![];
    
    for (name, desc) in verticals {
        col = col.push(
            container(
                row![
                    text(format!("• {}", name))
                        .size(16)
                        .font(Font::BOLD)
                        .style(theme::text::primary),
                    horizontal_space(10),
                    text(desc)
                        .size(14)
                        .style(theme::text::secondary),
                ]
            )
            .padding([5, 20])
        );
    }
    
    container(col)
        .padding(20)
        .width(Length::Fill)
        .style(theme::container::card)
        .into()
}