use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, HtmlElement, HtmlInputElement, HtmlButtonElement};

#[wasm_bindgen]
pub async fn run_landing_page() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    
    web_sys::console::log_1(&"Starting CIMBOL Landing Page...".into());
    
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let performance = window.performance().unwrap();
    
    // Clear any existing content
    body.set_inner_html("");
    
    // Add custom styles
    let style = document.create_element("style").unwrap();
    style.set_text_content(Some(r#"
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@300;400;600;700&display=swap');
        
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: 'Inter', sans-serif;
            overflow: hidden;
            position: relative;
        }
        
        #webgl-background {
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            z-index: 0;
        }
        
        .overlay {
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background: radial-gradient(ellipse at center top, rgba(20, 40, 80, 0.3) 0%, rgba(0, 0, 0, 0.7) 100%);
            z-index: 1;
        }
        
        .container {
            position: relative;
            z-index: 2;
            display: flex;
            flex-direction: column;
            min-height: 100vh;
            padding: 2rem;
        }
        
        .header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 4rem;
        }
        
        .logo-container {
            display: flex;
            align-items: center;
            gap: 1rem;
            position: relative;
        }
        
        .particles-container {
            position: absolute;
            top: 50%;
            left: 30px;
            transform: translate(-50%, -50%);
            width: 150px;
            height: 150px;
            pointer-events: auto;
            z-index: 1;
        }
        
        .particle {
            position: absolute;
            width: 6px;
            height: 6px;
            background: radial-gradient(circle, rgba(255, 255, 255, 1) 0%, rgba(255, 255, 255, 0.8) 30%, rgba(255, 255, 255, 0.4) 60%, transparent 100%);
            border-radius: 50%;
            filter: blur(0.5px);
            animation: float 8s infinite ease-in-out;
            box-shadow: 0 0 15px rgba(255, 255, 255, 0.9), 
                        0 0 30px rgba(255, 255, 255, 0.7),
                        0 0 45px rgba(255, 255, 255, 0.5),
                        0 0 60px rgba(255, 255, 255, 0.3);
            transition: transform 0.2s ease-out;
            pointer-events: none;
        }
        
        .glow-orb {
            position: absolute;
            background: radial-gradient(circle, rgba(255, 255, 255, 0.7) 0%, rgba(255, 255, 255, 0.4) 30%, rgba(255, 255, 255, 0.2) 60%, transparent 100%);
            border-radius: 50%;
            filter: blur(5px);
            animation: pulse 4s infinite ease-in-out;
            box-shadow: 0 0 40px rgba(255, 255, 255, 0.8),
                        0 0 80px rgba(255, 255, 255, 0.6);
        }
        
        @keyframes pulse {
            0%, 100% {
                transform: scale(1);
                opacity: 0.6;
            }
            50% {
                transform: scale(1.3);
                opacity: 0.9;
            }
        }
        
        
        @keyframes float {
            0%, 100% {
                transform: translateY(0px) translateX(0px);
                opacity: 0.7;
            }
            25% {
                transform: translateY(-10px) translateX(5px);
                opacity: 1;
            }
            50% {
                transform: translateY(5px) translateX(-3px);
                opacity: 0.9;
            }
            75% {
                transform: translateY(-5px) translateX(-5px);
                opacity: 1;
            }
        }
        
        .logo {
            width: 60px;
            height: 60px;
            filter: drop-shadow(0 4px 6px rgba(0, 0, 0, 0.3));
            position: relative;
            z-index: 2;
        }
        
        .logo-text {
            color: white;
            font-size: 1.5rem;
            font-weight: 700;
            text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.9),
                        2px 2px 4px rgba(0, 0, 0, 0.8),
                        3px 3px 6px rgba(0, 0, 0, 0.6);
            position: relative;
            z-index: 3;
        }
        
        .nav-links {
            display: flex;
            gap: 2rem;
        }
        
        .nav-link {
            color: white;
            text-decoration: none;
            font-weight: 500;
            font-size: 1.1rem;
            transition: all 0.3s ease;
            padding: 0.5rem 1rem;
            border-radius: 8px;
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(10px);
        }
        
        .nav-link:hover {
            background: rgba(255, 255, 255, 0.2);
            transform: translateY(-2px);
        }
        
        .main-content {
            flex: 1;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
        }
        
        .hero-section {
            text-align: center;
            margin-bottom: 3rem;
        }
        
        .hero-title {
            color: white;
            font-size: 3.5rem;
            font-weight: 700;
            margin-bottom: 1rem;
            text-shadow: 3px 3px 6px rgba(0, 0, 0, 0.5);
            animation: fadeInUp 1s ease;
        }
        
        .hero-subtitle {
            color: rgba(255, 255, 255, 0.9);
            font-size: 1.3rem;
            font-weight: 300;
            text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.5);
            animation: fadeInUp 1s ease 0.2s both;
        }
        
        .login-card {
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(20px);
            border-radius: 20px;
            padding: 2.5rem;
            width: 100%;
            max-width: 400px;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3), 0 0 100px rgba(102, 126, 234, 0.1);
            border: 1px solid rgba(255, 255, 255, 0.2);
            animation: fadeInUp 1s ease 0.4s both, glow 4s ease-in-out infinite;
        }
        
        @keyframes glow {
            0%, 100% {
                box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3), 0 0 80px rgba(30, 60, 114, 0.2);
            }
            50% {
                box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3), 0 0 120px rgba(42, 82, 152, 0.4);
            }
        }
        
        .login-title {
            color: white;
            font-size: 1.8rem;
            font-weight: 600;
            margin-bottom: 2rem;
            text-align: center;
        }
        
        .form-group {
            margin-bottom: 1.5rem;
        }
        
        .form-label {
            color: rgba(255, 255, 255, 0.8);
            font-size: 0.9rem;
            font-weight: 500;
            margin-bottom: 0.5rem;
            display: block;
        }
        
        .form-input {
            width: 100%;
            padding: 0.75rem 1rem;
            background: rgba(255, 255, 255, 0.1);
            border: 1px solid rgba(255, 255, 255, 0.3);
            border-radius: 10px;
            color: white;
            font-size: 1rem;
            transition: all 0.3s ease;
        }
        
        .form-input:focus {
            outline: none;
            background: rgba(255, 255, 255, 0.15);
            border-color: rgba(255, 255, 255, 0.5);
        }
        
        .form-input::placeholder {
            color: rgba(255, 255, 255, 0.5);
        }
        
        .login-button {
            width: 100%;
            padding: 1rem;
            background: linear-gradient(135deg, #1e3c72 0%, #2a5298 100%);
            border: none;
            border-radius: 10px;
            color: white;
            font-size: 1.1rem;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.3s ease;
            margin-bottom: 1rem;
        }
        
        .login-button:hover {
            transform: translateY(-2px);
            box-shadow: 0 10px 30px rgba(102, 126, 234, 0.4);
        }
        
        .login-button:active {
            transform: translateY(0);
        }
        
        .divider {
            color: rgba(255, 255, 255, 0.5);
            text-align: center;
            margin: 1.5rem 0;
            font-size: 0.9rem;
        }
        
        .sso-button {
            width: 100%;
            padding: 0.75rem;
            background: rgba(255, 255, 255, 0.1);
            border: 1px solid rgba(255, 255, 255, 0.3);
            border-radius: 10px;
            color: white;
            font-size: 1rem;
            font-weight: 500;
            cursor: pointer;
            transition: all 0.3s ease;
            margin-bottom: 0.75rem;
        }
        
        .sso-button:hover {
            background: rgba(255, 255, 255, 0.15);
            transform: translateY(-2px);
        }
        
        .footer-text {
            color: rgba(255, 255, 255, 0.7);
            text-align: center;
            font-size: 0.9rem;
            margin-top: 2rem;
        }
        
        .footer-link {
            color: rgba(255, 255, 255, 0.9);
            text-decoration: none;
            font-weight: 500;
        }
        
        .footer-link:hover {
            text-decoration: underline;
        }
        
        @keyframes fadeInUp {
            from {
                opacity: 0;
                transform: translateY(30px);
            }
            to {
                opacity: 1;
                transform: translateY(0);
            }
        }
    "#));
    
    document.head().unwrap().append_child(&style).unwrap();
    
    // Create WebGL background canvas
    let canvas = document
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    canvas.set_id("webgl-background");
    body.append_child(&canvas).unwrap();
    
    // Create overlay
    let overlay = document.create_element("div").unwrap();
    overlay.set_class_name("overlay");
    body.append_child(&overlay).unwrap();
    
    // Create main container
    let container = document.create_element("div").unwrap();
    container.set_class_name("container");
    
    // Create header
    let header = document.create_element("div").unwrap();
    header.set_class_name("header");
    
    // Logo container
    let logo_container = document.create_element("div").unwrap();
    logo_container.set_class_name("logo-container");
    
    // Create particles container
    let particles_container = document.create_element("div").unwrap();
    particles_container.set_class_name("particles-container");
    
    // Add 5 larger glow orbs for more ethereal white background
    for i in 0..5 {
        let glow_orb = document.create_element("div").unwrap();
        glow_orb.set_class_name("glow-orb");
        // Set different sizes and positions for each orb - bigger and whiter
        let style = match i {
            0 => "width: 70px; height: 70px; top: 10%; left: 10%; animation-delay: 0s;",
            1 => "width: 60px; height: 60px; top: 60%; left: 60%; animation-delay: 1.5s;",
            2 => "width: 65px; height: 65px; top: 30%; left: 70%; animation-delay: 3s;",
            3 => "width: 55px; height: 55px; top: 75%; left: 20%; animation-delay: 2s;",
            _ => "width: 50px; height: 50px; top: 15%; left: 50%; animation-delay: 4s;",
        };
        glow_orb.set_attribute("style", style).unwrap();
        particles_container.append_child(&glow_orb).unwrap();
    }
    
    // Add 32 smaller particles for a dense white cloud
    for i in 0..32 {
        let particle = document.create_element("div").unwrap();
        particle.set_class_name("particle");
        
        // Distribute particles randomly around the logo
        let size = 4 + (i % 6) * 2; // Vary sizes from 4px to 14px
        let top = 5 + (i * 13) % 80; // Distribute vertically
        let left = 5 + (i * 17) % 80; // Distribute horizontally  
        let delay = (i as f32 * 0.3) % 5.0; // Stagger animations
        
        let style = format!(
            "width: {}px; height: {}px; top: {}%; left: {}%; animation-delay: {}s;",
            size, size, top, left, delay
        );
        particle.set_attribute("style", &style).unwrap();
        particles_container.append_child(&particle).unwrap();
    }
    
    let logo_img = document.create_element("img").unwrap();
    logo_img.set_attribute("src", "/assets/logo.svg").unwrap();
    logo_img.set_class_name("logo");
    
    let logo_text = document.create_element("div").unwrap();
    logo_text.set_class_name("logo-text");
    logo_text.set_text_content(Some("Cowboy AI"));
    
    logo_container.append_child(&particles_container).unwrap();
    logo_container.append_child(&logo_img).unwrap();
    logo_container.append_child(&logo_text).unwrap();
    
    // Navigation links
    let nav_links = document.create_element("div").unwrap();
    nav_links.set_class_name("nav-links");
    
    let about_link = document.create_element("a").unwrap();
    about_link.set_class_name("nav-link");
    about_link.set_attribute("href", "https://www.thecowboy.ai").unwrap();
    about_link.set_attribute("target", "_blank").unwrap();
    about_link.set_text_content(Some("About"));
    
    let docs_link = document.create_element("a").unwrap();
    docs_link.set_class_name("nav-link");
    docs_link.set_attribute("href", "#").unwrap();
    docs_link.set_text_content(Some("Documentation"));
    
    nav_links.append_child(&about_link).unwrap();
    nav_links.append_child(&docs_link).unwrap();
    
    header.append_child(&logo_container).unwrap();
    header.append_child(&nav_links).unwrap();
    
    // Main content
    let main_content = document.create_element("div").unwrap();
    main_content.set_class_name("main-content");
    
    // Hero section
    let hero_section = document.create_element("div").unwrap();
    hero_section.set_class_name("hero-section");
    
    let hero_title = document.create_element("h1").unwrap();
    hero_title.set_class_name("hero-title");
    hero_title.set_text_content(Some("Welcome to CIMBOL"));
    
    let hero_subtitle = document.create_element("p").unwrap();
    hero_subtitle.set_class_name("hero-subtitle");
    hero_subtitle.set_text_content(Some("Your Composable Information Machine Awaits"));
    
    hero_section.append_child(&hero_title).unwrap();
    hero_section.append_child(&hero_subtitle).unwrap();
    
    // Login card
    let login_card = document.create_element("div").unwrap();
    login_card.set_class_name("login-card");
    
    let login_title = document.create_element("h2").unwrap();
    login_title.set_class_name("login-title");
    login_title.set_text_content(Some("Sign In"));
    
    // Username field
    let username_group = document.create_element("div").unwrap();
    username_group.set_class_name("form-group");
    
    let username_label = document.create_element("label").unwrap();
    username_label.set_class_name("form-label");
    username_label.set_text_content(Some("Username or Email"));
    
    let username_input = document.create_element("input").unwrap()
        .dyn_into::<HtmlInputElement>().unwrap();
    username_input.set_class_name("form-input");
    username_input.set_type("text");
    username_input.set_placeholder("Enter your username");
    username_input.set_id("username");
    
    username_group.append_child(&username_label).unwrap();
    username_group.append_child(&username_input).unwrap();
    
    // Password field
    let password_group = document.create_element("div").unwrap();
    password_group.set_class_name("form-group");
    
    let password_label = document.create_element("label").unwrap();
    password_label.set_class_name("form-label");
    password_label.set_text_content(Some("Password"));
    
    let password_input = document.create_element("input").unwrap()
        .dyn_into::<HtmlInputElement>().unwrap();
    password_input.set_class_name("form-input");
    password_input.set_type("password");
    password_input.set_placeholder("Enter your password");
    password_input.set_id("password");
    
    password_group.append_child(&password_label).unwrap();
    password_group.append_child(&password_input).unwrap();
    
    // Login button
    let login_button = document.create_element("button").unwrap()
        .dyn_into::<HtmlButtonElement>().unwrap();
    login_button.set_class_name("login-button");
    login_button.set_text_content(Some("Enter CIMBOL"));
    
    // Add click handler
    let window_clone = window.clone();
    let login_closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
        web_sys::console::log_1(&"Login clicked!".into());
        window_clone.alert_with_message("CIMBOL authentication coming soon!").unwrap();
    }) as Box<dyn FnMut(_)>);
    
    login_button.add_event_listener_with_callback("click", login_closure.as_ref().unchecked_ref()).unwrap();
    login_closure.forget();
    
    
    // Footer text
    let footer_text = document.create_element("p").unwrap();
    footer_text.set_class_name("footer-text");
    footer_text.set_inner_html("Don't have an account? <a href='#' class='footer-link'>Request Access</a>");
    
    // Assemble login card
    login_card.append_child(&login_title).unwrap();
    login_card.append_child(&username_group).unwrap();
    login_card.append_child(&password_group).unwrap();
    login_card.append_child(&login_button).unwrap();
    login_card.append_child(&footer_text).unwrap();
    
    main_content.append_child(&hero_section).unwrap();
    main_content.append_child(&login_card).unwrap();
    
    container.append_child(&header).unwrap();
    container.append_child(&main_content).unwrap();
    
    body.append_child(&container).unwrap();
    
    // Add repelling force effect on mouse hover
    let script = document.create_element("script").unwrap();
    script.set_text_content(Some(r#"
        const particlesContainer = document.querySelector('.particles-container');
        const particles = document.querySelectorAll('.particle');
        const particlePositions = [];
        
        // Store original positions
        particles.forEach((particle) => {
            const style = particle.getAttribute('style');
            const topMatch = style.match(/top:\s*([0-9.]+)%/);
            const leftMatch = style.match(/left:\s*([0-9.]+)%/);
            particlePositions.push({
                top: topMatch ? parseFloat(topMatch[1]) : 50,
                left: leftMatch ? parseFloat(leftMatch[1]) : 50
            });
        });
        
        if (particlesContainer && particles.length > 0) {
            particlesContainer.addEventListener('mousemove', (e) => {
                const rect = particlesContainer.getBoundingClientRect();
                const mouseX = ((e.clientX - rect.left) / rect.width) * 100;
                const mouseY = ((e.clientY - rect.top) / rect.height) * 100;
                
                particles.forEach((particle, index) => {
                    const originalPos = particlePositions[index];
                    const dx = originalPos.left - mouseX;
                    const dy = originalPos.top - mouseY;
                    const distance = Math.sqrt(dx * dx + dy * dy);
                    
                    if (distance < 30) {
                        // Calculate repulsion force
                        const force = Math.max(0, (30 - distance) / 30);
                        const repelX = (dx / distance) * force * 20;
                        const repelY = (dy / distance) * force * 20;
                        
                        particle.style.transform = `translate(${repelX}px, ${repelY}px)`;
                    } else {
                        particle.style.transform = 'translate(0, 0)';
                    }
                });
            });
            
            particlesContainer.addEventListener('mouseleave', () => {
                particles.forEach(particle => {
                    particle.style.transform = 'translate(0, 0)';
                });
            });
        }
    "#));
    body.append_child(&script).unwrap();
    
    // Initialize WebGL background animation
    initialize_webgl_background(canvas, performance).await?;
    
    web_sys::console::log_1(&"CIMBOL Landing Page initialized!".into());
    
    Ok(())
}

async fn initialize_webgl_background(
    canvas: HtmlCanvasElement,
    _performance: web_sys::Performance,
) -> Result<(), JsValue> {
    // Set canvas size
    let window = web_sys::window().unwrap();
    let width = window.inner_width()?.as_f64().unwrap() as u32;
    let height = window.inner_height()?.as_f64().unwrap() as u32;
    canvas.set_width(width);
    canvas.set_height(height);
    
    // Create WGPU instance
    let backend = wgpu::Backends::GL;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: backend,
        dx12_shader_compiler: Default::default(),
        flags: wgpu::InstanceFlags::empty(),
        gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
    });
    
    let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|e| JsValue::from_str(&format!("Failed to create surface: {}", e)))?;
    
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .ok_or_else(|| JsValue::from_str("Failed to find adapter"))?;
    
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
    
    let capabilities = surface.get_capabilities(&adapter);
    let format = capabilities.formats[0];
    
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width,
        height,
        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode: wgpu::CompositeAlphaMode::Opaque,
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    
    surface.configure(&device, &config);
    
    // Animation loop with static gradient
    wasm_bindgen_futures::spawn_local(async move {
        loop {
            // Navy blue to black gradient colors
            let r1 = 0.05;  // Slight red for richer navy
            let g1 = 0.1;   // Touch of green for depth
            let b1 = 0.35;  // Strong blue for navy gradient
            
            let output = match surface.get_current_texture() {
                Ok(texture) => texture,
                Err(_) => {
                    gloo_timers::future::TimeoutFuture::new(16).await;
                    continue;
                }
            };
            
            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
            
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Background Encoder"),
            });
            
            {
                let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Background Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: r1,
                                g: g1,
                                b: b1,
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
            
            queue.submit(std::iter::once(encoder.finish()));
            output.present();
            
            gloo_timers::future::TimeoutFuture::new(16).await;
        }
    });
    
    Ok(())
}