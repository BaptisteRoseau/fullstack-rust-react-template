#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

if ! command -v nixfmt >/dev/null; then
    echo "nixfmt is not installed, skipping the nix manifests" >&2
    echo "Run this script from 'nix develop' to get it" >&2
    exit 0
fi

NIX_FILES=$(find "$GIT_ROOT" -name "*.nix" -not -path "*/node_modules/*" -not -path "*/target/*")

nixfmt --check $NIX_FILES

statix check "$GIT_ROOT"

deadnix --fail "$GIT_ROOT"

# `nix flake check` would take every node down to its `system.build.toplevel`,
# where base.nix asserts that infrastructure/nix/prod/authorized_keys holds a
# key. That file is provisioned per machine and absent from a fresh clone, so
# the outputs are evaluated one by one instead and the nodes get a lint-only
# key. Everything except the key itself is checked; the real one is what
# `nix build .#nixosConfigurations.<node>...` verifies at deploy time.
SYSTEM=$(nix eval --impure --raw --expr builtins.currentSystem)

for output in "devShells.$SYSTEM.default" "packages.$SYSTEM.sql-gen"; do
    echo "Evaluating $output"
    nix eval --raw ".#$output.drvPath" >/dev/null
done

LINT_SSH_KEY="ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA lint@example"

for node in $(nix eval --json --apply builtins.attrNames .#nixosConfigurations | jq -r '.[]'); do
    echo "Evaluating nixosConfigurations.$node"
    nix eval --raw ".#nixosConfigurations.$node" --apply "
        node:
        (node.extendModules {
          modules = [
            { users.users.admin.openssh.authorizedKeys.keys = [ \"$LINT_SSH_KEY\" ]; }
          ];
        }).config.system.build.toplevel.drvPath" >/dev/null
done
