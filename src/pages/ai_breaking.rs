use iced::{Element, Font, Length, Alignment};
use iced::widget::{column, container, text, row, vertical_space, horizontal_space};
use crate::theme;
use crate::Message;

pub fn view<'a>() -> Element<'a, Message> {
    let content = container(
        column![
            vertical_space(60),
            text("AI Is Breaking at Scale")
                .size(48)
                .font(Font::BOLD)
                .style(theme::text::heading),
            vertical_space(40),
            
            // Three main problems
            row![
                problem_card(
                    "Fragmentation kills velocity",
                    "Businesses juggle many applications & tools, re-enter data, and never achieve true sync."
                ),
                horizontal_space(20),
                problem_card(
                    "The last 10% explodes compute",
                    "Teams reach 85-90% automation, then costs and fragility spike without the right structures."
                ),
                horizontal_space(20),
                problem_card(
                    "Lock-in & opacity",
                    "Cloud-centric AI → runaway spend, brittle workflows, and audits no one can pass."
                ),
            ]
            .align_y(Alignment::Start),
            
            vertical_space(60),
            
            // Root Cause vs Symptom table
            text("Root Cause vs Symptom")
                .size(32)
                .font(Font::BOLD)
                .style(theme::text::heading),
            vertical_space(20),
            
            cause_symptom_row("Fragmented & proprietary stacks", "Siloed data → duplication waste and a hard throughput ceiling."),
            cause_symptom_row("Vendor lock-in & cloud-only ops", "Exploding costs at scale; switching costs paralyze change."),
            cause_symptom_row("Brittle translations/APIs", "Scale-load failures; throttling/egress fees bleed budgets."),
            cause_symptom_row("Imperfect-math / nondeterministic code paths", "Inconsistent outputs; expensive debugging caps reliability."),
            cause_symptom_row("Weak governance & no single source of truth", "Untraceable decisions; audit exposure inflates compliance spend."),
            cause_symptom_row("Scale threshold (\"last 10%\")", "Compute spikes and instability; ROI collapses at ~85-90% automation."),
            
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

fn problem_card<'a>(title: &str, description: &str) -> Element<'a, Message> {
    container(
        column![
            text(title)
                .size(20)
                .font(Font::BOLD)
                .style(theme::text::primary),
            vertical_space(15),
            text(description)
                .size(16)
                .style(theme::text::secondary),
        ]
        .align_x(Alignment::Center)
    )
    .padding(25)
    .width(Length::FillPortion(1))
    .height(Length::Fixed(200.0))
    .style(theme::container::glass_card)
    .into()
}

fn cause_symptom_row<'a>(cause: &str, symptom: &str) -> Element<'a, Message> {
    container(
        row![
            container(
                text(cause)
                    .size(16)
                    .style(theme::text::primary)
            )
            .width(Length::FillPortion(1))
            .padding(15)
            .style(theme::container::card),
            horizontal_space(10),
            container(
                text(symptom)
                    .size(16)
                    .style(theme::text::secondary)
            )
            .width(Length::FillPortion(1))
            .padding(15)
            .style(theme::container::card),
        ]
    )
    .padding([5, 0])
    .into()
}