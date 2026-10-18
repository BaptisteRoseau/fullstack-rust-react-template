{ config, lib, ... }:

let
  privateInterface = config.app.node.privateInterface;
  k3s = config.services.k3s;
  alloyPort = 12345;
  alloyConfigs = ../../../configs/alloy;
  alloyEnvironmentFile = "/var/lib/secrets/alloy.env";
  k3sServerHost = builtins.head (builtins.match "https?://([^:/]+).*" k3s.serverAddr);
in
{
  config = {
    services.alloy = {
      enable = true;
      environmentFile = alloyEnvironmentFile;
      extraFlags = [
        "--server.http.listen-addr=0.0.0.0:${toString alloyPort}"
        "--disable-reporting"
        "--cluster.enabled=true"
        "--cluster.name=nodes"
      ]
      ++ lib.optional (privateInterface != null) "--cluster.advertise-interfaces=${privateInterface}"
      ++ lib.optional (
        k3s.serverAddr != ""
      ) "--cluster.join-addresses=${k3sServerHost}:${toString alloyPort}";
    };

    environment.etc = lib.listToAttrs (
      map (name: lib.nameValuePair "alloy/${name}.alloy" { source = alloyConfigs + "/${name}.alloy"; }) [
        "outputs"
        "host"
        "journal"
        "kubernetes"
      ]
    );

    systemd.services.alloy = {
      after = [ "k3s.service" ];

      environment = {
        NODE_NAME = config.networking.hostName;
        KUBERNETES_API_URL =
          if k3s.role == "server" then "https://127.0.0.1:6443" else "https://127.0.0.1:6444";
      };

      serviceConfig = {
        AmbientCapabilities = [ "CAP_DAC_READ_SEARCH" ];
        CapabilityBoundingSet = [ "CAP_DAC_READ_SEARCH" ];
      };
    };

    services.k3s.manifests = lib.mkIf (k3s.role == "server") {
      alloy.content = import ./alloy-rbac.nix;
    };

    networking.firewall.interfaces = lib.mkIf (privateInterface != null) {
      ${privateInterface}.allowedTCPPorts = [ alloyPort ];
    };
  };
}
