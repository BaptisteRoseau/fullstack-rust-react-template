{ config, lib, ... }:

let
  privateInterface = config.app.node.privateInterface;
in
{
  services.k3s = {
    enable = true;
    tokenFile = "/var/lib/secrets/k3s-token";
    extraFlags = [ "--disable=traefik" ];
  };

  boot.kernelModules = [
    "br_netfilter"
    "overlay"
  ];

  networking.firewall.allowedTCPPorts = [
    80
    443
  ];

  networking.firewall.interfaces = lib.mkIf (privateInterface != null) {
    ${privateInterface} = {
      allowedTCPPorts = [ 10250 ];
      allowedUDPPorts = [ 8472 ];
    };
  };
}
