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
  
  # Hash certificates into Nix store
  certificates = pkgs.stdenv.mkDerivation {
    name = "thecowboy-ai-certificates";
    src = ../certs;
    
    installPhase = ''
      mkdir -p $out
      cp thecowboy.ai.pem $out/thecowboy.ai.pem
      cp thecowboy.ai.key $out/thecowboy.ai.key
      chmod 644 $out/thecowboy.ai.pem
      chmod 600 $out/thecowboy.ai.key
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
      recommendedTlsSettings = true;
      
      virtualHosts."thecowboy.ai" = {
        default = true;
        forceSSL = true;
        enableACME = false; # Using Cloudflare Origin certificates
        
        sslCertificate = "${certificates}/thecowboy.ai.pem";
        sslCertificateKey = "${certificates}/thecowboy.ai.key";
        
        root = wasmSite;
        
        locations."/" = {
          index = "index.html";
          tryFiles = "$uri $uri/ /index.html";
        };
        
        locations."~ \\.wasm$" = {
          extraConfig = ''
            add_header Content-Type application/wasm;
            add_header Cache-Control "public, max-age=31536000";
            add_header Access-Control-Allow-Origin *;
          '';
        };
        
        locations."~ \\.js$" = {
          extraConfig = ''
            add_header Content-Type application/javascript;
            add_header Cache-Control "public, max-age=3600";
          '';
        };
      };
      
      # Also respond to www subdomain
      virtualHosts."www.thecowboy.ai" = {
        forceSSL = true;
        enableACME = false;
        
        sslCertificate = "${certificates}/thecowboy.ai.pem";
        sslCertificateKey = "${certificates}/thecowboy.ai.key";
        
        root = wasmSite;
        
        locations."/" = {
          index = "index.html";
          tryFiles = "$uri $uri/ /index.html";
        };
        
        locations."~ \\.wasm$" = {
          extraConfig = ''
            add_header Content-Type application/wasm;
            add_header Cache-Control "public, max-age=31536000";
            add_header Access-Control-Allow-Origin *;
          '';
        };
        
        locations."~ \\.js$" = {
          extraConfig = ''
            add_header Content-Type application/javascript;
            add_header Cache-Control "public, max-age=3600";
          '';
        };
      };
    };
  };
}