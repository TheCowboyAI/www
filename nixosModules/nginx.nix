{ inputs, self, ... }@flakeContext:
{ config, lib, pkgs, ... }: 
let
  # Build the static site
  staticSite = pkgs.stdenv.mkDerivation {
    name = "cowboy-ai-website";
    src = ../static;
    
    installPhase = ''
      mkdir -p $out
      cp -r * $out/
    '';
  };
in
{
  config = {
    services.nginx = {
      enable = true;
      
      virtualHosts."thecowboy.ai" = {
        default = true;
        root = staticSite;
        
        locations."/" = {
          index = "index.html";
          tryFiles = "$uri $uri/ /index.html";
        };
      };
      
      # Also respond to www subdomain
      virtualHosts."www.thecowboy.ai" = {
        root = staticSite;
        
        locations."/" = {
          index = "index.html";
          tryFiles = "$uri $uri/ /index.html";
        };
      };
    };
  };
}