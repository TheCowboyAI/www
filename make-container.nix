{ pkgs ? import <nixpkgs> {} }:

let
  # Build the static site
  staticSite = pkgs.stdenv.mkDerivation {
    name = "cowboy-ai-static-site";
    src = ./static;
    
    installPhase = ''
      mkdir -p $out
      cp -r * $out/
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

    # Nginx configuration
    services.nginx = {
      enable = true;
      virtualHosts."_" = {
        default = true;
        root = "${staticSite}";
        locations."/" = {
          index = "index.html";
          tryFiles = "$uri $uri/ /index.html";
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

  # Build the container
  nixosSystem = pkgs.nixos containerConfig;
  
in
  nixosSystem.config.system.build.tarball