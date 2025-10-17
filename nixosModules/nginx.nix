{ inputs, self, ... }@flakeContext:
{ config, lib, pkgs, ... }: 
let
  # Build the WASM presentation site
  wasmSite = pkgs.stdenv.mkDerivation {
    name = "cowboy-ai-website";
    src = ../iced-wasm;
    
    installPhase = ''
      mkdir -p $out
      
      # Copy the index.html from source
      cp $src/index.html $out/
      
      # Copy the WASM package from source
      cp -r $src/pkg $out/wasm-pkg
      
      # Fix the import path in index.html
      sed -i 's|./pkg/cowboy_presentation.js|./wasm-pkg/cowboy_presentation.js|g' $out/index.html
      
      # Ensure proper permissions
      chmod -R 755 $out
    '';
  };
in
{
  config = {
    services.nginx = {
      enable = true;
      recommendedGzipSettings = true;
      recommendedOptimisation = true;
      recommendedProxySettings = true;
      
      virtualHosts."thecowboy.ai" = {
        default = true;
        root = wasmSite;
        
        locations."/" = {
          index = "index.html";
          tryFiles = "$uri $uri/ /index.html";
        };
        
        locations."~ \\.wasm$" = {
          extraConfig = ''
            add_header Content-Type application/wasm;
            add_header Cache-Control "public, max-age=31536000";
          '';
        };
      };
      
      # Also respond to www subdomain
      virtualHosts."www.thecowboy.ai" = {
        root = wasmSite;
        
        locations."/" = {
          index = "index.html";
          tryFiles = "$uri $uri/ /index.html";
        };
        
        locations."~ \\.wasm$" = {
          extraConfig = ''
            add_header Content-Type application/wasm;
            add_header Cache-Control "public, max-age=31536000";
          '';
        };
      };
    };
  };
}