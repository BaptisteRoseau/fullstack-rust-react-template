{ ... }:

{
  imports = [
    ./hardware-configuration.nix
    ./disko.nix
    ../../modules/k3s-server.nix
  ];

  app.node.privateInterface = "eth1";

  services.k3s.clusterInit = true;

  system.stateVersion = "25.11";
}
