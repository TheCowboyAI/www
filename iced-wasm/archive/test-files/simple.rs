use iced::{
    widget::{button, column, container, text},
    Alignment, Element, Length, Task,
};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> (Self, Task<Message>) {
        (Counter { value: 0 }, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        container(
            column![
                button("Increment").on_press(Message::IncrementPressed),
                text(self.value).size(50),
                button("Decrement").on_press(Message::DecrementPressed),
            ]
            .padding(20)
            .align_x(Alignment::Center)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }
}

#[wasm_bindgen]
pub async fn run_simple() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"Starting simple counter app...".into());
    
    let _ = iced::application("Simple Counter", Counter::update, Counter::view)
        .run_with(Counter::new);
    
    web_sys::console::log_1(&"Simple counter initialized".into());
}