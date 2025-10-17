{
  description = "Cowboy AI Presentation - Iced.rs WASM Application";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            wasm-pack
            wasm-bindgen-cli
            binaryen
            
            # Development tools
            cargo-watch
            cargo-edit
            
            # Web server for testing
            python3
            
            # Build essentials
            pkg-config
            openssl
            
            # Graphics libraries for native testing
            libGL
            libGLU
            xorg.libX11
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            wayland
            libxkbcommon
          ];
          
          shellHook = ''
            echo "Cowboy AI Presentation - Iced.rs Development Environment"
            echo ""
            echo "Available commands:"
            echo "  cargo build              - Build native version"
            echo "  cargo run                - Run native version"
            echo "  ./build.sh               - Build WASM version"
            echo "  python3 -m http.server   - Serve WASM version locally"
            echo ""
            echo "Rust toolchain: $(rustc --version)"
            echo "WASM target: wasm32-unknown-unknown"
          '';
          
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.libGL
            pkgs.libGLU
            pkgs.xorg.libX11
            pkgs.xorg.libXcursor
            pkgs.xorg.libXrandr
            pkgs.xorg.libXi
            pkgs.wayland
            pkgs.libxkbcommon
          ];
        };
      });
}