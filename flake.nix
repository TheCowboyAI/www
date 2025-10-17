{
  description = "Cowboy AI Website";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    nixos-generators.url = "github:nix-community/nixos-generators";
    nixos-generators.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, nixos-generators, flake-utils }:
  let
    flakeContext = {
      inherit (self) inputs;
      inherit self;
    };
  in
  {
    nixosModules = {
      nginx = import ./nixosModules/nginx.nix flakeContext;
      system = import ./nixosModules/system.nix flakeContext;
    };

    packages = {
      x86_64-linux = {
        wwwlxc = import ./packages/www-lxc.nix flakeContext;  
      };
    };
  } // flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
      };
      
      # Elm build script
      buildElm = pkgs.writeShellScriptBin "build-elm" ''
        echo "Building Elm application..."
        elm make src/Main.elm --optimize --output=static/app.js
        
        echo "Minifying JavaScript..."
        ${pkgs.esbuild}/bin/esbuild static/app.js \
          --minify \
          --outfile=static/app.min.js \
          --allow-overwrite
        
        echo "Elm build complete! Files in ./static/"
      '';
      
      # Development server
      serveElm = pkgs.writeShellScriptBin "serve-elm" ''
        echo "Starting Elm development server on http://localhost:8000"
        elm-live src/Main.elm --open --dir=static -- --output=static/app.js --optimize
      '';
      
      # Static server for testing
      serveStatic = pkgs.writeShellScriptBin "serve-static" ''
        echo "Serving static files on http://localhost:8080"
        ${pkgs.python3}/bin/python3 -m http.server 8080 --directory ./static
      '';
      
    in
    with pkgs;
    {
      devShells.default = mkShell {
        buildInputs = [
          elmPackages.elm
          elmPackages.elm-format
          elmPackages.elm-live
          elmPackages.elm-analyse
          elmPackages.elm-test
          nodePackages.elm-oracle
          esbuild
          python3
        ];
        
        nativeBuildInputs = [
          buildElm
          serveElm
          serveStatic
        ];
        
        shellHook = ''
          echo "Cowboy AI Elm Development Environment"
          echo ""
          echo "Available commands:"
          echo "  build-elm     - Build and optimize the Elm application"
          echo "  serve-elm     - Start Elm development server with live reload"
          echo "  serve-static  - Serve the static files"
          echo ""
        '';
      };
    });
}