{ config, pkgs, lib, ... }:

let
  # Build the WASM presentation site with proper paths
  cowboy-presentation-wasm = pkgs.stdenv.mkDerivation {
    name = "cowboy-presentation-wasm";
    src = ./iced-wasm;
    
    installPhase = ''
      mkdir -p $out
      
      # Copy the index.html (it's in the root of src after unpacking)
      cp index.html $out/
      
      # Copy the WASM package directory
      cp -r pkg $out/pkg
      
      # Fix the import path to use correct relative path
      sed -i 's|./pkg/cowboy_presentation.js|./pkg/cowboy_presentation.js|g' $out/index.html
      
      # Ensure proper permissions
      chmod -R 755 $out
    '';
  };
  
  # Hash certificates into Nix store - they're now in ./certs
  certificates = pkgs.stdenv.mkDerivation {
    name = "thecowboy-ai-certificates";
    src = ./certs;
    
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
  # Basic system configuration
  system.stateVersion = "24.05";
  
  # Network configuration
  networking = {
    hostName = "www";
    useDHCP = false;
    interfaces.eth0 = {
      ipv4.addresses = [{
        address = "10.0.64.134";
        prefixLength = 19;
      }];
    };
    defaultGateway = "10.0.64.1";
    nameservers = [ "8.8.8.8" "8.8.4.4" ];
  };

  # Enable SSH
  services.openssh = {
    enable = true;
    settings = {
      PermitRootLogin = "yes";
      PasswordAuthentication = false;
    };
    ports = [ 22 ];
  };

  # Add SSH key for access
  users.users.root = {
    openssh.authorizedKeys.keys = [
      (builtins.readFile /home/steele/.ssh/id_cim_thecowboyai.pub)
    ];
  };

  # Nginx configuration with SSL
  services.nginx = {
    enable = true;
    recommendedGzipSettings = true;
    recommendedOptimisation = true;
    recommendedProxySettings = true;
    recommendedTlsSettings = true;
    
    virtualHosts."thecowboy.ai" = {
      default = true;
      forceSSL = true;
      enableACME = false; # Using provided certificates
      
      sslCertificate = "${certificates}/thecowboy.ai.pem";
      sslCertificateKey = "${certificates}/thecowboy.ai.key";
      
      root = "${cowboy-presentation-wasm}";
      
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
    
    virtualHosts."www.thecowboy.ai" = {
      forceSSL = true;
      enableACME = false;
      
      sslCertificate = "${certificates}/thecowboy.ai.pem";
      sslCertificateKey = "${certificates}/thecowboy.ai.key";
      
      root = "${cowboy-presentation-wasm}";
      
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

  # Open firewall ports
  networking.firewall = {
    enable = true;
    allowedTCPPorts = [ 22 80 443 ];
  };

}