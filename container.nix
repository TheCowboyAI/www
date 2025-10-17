{ config, pkgs, lib, ... }:

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

  # Nginx configuration
  services.nginx = {
    enable = true;
    virtualHosts."thecowboy.ai" = {
      default = true;
      root = "/var/www/html";
      locations."/" = {
        index = "index.html";
        tryFiles = "$uri $uri/ /index.html";
      };
    };
  };

  # Open firewall ports
  networking.firewall = {
    enable = true;
    allowedTCPPorts = [ 22 80 443 ];
  };

  # Copy static files
  system.activationScripts.copyWebFiles = ''
    mkdir -p /var/www/html
    cp -r ${./static}/* /var/www/html/
    chown -R nginx:nginx /var/www/html
  '';
}