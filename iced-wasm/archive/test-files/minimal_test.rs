use iced::{
    widget::{container, text},
    Element, Length, Task, Color, Background,
};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
pub enum Message {}

pub struct MinimalApp;

impl MinimalApp {
    pub fn new() -> (Self, Task<Message>) {
        web_sys::console::log_1(&"MinimalApp::new called".into());
        (MinimalApp, Task::none())
    }

    pub fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    pub fn view(&self) -> Element<Message> {
        web_sys::console::log_1(&"MinimalApp::view called".into());
        
        container(
            text("Hello from Iced WASM!")
                .size(50)
                .color(Color::WHITE)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(Background::Color(Color::from_rgb(0.2, 0.3, 0.8))),
            text_color: Some(Color::WHITE),
            ..Default::default()
        })
        .into()
    }
}

#[wasm_bindgen]
pub fn run_minimal() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"run_minimal called".into());
    
    let result = iced::application("Minimal Test", MinimalApp::update, MinimalApp::view)
        .run_with(MinimalApp::new);
        
    web_sys::console::log_1(&format!("Minimal app result: {:?}", result).into());
}