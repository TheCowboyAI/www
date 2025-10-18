use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Set panic hook
    console_error_panic_hook::set_once();
    
    // Simple console log  
    web_sys::console::log_1(&"ULTRA MINIMAL: Starting...".into());
    
    // Just try to create a simple colored rectangle using iced
    iced::run::<(), (), iced::Theme, iced::Renderer>("Test", |_state: &mut (), _message: ()| {
        web_sys::console::log_1(&"ULTRA MINIMAL: Update called".into());
        ()
    }, |_state: &()| {
        use iced::{widget::{container, text}, Color, Background, Length, Theme, Element};
        
        web_sys::console::log_1(&"ULTRA MINIMAL: View called".into());
        
        // Need content in the container
        let content: Element<(), Theme, iced::Renderer> = container(text("RED SCREEN TEST").size(100).color(Color::WHITE))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_theme: &Theme| container::Style {
                background: Some(Background::Color(Color::from_rgb(1.0, 0.0, 0.0))),
                text_color: Some(Color::WHITE),
                ..Default::default()
            })
            .into();
        content
    });
    
    Ok(())
}