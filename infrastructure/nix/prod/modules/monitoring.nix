{ config, lib, ... }:

let
  privateInterface = config.app.node.privateInterface;
in
{
  services.prometheus.exporters.node = {
    enable = true;
    port = 9100;
    openFirewall = false;
    enabledCollectors = [ "systemd" ];
  };

  networking.firewall.interfaces = lib.mkIf (privateInterface != null) {
    ${privateInterface}.allowedTCPPorts = [ 9100 ];
  };
}
