# Cowboy AI Website Deployment Documentation

## Project Overview
This project contains the Cowboy AI cognitive orchestration platform presentation website, originally built as an Iced desktop application and needs to be converted to a web application for deployment.

## Current Infrastructure

### Server Details
- **Host**: thecowboy.ai - 143.105.59.182
- **Proxmox Host**: pve1
- **SSH Port**: 22 (Proxmox host)
- **SSH Key**: `~/.ssh/id_cim_thecowboyai`
- **Container ID**: 134
- **Container Hostname**: www
- **Container IP**: 10.0.64.134/19
- **Container Gateway**: 10.0.64.1

### Container Configuration (LXC 134)
```
arch: amd64
cores: 4
memory: 8192 MB
hostname: www
network: eth0 on vmbr0 (tag 64)
storage: 64GB on local-lvm
OS: unmanaged (NixOS)
```

## Content Extracted from PDF

### Key Pages Implemented:
1. **Hero/Landing Page**
   - Main tagline: "The Platform For Composable, Cognitive, Audit-Grade AI Swarms"
   - Sub-tagline: "Your New Business Brain & Nervous System"
   - Warning disclaimer about AI capabilities

2. **Cognition Page**
   - "No Cognition? - No AGI!" concept
   - Four quadrants explaining Hardware, LLM, Data, and Cognition

3. **AI Breaking at Scale**
   - Three main problems: Fragmentation, Last 10% compute explosion, Lock-in & opacity
   - Root Cause vs Symptom analysis table

4. **Platform Overview**
   - Build Once, Replicate Anywhere
   - Open for Builders
   - Proven Milestones (8M+ micro-transactions)
   - No-code modular agent swarms

5. **SAGE Page**
   - Universal Development Intelligence Platform
   - 100+ Specialized AI Experts
   - Complete Stack Mastery, Mathematical Foundation, Distributed Intelligence

6. **Trust/Security Page**
   - Infrastructure of Integrity
   - Security That Scales
   - Explainable Intelligence
   - Mathematics of Trust

7. **Team Page**
   - Leadership team profiles
   - Technical team members

8. **Roadmap Page**
   - 6-12 months: Building the Foundation
   - 12-18 months: Scaling and Expansion
   - Strategic Growth across verticals

## Current Application Structure

### Technology Stack
- **Language**: Rust
- **Current Framework**: Iced (desktop GUI)
- **Build System**: Nix Flake
- **Package Manager**: Cargo

### File Structure
```
/git/thecowboyai/www/
├── src/
│   ├── main.rs                 # Main application entry
│   ├── main_backup.rs          # Complex version backup
│   ├── pages/                  # Page modules
│   │   ├── mod.rs
│   │   ├── hero.rs
│   │   ├── cognition.rs
│   │   ├── ai_breaking.rs
│   │   ├── platform.rs
│   │   ├── sage.rs
│   │   ├── trust.rs
│   │   ├── team.rs
│   │   └── roadmap.rs
│   ├── theme.rs                # Theme configuration
│   └── components/              # Reusable components
├── Cargo.toml                   # Rust dependencies
├── flake.nix                    # Nix configuration
├── assets/                      # Images and assets
└── DEPLOYMENT.md               # This file
```

## Deployment Requirements

### Current Challenge
The application is built with Iced, which creates native desktop applications. For web deployment on container 134, we need to:

1. **Convert to Web Framework** - Options:
   - Leptos (Rust full-stack web framework)
   - Yew (Rust frontend framework)
   - Axum + static HTML/JS (simple web server)
   - Dioxus (cross-platform Rust framework with web support)

2. **Deployment Method**:
   - Build the web application locally
   - Create a Nix derivation for the web app
   - Deploy to container 134 via SSH
   - Configure web server (nginx/caddy) on container

## Access Instructions

### Connect to Proxmox Host
```bash
ssh -i ~/.ssh/id_cim_thecowboyai -p 22 root@thecowboy.ai
```

### View Container Configuration
```bash
pct config 134
```

### Access Container (from Proxmox host)
```bash
pct enter 134
```

### Direct Container Access (if configured)
```bash
ssh -p 2222 root@thecowboy.ai  # May need different credentials
```

## Next Steps for Deployment

1. **Convert Application to Web**
   - Recommended: Use Leptos for full-stack Rust web app
   - Alternative: Create static HTML/CSS/JS served by Axum

2. **Create Deployment Package**
   ```nix
   # Add to flake.nix
   packages.x86_64-linux.cowboy-ai-web = ...
   ```

3. **Deploy to Container 134**
   - Build the package
   - Copy to container
   - Configure systemd service
   - Setup reverse proxy if needed

4. **Configure Web Access**
   - Domain: thecowboy.ai
   - Local port: 80
   - External port: 8080

## Development Commands

### Build Current Iced App
```bash
nix develop
cargo build
./target/debug/thecowboyai-www
```

### Test Web Version (future)
```bash
cargo leptos watch  # If using Leptos
cargo run --bin web-server  # If using Axum
```

## Important Notes

- The system is on NixOS (as indicated by nix flake configuration)
- Container 134 is unprivileged LXC container
- The application needs to be accessible at thecowboy.ai:8080 (port 80 locally)
- All content has been extracted from the PDF presentation
- Team member information and company details are preserved

## Security Considerations

- Use SSH keys for authentication
- Container is unprivileged for security
- Implement HTTPS for production deployment
- Consider rate limiting and DDoS protection

## Maintenance

- Regular updates to Rust dependencies
- Monitor container resource usage (8GB RAM, 4 cores)
- Backup container configuration and data
- Keep documentation updated with changes