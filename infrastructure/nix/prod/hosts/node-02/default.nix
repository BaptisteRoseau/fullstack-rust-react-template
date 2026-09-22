{ ... }:

{
  imports = [
    ./hardware-configuration.nix
    ./disko.nix
    ../../modules/k3s-agent.nix
  ];

  app.node.privateInterface = "eth1";

  services.k3s.serverAddr = "https://10.0.0.1:6443";

  system.stateVersion = "25.11";
}
