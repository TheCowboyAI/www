use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
pub async fn run_minimal_wgpu() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    
    web_sys::console::log_1(&"Starting minimal WGPU test...".into());
    
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
            
            canvas.set_id("wgpu_canvas");
            canvas.set_width(800);
            canvas.set_height(600);
            canvas.style().set_property("display", "block").unwrap();
            canvas.style().set_property("border", "2px solid red").unwrap();
            
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
    
    // Draw a simple colored rectangle
    web_sys::console::log_1(&"Starting render loop...".into());
    
    // Simple render loop - just clear to a color
    let mut frame_count = 0;
    loop {
        frame_count += 1;
        
        // Get current texture
        let output = match surface.get_current_texture() {
            Ok(texture) => texture,
            Err(e) => {
                web_sys::console::error_1(&format!("Failed to get texture: {:?}", e).into());
                // Reconfigure surface and try again
                surface.configure(&device, &config);
                continue;
            }
        };
        
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        // Create command encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        // Clear to red color
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,  // Red
                            g: 0.0,
                            b: 0.0,
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
        
        // Submit commands
        queue.submit(std::iter::once(encoder.finish()));
        output.present();
        
        if frame_count == 1 {
            web_sys::console::log_1(&"First frame rendered!".into());
        }
        
        // Break after first frame for now - in real app would continue
        break;
    }
    
    web_sys::console::log_1(&"Minimal WGPU test complete - should show red canvas".into());
    
    Ok(())
}