use iced::{
    Background, Color,
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