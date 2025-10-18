use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, Performance};

#[wasm_bindgen]
pub async fn run_logo_animation() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    
    web_sys::console::log_1(&"Starting logo animation test...".into());
    
    // Get the canvas element
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let performance = window.performance().unwrap();
    
    // Clear any existing canvases
    if let Some(existing) = document.query_selector("canvas").unwrap() {
        existing.remove();
    }
    
    // Create canvas
    let canvas = document
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    
    canvas.set_id("logo_canvas");
    canvas.set_width(900);
    canvas.set_height(600);
    canvas.style().set_property("display", "block").unwrap();
    canvas.style().set_property("margin", "0 auto").unwrap();
    canvas.style().set_property("background", "linear-gradient(135deg, #667eea 0%, #764ba2 100%)").unwrap();
    
    body.append_child(&canvas).unwrap();
    
    web_sys::console::log_1(&format!("Canvas created: {}x{}", canvas.width(), canvas.height()).into());
    
    // Create WGPU instance with WebGL backend
    let backend = wgpu::Backends::GL;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: backend,
        dx12_shader_compiler: Default::default(),
        flags: wgpu::InstanceFlags::empty(),
        gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
    });
    
    // Create surface from canvas
    let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|e| JsValue::from_str(&format!("Failed to create surface: {}", e)))?;
    
    // Request adapter
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .ok_or_else(|| JsValue::from_str("Failed to find adapter"))?;
    
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
    
    web_sys::console::log_1(&"WGPU initialized, starting animation loop...".into());
    
    // Animation loop
    let start_time = performance.now();
    let mut frame_count = 0;
    
    // Run for a few seconds
    loop {
        let current_time = performance.now();
        let elapsed = (current_time - start_time) / 1000.0; // Convert to seconds
        
        // Animate color based on time
        let r = ((elapsed.sin() + 1.0) / 2.0) as f64;
        let g = ((elapsed * 1.5).cos() + 1.0) / 2.0;
        let b = ((elapsed * 2.0).sin() + 1.0) / 2.0;
        
        // Get current texture
        let output = match surface.get_current_texture() {
            Ok(texture) => texture,
            Err(e) => {
                web_sys::console::error_1(&format!("Failed to get texture: {:?}", e).into());
                surface.configure(&device, &config);
                continue;
            }
        };
        
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        // Create command encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Animation Encoder"),
        });
        
        // Clear with animated color
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Animation Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r, g, b, a: 1.0 }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }
        
        // Submit commands
        queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        frame_count += 1;
        
        // Log every 60 frames (approximately once per second at 60fps)
        if frame_count % 60 == 0 {
            web_sys::console::log_1(&format!("Animation running... {} frames, {:.2}s elapsed", frame_count, elapsed).into());
        }
        
        // Stop after 3 seconds
        if elapsed > 3.0 {
            break;
        }
        
        // Small delay to control frame rate (approximately 60fps)
        // In a real application, you'd use requestAnimationFrame
        gloo_timers::future::TimeoutFuture::new(16).await;
    }
    
    // Now add a logo in the center
    web_sys::console::log_1(&"Animation complete, adding logo...".into());
    
    // Create an image element for the logo
    let img = document.create_element("img").unwrap();
    img.set_attribute("src", "/assets/logo.svg").unwrap();
    img.set_attribute("style", 
        "position: absolute; \
         top: 50%; \
         left: 50%; \
         transform: translate(-50%, -50%); \
         width: 300px; \
         height: auto; \
         filter: drop-shadow(0 10px 20px rgba(0,0,0,0.3)); \
         animation: pulse 2s infinite;"
    ).unwrap();
    
    // Add CSS animation
    let style = document.create_element("style").unwrap();
    style.set_text_content(Some(
        "@keyframes pulse { \
            0% { transform: translate(-50%, -50%) scale(1); } \
            50% { transform: translate(-50%, -50%) scale(1.05); } \
            100% { transform: translate(-50%, -50%) scale(1); } \
        }"
    ));
    document.head().unwrap().append_child(&style).unwrap();
    
    body.append_child(&img).unwrap();
    
    web_sys::console::log_1(&"Logo animation complete!".into());
    
    Ok(())
}