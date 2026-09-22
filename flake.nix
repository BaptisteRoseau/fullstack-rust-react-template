{
  description = "Fullstack Rust + React development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    disko = {
      url = "github:nix-community/disko";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      disko,
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forAllSystems = builder: nixpkgs.lib.genAttrs systems (system: builder nixpkgs.legacyPackages.${system});

      sqlGenFor = pkgs: pkgs.callPackage ./infrastructure/nix/dev/sql-gen.nix { };

      nodes = [
        "node-01"
        "node-02"
      ];
      mkNode =
        host:
        nixpkgs.lib.nixosSystem {
          system = "x86_64-linux";
          modules = [
            disko.nixosModules.disko
            ./infrastructure/nix/prod/modules/base.nix
            ./infrastructure/nix/prod/modules/hardening.nix
            ./infrastructure/nix/prod/modules/monitoring.nix
            ./infrastructure/nix/prod/hosts/${host}
            { networking.hostName = host; }
          ];
        };
    in
    {
      nixosConfigurations = nixpkgs.lib.genAttrs nodes mkNode;

      packages = forAllSystems (pkgs: {
        sql-gen = sqlGenFor pkgs;
      });

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            rustc
            cargo
            clippy
            rustfmt
            rust-analyzer
            cargo-llvm-cov
            cargo-audit
            cargo-deny
            sqlx-cli
            (sqlGenFor pkgs)

            bun
            nodejs_22

            docker-compose
            kubectl
            kustomize
            kubeconform

            markdownlint-cli
            cspell
            typos
            shellcheck
            shfmt

            nixfmt-rfc-style
            statix
            deadnix

            pkg-config
            openssl
            git
            jq
            yq-go
          ];

          env = {
            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          };

          shellHook = ''
            echo "Dev shell ready: $(cargo --version), bun $(bun --version)"
          '';
        };
      });

      formatter = forAllSystems (pkgs: pkgs.nixfmt-rfc-style);
    };
}
