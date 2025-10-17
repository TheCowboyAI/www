{ pkgs ? import <nixpkgs> {} }:

let
  # Build the WASM presentation site
  wasmSite = pkgs.stdenv.mkDerivation {
    name = "cowboy-presentation-wasm";
    src = ./iced-wasm;
    
    installPhase = ''
      mkdir -p $out
      cp $src/index.html $out/
      cp -r $src/pkg $out/
      chmod -R 755 $out
    '';
  };

  # Create the container configuration
  containerConfig = {
    imports = [ ];
    
    system.stateVersion = "24.05";
    boot.isContainer = true;
    
    # Network configuration for container 134
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
      nameservers = [ "10.0.0.254" "1.1.1.1" ];
      firewall = {
        enable = true;
        allowedTCPPorts = [ 22 80 443 ];
      };
    };

    # SSH configuration
    services.openssh = {
      enable = true;
      ports = [ 8822 ];  # Container 134 uses port 8822
      settings = {
        PermitRootLogin = "yes";
        PasswordAuthentication = false;
      };
    };
    
    # Root SSH access with the correct key
    users.users.root = {
      openssh.authorizedKeys.keys = [
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIDecTwCL7tc0mzBabAsFp1k9C3G30Nr+LIOE4MW4KWNO steele@thecowboy.ai"
      ];
    };

    # Nginx configuration for WASM app
    services.nginx = {
      enable = true;
      recommendedGzipSettings = true;
      recommendedOptimisation = true;
      
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

    # Basic packages
    environment.systemPackages = with pkgs; [
      vim
      curl
      htop
      python3
    ];
  };

  # Build the NixOS system
  nixosSystem = (pkgs.nixos containerConfig).config.system.build.toplevel;

in
  # Create LXC rootfs tarball
  pkgs.runCommand "lxc-container-134.tar.gz" {} ''
    mkdir -p $out
    mkdir -p rootfs/nix/store
    mkdir -p rootfs/etc
    mkdir -p rootfs/var
    mkdir -p rootfs/tmp
    mkdir -p rootfs/dev
    mkdir -p rootfs/proc
    mkdir -p rootfs/sys
    
    # Copy the NixOS closure
    cp -a ${nixosSystem} rootfs/nix-system
    
    # Create init script
    cat > rootfs/init << 'EOF'
    #!/bin/sh
    exec /nix-system/init
    EOF
    chmod +x rootfs/init
    
    # Create the tarball
    tar czf $out/rootfs.tar.gz -C rootfs .
    echo "LXC container tarball created at $out/rootfs.tar.gz"
  ''