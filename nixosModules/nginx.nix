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
      
      # Check if dist directory exists (from trunk build)
      if [ -d "$src/dist" ]; then
        echo "Using pre-built dist directory"
        cp -r $src/dist/* $out/
      # Fallback to pkg if it exists
      elif [ -d "$src/pkg" ]; then
        echo "Using pkg directory"
        cp $src/index.html $out/
        cp -r $src/pkg $out/wasm-pkg
        cp ${../assets}/logo.svg $out/logo.svg
        cp ${../assets}/favicon.ico $out/favicon.ico
      else
        echo "Warning: No build artifacts found, creating minimal site"
        # Create a minimal index.html
        echo '<html><body><h1>Cowboy AI - Build pending</h1></body></html>' > $out/index.html
      fi
      
      # Ensure proper permissions
      chmod -R 755 $out
    '';
  };
  
  # Certificate paths - included in Nix store for Cloudflare Origin certificates
  certPath = "${../certs/thecowboy.ai.pem}";
  keyPath = "${../certs/thecowboy.ai.key}";
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