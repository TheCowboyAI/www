{ pkgs ? import <nixpkgs> {} }:

let
  # Build the WASM presentation
  wasmPresentation = pkgs.stdenv.mkDerivation {
    name = "cowboy-presentation-wasm";
    version = "1.0.0";
    
    src = ./iced-wasm;
    
    installPhase = ''
      mkdir -p $out
      
      # Copy the index.html
      cp ${./iced-wasm}/index.html $out/
      
      # Copy the WASM package directory
      cp -r ${./iced-wasm}/pkg $out/wasm-pkg
      
      # Fix the import path in index.html to match our structure
      sed -i 's|./pkg/cowboy_presentation.js|./wasm-pkg/cowboy_presentation.js|g' $out/index.html
      
      # Ensure proper permissions
      chmod -R 755 $out
    '';
  };

  # NixOS configuration for the container
  containerConfig = {
    imports = [ ];
    
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
    };
    
    # Add SSH key for root access
    users.users.root = {
      openssh.authorizedKeys.keys = [
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIJ2RaQ9cCyLZiPmm5FhzxCuSS8S2j2BW2T7h17zanOhw steele@nixos"
      ];
    };

    # Nginx configuration
    services.nginx = {
      enable = true;
      recommendedGzipSettings = true;
      recommendedOptimisation = true;
      
      virtualHosts."_" = {
        default = true;
        root = "${wasmPresentation}";
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
        locations."~ \\.js$" = {
          extraConfig = ''
            add_header Content-Type application/javascript;
          '';
        };
      };
    };

    # Open firewall ports
    networking.firewall = {
      enable = true;
      allowedTCPPorts = [ 22 80 ];
    };

    # Minimal system
    boot.isContainer = true;
    
    # Add basic packages
    environment.systemPackages = with pkgs; [
      vim
      curl
      htop
    ];
  };

  # Build the container system
  nixosSystem = (pkgs.nixos containerConfig).config.system.build.toplevel;
  
  # Create LXC tarball
  tarball = pkgs.runCommand "cowboy-ai-container.tar.gz" {
    buildInputs = [ pkgs.gnutar pkgs.gzip ];
  } ''
    mkdir -p $out
    
    # Create container root
    mkdir -p container/rootfs
    
    # Copy the NixOS system
    cp -r ${nixosSystem}/* container/rootfs/ || true
    
    # Create the tarball
    tar -czf $out/cowboy-ai-container.tar.gz -C container .
    
    echo "Container tarball created at $out/cowboy-ai-container.tar.gz"
  '';
  
in
  tarball