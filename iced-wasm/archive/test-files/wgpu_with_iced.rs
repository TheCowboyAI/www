use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use iced_wgpu::{Engine, Renderer, graphics::Viewport};
use iced_core::{Font, Pixels, Size, Color, Background};
use iced_widget::{container, text};
use iced_core::Element;

#[wasm_bindgen]
pub async fn run_wgpu_with_iced() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    
    web_sys::console::log_1(&"Starting WGPU with Iced renderer test...".into());
    
    // Get the canvas element
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    
    // Find or create canvas
    let canvas = document
        .query_selector("canvas")
        .unwrap()
        .and_then(|el| el.dyn_into::<HtmlCanvasElement>().ok())
        .unwrap_or_else(|| {
            let canvas = document
                .create_element("canvas")
                .unwrap()
                .dyn_into::<HtmlCanvasElement>()
                .unwrap();
            
            canvas.set_id("iced_canvas");
            canvas.set_width(800);
            canvas.set_height(600);
            canvas.style().set_property("display", "block").unwrap();
            canvas.style().set_property("border", "2px solid blue").unwrap();
            
            body.append_child(&canvas).unwrap();
            canvas
        });
    
    web_sys::console::log_1(&format!("Canvas size: {}x{}", canvas.width(), canvas.height()).into());
    
    // Create WGPU instance with WebGL backend
    let backend = wgpu::Backends::GL;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: backend,
        dx12_shader_compiler: Default::default(),
        flags: wgpu::InstanceFlags::empty(),
        gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
    });
    
    web_sys::console::log_1(&"WGPU instance created with GL backend".into());
    
    // Create surface from canvas
    let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|e| JsValue::from_str(&format!("Failed to create surface: {}", e)))?;
    
    web_sys::console::log_1(&"Surface created from canvas".into());
    
    // Request adapter
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .ok_or_else(|| JsValue::from_str("Failed to find adapter"))?;
    
    web_sys::console::log_1(&format!("Adapter: {:?}", adapter.get_info()).into());
    
    // Request device and queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .map_err(|e| JsValue::from_str(&format!("Failed to create device: {}", e)))?;
    
    web_sys::console::log_1(&"Device and queue created".into());
    
    // Configure surface
    let size = (canvas.width(), canvas.height());
    let capabilities = surface.get_capabilities(&adapter);
    let format = capabilities
        .formats
        .iter()
        .find(|f| f.is_srgb())
        .copied()
        .unwrap_or(capabilities.formats[0]);
    
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.0,
        height: size.1,
        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode: wgpu::CompositeAlphaMode::Opaque,
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    
    surface.configure(&device, &config);
    
    web_sys::console::log_1(&format!("Surface configured: {}x{}, format: {:?}", size.0, size.1, format).into());
    
    // Now initialize Iced's renderer
    web_sys::console::log_1(&"Initializing Iced renderer...".into());
    
    let viewport = Viewport::with_physical_size(
        Size::new(size.0, size.1),
        1.0, // scale factor
    );
    
    let engine = Engine::new(&adapter, &device, &queue, format, None);
    let mut renderer = Renderer::new(&device, &engine, Font::default(), Pixels::from(16));
    
    web_sys::console::log_1(&"Iced engine and renderer initialized".into());
    
    // Create a simple UI element
    let ui: Element<(), iced_core::Theme, iced_wgpu::Renderer> = container(
        text("Hello from Iced + WGPU!")
            .size(50)
            .color(Color::WHITE)
    )
    .width(iced_core::Length::Fill)
    .height(iced_core::Length::Fill)
    .center_x(iced_core::Length::Fill)
    .center_y(iced_core::Length::Fill)
    .style(|_theme: &iced_core::Theme| iced_widget::container::Style {
        background: Some(Background::Color(Color::from_rgb(0.0, 0.0, 1.0))), // Blue background
        text_color: Some(Color::WHITE),
        ..Default::default()
    })
    .into();
    
    web_sys::console::log_1(&"UI element created".into());
    
    // Render one frame
    web_sys::console::log_1(&"Starting render...".into());
    
    // Get current texture
    let output = surface.get_current_texture()
        .map_err(|e| JsValue::from_str(&format!("Failed to get texture: {:?}", e)))?;
    
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    
    // Create command encoder
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Iced Render Encoder"),
    });
    
    // Clear to blue
    {
        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Clear Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 1.0,  // Blue
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
    }
    
    // Try to render with Iced
    web_sys::console::log_1(&"Calling Iced renderer.present()...".into());
    
    use iced_runtime::Debug;
    let debug = Debug::new();
    
    renderer.present(
        &engine,
        &device,
        &queue,
        &mut encoder,
        None, // No clear color since we already cleared
        format,
        &view,
        &viewport,
        &debug.overlay(),
    );
    
    // Submit commands
    queue.submit(std::iter::once(encoder.finish()));
    output.present();
    
    web_sys::console::log_1(&"Frame rendered with Iced!".into());
    
    Ok(())
}