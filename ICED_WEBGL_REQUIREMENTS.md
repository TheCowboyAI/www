# Iced WebGL Requirements and Known Issues

## Critical Issue: Iced 0.13 WebGL Rendering Bug

**Issue #2519**: Iced 0.13's standard application API does not render in WASM with WebGL backend.

### Problem Description
When using Iced 0.13.1 with WebGL feature in WASM:
- The canvas element is created successfully
- WebGL context initializes without errors
- No rendering appears on screen (blank canvas)
- No error messages in console

### Root Cause
The standard Iced application API (`iced::application`) has a rendering pipeline issue specific to WebGL in WASM environments. This is a known bug tracked as issue #2519 in the Iced repository.

## Working Solution: Raw WGPU with WebGL Backend

### Requirements
```toml
# Cargo.toml
[dependencies]
iced = { version = "0.13.1", default-features = false, features = ["webgl", "canvas", "image", "svg", "debug"] }
wgpu = { version = "0.20", features = ["webgl"] }
wasm-bindgen = "=0.2.104"  # Version must be pinned
web-sys = { version = "0.3", features = [
    "console", "Document", "Window", "Element",
    "HtmlCanvasElement", "HtmlInputElement", "HtmlButtonElement", "Event"
]}
```

### Key Configuration Points

1. **Use WGPU directly for rendering**
   - Create WGPU instance with `wgpu::Backends::GL`
   - Initialize surface from canvas element
   - Handle render loop manually

2. **Build with Trunk**
   - Use `trunk serve` for development (NOT wasm-pack)
   - Trunk handles WASM compilation and module loading correctly

3. **Pin wasm-bindgen version**
   - Must use exact version `=0.2.104`
   - Version mismatches cause cryptic runtime errors

## Implementation Pattern

### Working Approach
```rust
// Use raw WGPU for WebGL rendering
let backend = wgpu::Backends::GL;
let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
    backends: backend,
    dx12_shader_compiler: Default::default(),
    flags: wgpu::InstanceFlags::empty(),
    gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
});

// Create surface from canvas
let surface = instance.create_surface(
    wgpu::SurfaceTarget::Canvas(canvas.clone())
)?;

// Build UI with DOM manipulation
// Use web-sys for creating HTML elements
// Apply styles via CSS strings
```

### What Doesn't Work
- Standard Iced application pattern (`iced::application`)
- Iced's built-in view/update cycle in WASM
- WebGPU backend (adapter not found in browser)
- Using wasm-pack for building

## Architecture Decision

Due to these limitations, the application uses:
1. **Raw WGPU** for WebGL background rendering
2. **DOM manipulation** via web-sys for UI elements
3. **CSS styling** for glass-morphism and effects
4. **JavaScript** for interactive features (particle effects)

This hybrid approach provides:
- Reliable WebGL rendering
- Rich UI with proper browser integration
- Better performance than pure canvas rendering
- Full CSS animation and effect support

## Testing Requirements

### Browser Compatibility
- Chrome/Edge: Full support
- Firefox: Full support
- Safari: WebGL2 required (macOS 10.15+)

### Development Server
Always use Trunk for serving:
```bash
trunk serve --port 8080
```

### Build for Production
```bash
trunk build --release
```

## Known Workarounds

1. **Canvas sizing**: Set width/height explicitly on canvas element
2. **Module loading**: Use Trunk's automatic WASM loading, not manual
3. **Performance**: Use `AutoVsync` present mode for smooth animations
4. **Memory**: Configure proper limits with `downlevel_webgl2_defaults()`

## Future Considerations

When Iced fixes issue #2519:
- Migration path: Move from DOM to pure Iced widgets
- Keep WGPU rendering layer for performance
- Gradually adopt Iced's view/update pattern

## References

- [Iced Issue #2519](https://github.com/iced-rs/iced/issues/2519)
- [WGPU WebGL Backend Documentation](https://docs.rs/wgpu/latest/wgpu/struct.Backends.html)
- [Trunk Documentation](https://trunkrs.dev/)
- [web-sys Documentation](https://rustwasm.github.io/wasm-bindgen/api/web_sys/)