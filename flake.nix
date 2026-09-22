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
      nixpkgs,
      disko,
      ...
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forAllSystems =
        builder: nixpkgs.lib.genAttrs systems (system: builder nixpkgs.legacyPackages.${system});

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
            markdown-link-check
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

            # cargo-llvm-cov reads the profiles rustc's instrumentation writes, so
            # its llvm-profdata has to be the one rustc was built against rather
            # than whatever LLVM nixpkgs defaults to. Without these it stops at
            # "failed to find llvm-tools-preview".
            LLVM_COV = "${pkgs.rustc.llvmPackages.llvm}/bin/llvm-cov";
            LLVM_PROFDATA = "${pkgs.rustc.llvmPackages.llvm}/bin/llvm-profdata";
          };

          shellHook = ''
            # openssl-sys links libssl dynamically and the binary carries no rpath,
            # so the test and dev binaries need it on the runtime path too.
            export LD_LIBRARY_PATH="${
              pkgs.lib.makeLibraryPath [ pkgs.openssl ]
            }''${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"

            echo "Dev shell ready: $(cargo --version), bun $(bun --version)"
          '';
        };
      });

      formatter = forAllSystems (pkgs: pkgs.nixfmt-rfc-style);
    };
}
