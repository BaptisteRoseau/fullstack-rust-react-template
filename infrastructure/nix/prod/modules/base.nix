{ config, lib, pkgs, ... }:

{
  options.app.node.privateInterface = lib.mkOption {
    type = lib.types.nullOr lib.types.str;
    default = null;
    example = "eth1";
    description = "Interface carrying cluster traffic. Internal ports are opened on it only.";
  };

  config = {
    boot.loader.systemd-boot.enable = true;
    boot.loader.efi.canTouchEfiVariables = true;
    boot.tmp.cleanOnBoot = true;

    time.timeZone = "UTC";
    i18n.defaultLocale = "en_US.UTF-8";

    nix.settings = {
      experimental-features = [
        "nix-command"
        "flakes"
      ];
      auto-optimise-store = true;
      trusted-users = [
        "root"
        "@wheel"
      ];
    };

    nix.gc = {
      automatic = true;
      dates = "weekly";
      options = "--delete-older-than 30d";
    };

    users.mutableUsers = false;
    users.users.admin = {
      isNormalUser = true;
      extraGroups = [ "wheel" ];
      openssh.authorizedKeys.keys = [ ];
    };

    security.sudo.wheelNeedsPassword = false;

    services.openssh = {
      enable = true;
      settings = {
        PermitRootLogin = "no";
        PasswordAuthentication = false;
        KbdInteractiveAuthentication = false;
        X11Forwarding = false;
      };
    };

    services.journald.extraConfig = "SystemMaxUse=2G";

    environment.systemPackages = with pkgs; [
      curl
      git
      htop
      jq
    ];

    networking.firewall.enable = true;
    documentation.nixos.enable = false;

    assertions = [
      {
        assertion = config.users.users.admin.openssh.authorizedKeys.keys != [ ];
        message = "users.users.admin.openssh.authorizedKeys.keys is empty: the node would be unreachable.";
      }
    ];
  };
}
