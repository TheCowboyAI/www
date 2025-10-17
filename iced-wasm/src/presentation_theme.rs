use iced::{
    Background, Border, Color, Theme,
    widget::{container, button, text},
};

pub struct PresentationTheme;

impl PresentationTheme {
    pub fn hero_gradient() -> Background {
        Background::Gradient(iced::gradient::Gradient::Linear(
            iced::gradient::Linear::new(45.0)
                .add_stop(0.0, Color::from_rgb(0.4, 0.49, 0.92))
                .add_stop(1.0, Color::from_rgb(0.45, 0.29, 0.64))
        ))
    }

    pub fn cognition_gradient() -> Background {
        Background::Gradient(iced::gradient::Gradient::Linear(
            iced::gradient::Linear::new(135.0)
                .add_stop(0.0, Color::from_rgb(0.4, 0.49, 0.92))
                .add_stop(1.0, Color::from_rgb(0.45, 0.29, 0.64))
        ))
    }

    pub fn problem_gradient() -> Background {
        Background::Gradient(iced::gradient::Gradient::Linear(
            iced::gradient::Linear::new(135.0)
                .add_stop(0.0, Color::from_rgb(0.94, 0.58, 0.98))
                .add_stop(1.0, Color::from_rgb(0.96, 0.34, 0.42))
        ))
    }

    pub fn platform_gradient() -> Background {
        Background::Gradient(iced::gradient::Gradient::Linear(
            iced::gradient::Linear::new(135.0)
                .add_stop(0.0, Color::from_rgb(0.31, 0.68, 0.99))
                .add_stop(1.0, Color::from_rgb(0.0, 0.95, 0.99))
        ))
    }

    pub fn breakthrough_gradient() -> Background {
        Background::Gradient(iced::gradient::Gradient::Linear(
            iced::gradient::Linear::new(135.0)
                .add_stop(0.0, Color::from_rgb(0.98, 0.44, 0.60))
                .add_stop(1.0, Color::from_rgb(0.99, 0.88, 0.25))
        ))
    }

    pub fn trust_gradient() -> Background {
        Background::Gradient(iced::gradient::Gradient::Linear(
            iced::gradient::Linear::new(135.0)
                .add_stop(0.0, Color::from_rgb(0.19, 0.81, 0.82))
                .add_stop(1.0, Color::from_rgb(0.2, 0.03, 0.4))
        ))
    }

    pub fn sage_gradient() -> Background {
        Background::Gradient(iced::gradient::Gradient::Linear(
            iced::gradient::Linear::new(135.0)
                .add_stop(0.0, Color::from_rgb(0.66, 0.93, 0.92))
                .add_stop(1.0, Color::from_rgb(0.99, 0.84, 0.89))
        ))
    }
}

pub fn card_style(theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(Color::WHITE)),
        border: Border {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
            width: 1.0,
            radius: 16.0.into(),
        },
        text_color: Some(Color::BLACK),
        ..Default::default()
    }
}

pub fn hero_card_style(theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.95))),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 24.0.into(),
        },
        text_color: Some(Color::BLACK),
        ..Default::default()
    }
}

pub fn glass_card_style(theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.1))),
        border: Border {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.2),
            width: 1.0,
            radius: 16.0.into(),
        },
        text_color: Some(Color::WHITE),
        ..Default::default()
    }
}

pub fn primary_button_style(theme: &Theme) -> button::Appearance {
    button::Appearance {
        background: Some(Background::Color(Color::from_rgb(0.23, 0.51, 0.96))),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 8.0.into(),
        },
        text_color: Color::WHITE,
        ..Default::default()
    }
}

pub fn nav_button_style(theme: &Theme) -> button::Appearance {
    button::Appearance {
        background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.9))),
        border: Border {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
            width: 1.0,
            radius: 20.0.into(),
        },
        text_color: Color::from_rgb(0.2, 0.2, 0.2),
        ..Default::default()
    }
}

pub fn slide_indicator_style(active: bool) -> impl Fn(&Theme) -> button::Appearance {
    move |theme: &Theme| {
        if active {
            button::Appearance {
                background: Some(Background::Color(Color::from_rgb(0.23, 0.51, 0.96))),
                border: Border {
                    color: Color::WHITE,
                    width: 2.0,
                    radius: 12.0.into(),
                },
                text_color: Color::WHITE,
                ..Default::default()
            }
        } else {
            button::Appearance {
                background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.5))),
                border: Border {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                    width: 1.0,
                    radius: 12.0.into(),
                },
                text_color: Color::from_rgb(0.4, 0.4, 0.4),
                ..Default::default()
            }
        }
    }
}