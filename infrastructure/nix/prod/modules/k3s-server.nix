{ config, lib, ... }:

let
  privateInterface = config.app.node.privateInterface;
in
{
  imports = [ ./k3s.nix ];

  services.k3s.role = "server";
  services.k3s.extraFlags = [ "--write-kubeconfig-mode=0640" ];

  networking.firewall.interfaces = lib.mkIf (privateInterface != null) {
    ${privateInterface}.allowedTCPPorts = [
      2379
      2380
      6443
    ];
  };
}
