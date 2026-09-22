{ ... }:

{
  imports = [ ./k3s.nix ];

  services.k3s.role = "agent";
}
