# Cowboy AI WASM Presentation

A modern, interactive web presentation built with Rust, Iced, and WebAssembly, featuring an ethereal particle cloud effect and glass-morphism design.

## Features

- **CIMBOL Landing Page**: Beautiful login interface with glass-morphism design
- **Ethereal Particle Cloud**: 32 animated particles with repelling force interaction
- **Navy Gradient Background**: Rich gradient from navy blue to black
- **WebGL Rendering**: Hardware-accelerated graphics via WGPU
- **Responsive Design**: Adapts to different screen sizes

## Technology Stack

- **Rust** with Iced 0.13.1 framework
- **WebAssembly** (WASM) compilation
- **WebGL** rendering via WGPU
- **Trunk** for building and serving
- **NixOS** for deployment environment

## Prerequisites

- Rust toolchain (nightly recommended)
- Trunk (`cargo install trunk`)
- wasm32-unknown-unknown target (`rustup target add wasm32-unknown-unknown`)

## Development

### Enter development environment
```bash
nix develop
```

### Run development server
```bash
cd iced-wasm
trunk serve --port 8080
```

The application will be available at http://localhost:8080

### Build for production
```bash
trunk build --release
```

## Deployment

This project is deployed as a NixOS LXC container on Proxmox. See `CLAUDE.md` for detailed deployment instructions.

### Quick Deploy
```bash
./deploy-lxc-correct.sh
```

## Project Structure

```
.
├── iced-wasm/           # Main WASM application
│   ├── src/
│   │   ├── lib.rs       # Main application entry
│   │   ├── landing_page.rs  # CIMBOL landing page
│   │   ├── logo_animation.rs # Logo animation logic
│   │   └── presentation_theme.rs # Theme definitions
│   ├── Cargo.toml       # Rust dependencies
│   ├── Trunk.toml       # Trunk configuration
│   └── index.html       # HTML entry point
├── assets/              # Static assets
│   ├── logo.svg        # Cowboy AI logo
│   └── favicon.ico     # Site favicon
├── nixosModules/        # NixOS configuration
│   ├── nginx.nix       # Web server config
│   └── system.nix      # System configuration
└── flake.nix           # Nix flake configuration
```

## Known Issues

See `ICED_WEBGL_REQUIREMENTS.md` for detailed information about Iced 0.13 WebGL rendering issues and workarounds.

### Key Points
- Iced 0.13 standard API doesn't render in WASM with WebGL (issue #2519)
- Solution uses raw WGPU for rendering + DOM for UI
- wasm-bindgen must be pinned to version 0.2.104

## Interactive Features

### Particle Cloud
- **32 glowing white particles** create an ethereal cloud effect
- **Repelling force**: Particles move away from mouse cursor
- **Smooth animations**: Natural floating movement

### UI Elements
- Glass-morphism login card with backdrop blur
- Navy gradient background with enhanced depth
- Tight drop shadows for text clarity

## Architecture Notes

Due to Iced WebGL limitations, the application uses a hybrid approach:
1. **WGPU/WebGL** for background rendering
2. **Web-sys/DOM** for UI elements
3. **CSS** for styling and effects
4. **JavaScript** for particle interactions

This provides better performance and browser compatibility than pure canvas rendering.

## Browser Support

- ✅ Chrome/Edge (Full support)
- ✅ Firefox (Full support)
- ✅ Safari (Requires WebGL2/macOS 10.15+)

## License

Private project - All rights reserved

## Contact

For more information, visit [thecowboy.ai](https://www.thecowboy.ai)