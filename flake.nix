{
  description = "Cowboy AI Website - Iced WASM Presentation";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    nixos-generators = {
      url = "github:nix-community/nixos-generators";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, nixos-generators, flake-utils }:
  let
    flakeContext = {
      inherit (self) inputs;
      inherit self;
    };
    system = "x86_64-linux";
    
    # Apply rust-overlay to get nightly Rust
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ (import rust-overlay) ];
    };
  in
  {
    nixosModules = {
      nginx = import ./nixosModules/nginx.nix flakeContext;
      system = import ./nixosModules/system.nix flakeContext;
    };

    nixosConfigurations = {
      wwwlxc = nixpkgs.lib.nixosSystem {
        inherit system;
        modules = [
          (import ./nixosModules/system.nix flakeContext)
        ];
      };
    };

    packages = {
      x86_64-linux = {
        wwwlxc = nixos-generators.nixosGenerate {
          inherit system;
          format = "proxmox-lxc";
          modules = [
            (import ./nixosModules/system.nix flakeContext)
          ];
        };
      };
    };
  } // flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [ (import rust-overlay) ];
      
      pkgs = import nixpkgs {
        inherit system overlays;
      };
      
      # Rust nightly with WASM target
      rustNightly = pkgs.rust-bin.nightly.latest.default.override {
        extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
        targets = [ "wasm32-unknown-unknown" ];
      };
      
      # Build WASM script
      buildWasm = pkgs.writeShellScriptBin "build-wasm" ''
        set -e
        echo "🦀 Building WASM with Rust nightly..."
        echo "   Rust: $(rustc --version)"
        cd iced-wasm
        
        # Clean previous builds
        rm -rf pkg
        
        # Build with wasm-pack
        wasm-pack build --target web --out-dir pkg --release
        
        # Optimize WASM with binaryen
        echo "⚡ Optimizing WASM..."
        wasm-opt -Os pkg/*_bg.wasm -o pkg/optimized.wasm
        mv pkg/optimized.wasm pkg/cowboy_presentation_bg.wasm
        
        echo "✅ Build complete! Size: $(du -h pkg/*.wasm)"
      '';
      
      # Development build (faster, with debug info)
      buildDev = pkgs.writeShellScriptBin "build-dev" ''
        set -e
        echo "🔧 Building WASM in development mode..."
        cd iced-wasm
        wasm-pack build --target web --out-dir pkg --dev
        echo "✅ Dev build complete!"
      '';
      
      # Local server
      serve = pkgs.writeShellScriptBin "serve" ''
        echo "🚀 Starting local server at http://localhost:8080"
        cd iced-wasm
        ${pkgs.python3}/bin/python3 -m http.server 8080 --bind 127.0.0.1
      '';
      
      # Dev server (build + serve)
      dev = pkgs.writeShellScriptBin "dev" ''
        set -e
        echo "🔧 Building and serving locally..."
        build-dev
        echo ""
        serve
      '';
      
      # Watch and rebuild on changes
      watch = pkgs.writeShellScriptBin "watch" ''
        echo "👁️  Watching for changes..."
        cd iced-wasm
        ${pkgs.cargo-watch}/bin/cargo-watch -w src -s "wasm-pack build --target web --out-dir pkg --dev"
      '';
      
      # Update all dependencies to latest
      updateDeps = pkgs.writeShellScriptBin "update-deps" ''
        echo "📦 Updating all dependencies to latest versions..."
        cd iced-wasm
        
        # Update Cargo.toml to latest versions
        cargo upgrade --incompatible
        
        # Update Cargo.lock
        cargo update
        
        echo "✅ Dependencies updated!"
      '';
      
    in
    {
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          # Rust nightly toolchain
          rustNightly
          
          # WASM tools
          trunk  # The proper WASM bundler for Iced!
          wasm-pack
          wasm-bindgen-cli
          binaryen  # For wasm-opt
          twiggy    # WASM size profiler
          
          # Cargo tools
          cargo-watch
          cargo-edit     # cargo add/rm/upgrade
          cargo-outdated
          cargo-audit
          cargo-expand
          cargo-bloat
          cargo-flamegraph
          
          # Web dev
          python3
          nodePackages.live-server
          simple-http-server
          miniserve
          
          # Utils
          jq
          ripgrep
          fd
          bat
          hyperfine  # Benchmarking
          tokei      # Code statistics
        ];
        
        nativeBuildInputs = [
          buildWasm
          buildDev
          serve
          dev
          watch
          updateDeps
        ];
        
        shellHook = ''
          echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
          echo "🦀 Cowboy AI WASM Development Environment"
          echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
          echo ""
          echo "🔥 Rust: $(rustc --version | cut -d' ' -f2-4)"
          echo "📦 wasm-pack: $(wasm-pack --version | cut -d' ' -f2)"
          echo "🎯 Target: wasm32-unknown-unknown"
          echo ""
          echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
          echo "Available commands:"
          echo ""
          echo "  dev          - Build (dev mode) and serve locally"
          echo "  build-dev    - Build WASM in dev mode (fast)"
          echo "  build-wasm   - Build optimized WASM for production"
          echo "  serve        - Start local HTTP server"
          echo "  watch        - Auto-rebuild on file changes"
          echo "  update-deps  - Update all deps to latest versions"
          echo ""
          echo "  cargo build --target wasm32-unknown-unknown"
          echo "  cargo run    - Run native version"
          echo ""
          echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
          echo "📁 cd iced-wasm to start working"
          echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        '';
        
        # Set Rust to nightly by default in this shell
        RUSTUP_TOOLCHAIN = "nightly";
        RUST_BACKTRACE = "1";
      };
    }
  );
}