{ inputs, self, ... }@flakeContext:
{ config, lib, pkgs, ... }: 
let
  # Build the WASM presentation site
  wasmSite = pkgs.stdenv.mkDerivation {
    name = "cowboy-ai-website";
    src = ../iced-wasm;
    
    buildInputs = [ pkgs.coreutils ];
    
    installPhase = ''
      mkdir -p $out
      
      # Copy the index.html from source
      cp $src/index.html $out/
      
      # Copy the WASM package from source
      cp -r $src/pkg $out/wasm-pkg
      
      # Copy assets
      cp ${../assets}/logo.svg $out/logo.svg
      cp ${../assets}/favicon.ico $out/favicon.ico
      
      # The import path should already be correct in index.html
      # No need to modify it
      
      # Ensure proper permissions
      chmod -R 755 $out
    '';
  };
  
  # Certificate paths - these should be manually deployed to /etc/ssl/certs/
  # NOT included in Nix store for security reasons
  certPath = "/etc/ssl/certs/thecowboy.ai.pem";
  keyPath = "/etc/ssl/private/thecowboy.ai.key";
in
{
  config = {
    # Note: SSL certificates must be manually deployed to the server
    # Place them in /etc/ssl/certs/ and /etc/ssl/private/
    
    services.nginx = {
      enable = true;
      recommendedGzipSettings = true;
      recommendedOptimisation = true;
      recommendedProxySettings = true;
      recommendedTlsSettings = true;
      
      virtualHosts."thecowboy.ai" = {
        default = true;
        forceSSL = false;  # Don't force SSL - Cloudflare handles this
        addSSL = true;     # Support both HTTP and HTTPS
        enableACME = false; # Using Cloudflare Origin certificates
        
        sslCertificate = certPath;
        sslCertificateKey = keyPath;
        
        root = wasmSite;
        
        locations."/" = {
          index = "index.html";
          tryFiles = "$uri $uri/ /index.html";
        };
        
        # Serve wasm-pkg directory with proper MIME types
        locations."/wasm-pkg/" = {
          priority = 1;
          alias = "${wasmSite}/wasm-pkg/";
          extraConfig = ''
            add_header Access-Control-Allow-Origin *;
          '';
        };
        
        locations."~ \\.js$" = {
          priority = 2;
          root = wasmSite;
          extraConfig = ''
            add_header Content-Type "application/javascript; charset=utf-8";
            add_header Cache-Control "public, max-age=3600";
            add_header Access-Control-Allow-Origin "*";
            add_header Access-Control-Allow-Methods "GET, OPTIONS";
          '';
        };
        
        locations."~ \\.wasm$" = {
          priority = 2;
          root = wasmSite;
          extraConfig = ''
            add_header Content-Type application/wasm;
            add_header Cache-Control "public, max-age=31536000";
            add_header Access-Control-Allow-Origin *;
          '';
        };
      };
      
      # Also respond to www subdomain
      virtualHosts."www.thecowboy.ai" = {
        forceSSL = false;  # Don't force SSL - Cloudflare handles this
        addSSL = true;     # Support both HTTP and HTTPS
        enableACME = false;
        
        sslCertificate = certPath;
        sslCertificateKey = keyPath;
        
        root = wasmSite;
        
        locations."/" = {
          index = "index.html";
          tryFiles = "$uri $uri/ /index.html";
        };
        
        # Serve wasm-pkg directory with proper MIME types
        locations."/wasm-pkg/" = {
          priority = 1;
          alias = "${wasmSite}/wasm-pkg/";
          extraConfig = ''
            add_header Access-Control-Allow-Origin *;
          '';
        };
        
        locations."~ \\.js$" = {
          priority = 2;
          root = wasmSite;
          extraConfig = ''
            add_header Content-Type "application/javascript; charset=utf-8";
            add_header Cache-Control "public, max-age=3600";
            add_header Access-Control-Allow-Origin "*";
            add_header Access-Control-Allow-Methods "GET, OPTIONS";
          '';
        };
        
        locations."~ \\.wasm$" = {
          priority = 2;
          root = wasmSite;
          extraConfig = ''
            add_header Content-Type application/wasm;
            add_header Cache-Control "public, max-age=31536000";
            add_header Access-Control-Allow-Origin *;
          '';
        };
      };
    };
  };
}