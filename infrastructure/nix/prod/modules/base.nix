{
  config,
  lib,
  pkgs,
  ...
}:

let
  authorizedKeysFile = ../authorized_keys;
  authorizedKeys =
    if builtins.pathExists authorizedKeysFile then
      lib.filter (line: line != "" && !(lib.hasPrefix "#" line)) (
        lib.splitString "\n" (builtins.readFile authorizedKeysFile)
      )
    else
      [ ];
in
{
  options.app.node.privateInterface = lib.mkOption {
    type = lib.types.nullOr lib.types.str;
    default = null;
    example = "eth1";
    description = "Interface carrying cluster traffic. Internal ports are opened on it only.";
  };

  config = {
    boot = {
      loader.systemd-boot.enable = true;
      loader.efi.canTouchEfiVariables = true;
      tmp.cleanOnBoot = true;
    };

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
      openssh.authorizedKeys.keys = authorizedKeys;
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

    services.journald.settings.Journal.SystemMaxUse = "2G";

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
        message = "infrastructure/nix/prod/authorized_keys is missing or empty: the node would be unreachable. Write your SSH public keys in it, one per line.";
      }
    ];
  };
}
