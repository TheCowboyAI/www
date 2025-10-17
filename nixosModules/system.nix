{ inputs, self, ... }@flakeContext:
{ config, lib, pkgs, ... }: {
  imports = [
    (import ./nginx.nix flakeContext)
  ];
  config = {
    system.stateVersion = "24.05";

    networking = {
      hostName = "www";
      domain = "thecowboy.ai";

      enableIPv6 = false;

      defaultGateway = {
        address = "10.0.64.1";
        interface = "eth0";
      };
      nameservers = [ "10.0.0.254" "1.1.1.1" ];

      interfaces.eth0.useDHCP = false;
      interfaces.eth0.ipv4.addresses = [ {
        address = "10.0.64.134";
        prefixLength = 19;
      } ];

      firewall = {
        enable = true;
        allowedTCPPorts = [
          22
          80
          443
        ];
      };
    };

    services = {
      openssh = {
        enable = true;
        settings = {
          PasswordAuthentication = false;
        };
      };
    };
    
    users = {
      users = {
        root = {
          openssh = {
            authorizedKeys = {
              keys = [
                "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIDecTwCL7tc0mzBabAsFp1k9C3G30Nr+LIOE4MW4KWNO steele@thecowboy.ai"
              ];
            };
          };
          password = "root";
        };
      };
    };
  };
}