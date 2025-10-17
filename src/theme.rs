use iced::{Color, Theme, Background};

#[derive(Debug, Clone, Copy)]
pub enum AppTheme {
    Dark,
}

impl Default for AppTheme {
    fn default() -> Self {
        AppTheme::Dark
    }
}

impl From<AppTheme> for Theme {
    fn from(_theme: AppTheme) -> Self {
        Theme::Dark
    }
}

pub mod colors {
    use iced::Color;
    
    pub const PRIMARY: Color = Color::from_rgb(0.31, 0.51, 0.91);
    pub const SECONDARY: Color = Color::from_rgb(0.18, 0.82, 0.35);
    pub const ACCENT: Color = Color::from_rgb(0.45, 0.78, 0.96);
    pub const BACKGROUND: Color = Color::from_rgb(0.05, 0.05, 0.08);
    pub const SURFACE: Color = Color::from_rgb(0.08, 0.08, 0.12);
    pub const TEXT_PRIMARY: Color = Color::from_rgb(0.95, 0.95, 0.98);
    pub const TEXT_SECONDARY: Color = Color::from_rgb(0.65, 0.65, 0.70);
    pub const ERROR: Color = Color::from_rgb(0.91, 0.31, 0.31);
    pub const WARNING: Color = Color::from_rgb(0.91, 0.71, 0.31);
}

pub mod container {
    use iced::widget::container;
    use super::colors;
    
    pub fn dark_background(theme: &iced::Theme) -> container::Style {
        container::Style {
            background: Some(Background::Color(colors::BACKGROUND)),
            text_color: Some(colors::TEXT_PRIMARY),
            ..container::Style::default()
        }
    }
    
    pub fn header(theme: &iced::Theme) -> container::Style {
        container::Style {
            background: Some(Background::Color(colors::SURFACE)),
            text_color: Some(colors::TEXT_PRIMARY),
            border: iced::Border {
                color: colors::PRIMARY.scale_alpha(0.2),
                width: 0.0,
                radius: 0.0.into(),
            },
            ..container::Style::default()
        }
    }
    
    pub fn card(theme: &iced::Theme) -> container::Style {
        container::Style {
            background: Some(Background::Color(colors::SURFACE)),
            text_color: Some(colors::TEXT_PRIMARY),
            border: iced::Border {
                color: colors::PRIMARY.scale_alpha(0.3),
                width: 1.0,
                radius: 12.0.into(),
            },
            ..container::Style::default()
        }
    }
    
    pub fn glass_card(theme: &iced::Theme) -> container::Style {
        container::Style {
            background: Some(Background::Color(colors::SURFACE.scale_alpha(0.8))),
            text_color: Some(colors::TEXT_PRIMARY),
            border: iced::Border {
                color: colors::ACCENT.scale_alpha(0.2),
                width: 1.0,
                radius: 16.0.into(),
            },
            ..container::Style::default()
        }
    }
}

pub mod button {
    use iced::{Color, Background};
    use iced::widget::button;
    use super::colors;
    
    pub fn primary(theme: &iced::Theme, status: button::Status) -> button::Style {
        match status {
            button::Status::Active => button::Style {
                background: Some(Background::Color(colors::PRIMARY)),
                text_color: Color::WHITE,
                border: iced::Border {
                    color: colors::PRIMARY,
                    width: 0.0,
                    radius: 8.0.into(),
                },
                ..button::Style::default()
            },
            button::Status::Hovered => button::Style {
                background: Some(Background::Color(colors::PRIMARY.scale_alpha(0.9))),
                text_color: Color::WHITE,
                border: iced::Border {
                    color: colors::PRIMARY,
                    width: 0.0,
                    radius: 8.0.into(),
                },
                ..button::Style::default()
            },
            _ => button::Style::default(),
        }
    }
    
    pub fn nav_button(theme: &iced::Theme, status: button::Status) -> button::Style {
        match status {
            button::Status::Active => button::Style {
                background: None,
                text_color: colors::TEXT_PRIMARY,
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 4.0.into(),
                },
                ..button::Style::default()
            },
            button::Status::Hovered => button::Style {
                background: Some(Background::Color(colors::PRIMARY.scale_alpha(0.1))),
                text_color: colors::ACCENT,
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 4.0.into(),
                },
                ..button::Style::default()
            },
            _ => button::Style::default(),
        }
    }
    
    pub fn secondary(theme: &iced::Theme, status: button::Status) -> button::Style {
        match status {
            button::Status::Active => button::Style {
                background: None,
                text_color: colors::PRIMARY,
                border: iced::Border {
                    color: colors::PRIMARY,
                    width: 2.0,
                    radius: 8.0.into(),
                },
                ..button::Style::default()
            },
            button::Status::Hovered => button::Style {
                background: Some(Background::Color(colors::PRIMARY.scale_alpha(0.1))),
                text_color: colors::PRIMARY,
                border: iced::Border {
                    color: colors::PRIMARY,
                    width: 2.0,
                    radius: 8.0.into(),
                },
                ..button::Style::default()
            },
            _ => button::Style::default(),
        }
    }
}

pub mod text {
    use iced::widget::text;
    use super::colors;
    
    pub fn primary(theme: &iced::Theme) -> text::Style {
        text::Style {
            color: Some(colors::TEXT_PRIMARY),
        }
    }
    
    pub fn secondary(theme: &iced::Theme) -> text::Style {
        text::Style {
            color: Some(colors::TEXT_SECONDARY),
        }
    }
    
    pub fn accent(theme: &iced::Theme) -> text::Style {
        text::Style {
            color: Some(colors::ACCENT),
        }
    }
    
    pub fn heading(theme: &iced::Theme) -> text::Style {
        text::Style {
            color: Some(colors::TEXT_PRIMARY),
        }
    }
}