# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

This is the Cowboy AI WASM presentation website, built with Iced.rs and deployed as a NixOS LXC container on Proxmox.

## Building the WASM Application with Iced 0.13

### Critical Requirements for Iced 0.13 WebGL in WASM

The Cowboy AI presentation uses **Iced.rs v0.13.1** compiled to WebAssembly with WebGL. Getting this to work requires specific configuration:

1. **Use Trunk for building** - NOT wasm-pack alone! Trunk properly handles WebGL context initialization that Iced requires.

2. **WebGL Rendering Issue with Standard API**:
   The standard `iced::application()` API has a known issue (#2519) where WebGL doesn't render in WASM. The canvas is created but stays blank.
   
   **Solution**: Use the "integration" pattern from Iced examples, which manually initializes the renderer. This approach is used successfully in projects like polyblade.
   
   Reference: 
   - Issue: https://github.com/iced-rs/iced/issues/2519
   - Integration example: https://github.com/iced-rs/iced/tree/0.13/examples/integration

3. **For the integration pattern, use these dependencies**:
   ```toml
   [dependencies]
   # Core iced crates for manual integration
   iced_wgpu = { version = "0.13.1", features = ["webgl"] }
   iced_winit = { version = "0.13.1" }
   iced_runtime = { version = "0.13.1" }
   iced_widget = { version = "0.13.1" }
   iced_core = { version = "0.13.1" }
   
   # WebGL/wgpu dependencies
   wgpu = { version = "0.20", features = ["webgl"] }
   winit = { version = "0.30", features = ["rwh_06"] }
   
   # WASM specific
   wasm-bindgen = "=0.2.104"  # Pin version to avoid mismatches
   wasm-bindgen-futures = "0.4"
   web-sys = { version = "0.3", features = ["console", "Document", "Window", "Element", "HtmlCanvasElement"] }
   ```

4. **Key Implementation Points**:
   - Manually create the wgpu instance with `Backends::GL` for WebGL
   - Initialize Engine and Renderer separately
   - Handle the render loop manually
   - Set proper canvas dimensions before initialization
   - Use `Trunk.toml` with `release = false` for development (release mode has issues with WebGL)

5. **Use `#[wasm_bindgen(start)]` attribute** - The main entry point must use the `start` attribute:
   ```rust
   #[wasm_bindgen(start)]
   pub fn main() {
       console_error_panic_hook::set_once();
       // Initialize using integration pattern
   }
   ```

### Building for Development

```bash
cd iced-wasm
nix develop -c trunk build
```

### Building for Production

```bash
cd iced-wasm
nix develop -c trunk build --release
```

### Serving Locally

```bash
cd iced-wasm
nix develop -c trunk serve
# Opens at http://localhost:8080
```

### Build Output

Trunk creates a `dist/` directory with:
- Optimized WASM binary
- Generated JavaScript bindings
- HTML with proper initialization
- All assets properly linked

This `dist/` directory is what gets deployed to the nginx server in the container.

## Building and Deploying NixOS LXC Containers

### Critical Requirements for NixOS on Proxmox LXC

When building NixOS containers for Proxmox LXC deployment, you MUST:

1. **Import the LXC module** in your NixOS configuration:
   ```nix
   imports = [
     (modulesPath + "/virtualisation/proxmox-lxc.nix")
   ];
   ```

2. **Set container-specific options**:
   ```nix
   boot.isContainer = true;
   
   # Suppress systemd units that don't work in LXC
   systemd.suppressedSystemUnits = [
     "dev-mqueue.mount"
     "sys-kernel-debug.mount"
     "sys-fs-fuse-connections.mount"
   ];
   ```

3. **Use `pct create` command** - NEVER use the Proxmox web UI for NixOS containers

4. **Required `pct create` parameters**:
   - `--ostype unmanaged` - CRITICAL: Prevents Proxmox from breaking NixOS
   - `--features nesting=1` - REQUIRED: NixOS needs this for cgroups/systemd
   - `--cmode console` - Fixes console access
   - `--unprivileged 1` - Better security (or 0 for privileged if needed)

### Building the Container

```bash
# Build the LXC container
nix build .#packages.x86_64-linux.wwwlxc
```

### Deploying to Proxmox

Use the deployment script:
```bash
./deploy-lxc-correct.sh
```

Or manually:
```bash
# Upload template
scp -P 9922 -i ~/.ssh/id_cim_thecowboyai \
  result/tarball/*.tar.xz \
  root@143.105.59.182:/var/lib/vz/template/cache/nixos-www.tar.xz

# Create container (via SSH, NOT web UI!)
ssh -p 9922 -i ~/.ssh/id_cim_thecowboyai root@143.105.59.182
pct create 134 \
  local:vztmpl/nixos-www.tar.xz \
  --arch amd64 \
  --ostype unmanaged \
  --hostname www \
  --net0 name=eth0,bridge=vmbr1,ip=10.0.64.134/24,gw=10.0.64.1 \
  --storage local-lvm \
  --memory 2048 \
  --rootfs local-lvm:8 \
  --unprivileged 1 \
  --features nesting=1 \
  --cmode console \
  --onboot 1 \
  --start 1
```

### Known Issues

1. **First `nixos-rebuild` may fail**: This is expected. Simply reboot the container after the first rebuild.

2. **Console appears blank**: Press Enter to see the login prompt.

3. **"sync_wait" errors**: Usually means the container is missing proper LXC configuration. Ensure you're importing the proxmox-lxc.nix module.

4. **Container won't start**: Check that:
   - You used `--ostype unmanaged`
   - You enabled `nesting=1`
   - The module includes `boot.isContainer = true`

### Common Tasks

#### Check container status
```bash
ssh -p 9922 -i ~/.ssh/id_cim_thecowboyai root@143.105.59.182 "pct status 134"
```

#### Access container console
```bash
ssh -p 9922 -i ~/.ssh/id_cim_thecowboyai root@143.105.59.182 "pct console 134"
```

#### Execute commands in container
```bash
ssh -p 9922 -i ~/.ssh/id_cim_thecowboyai root@143.105.59.182 \
  "pct exec 134 -- /run/current-system/sw/bin/systemctl status nginx"
```

Note: Always use `/run/current-system/sw/bin/` prefix for NixOS commands when using `pct exec`.

## Architecture

- **Container ID**: 134
- **IP Address**: 10.0.64.134
- **Gateway**: 10.0.64.1
- **Bridge**: vmbr1
- **Proxmox Host**: 143.105.59.182:9922

## SSL Certificates

The container uses Cloudflare Origin certificates stored in `/git/thecowboyai/www/certs/`:
- `thecowboy.ai.pem` - Certificate
- `thecowboy.ai.key` - Private key

These are included in the NixOS build and served by nginx.