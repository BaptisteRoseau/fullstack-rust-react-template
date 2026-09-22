# Production Nodes

NixOS configurations for the bare metal servers and VMs running the Kubernetes cluster.

A node configuration describes **the machine**, never the application: the operating system, the
users allowed in, the firewall, and the cluster agent that registers the node. Everything that
ships a new version when the backend changes lives in [../../kubernetes](../../kubernetes),
otherwise every release would become a reboot.

Nodes run [k3s](https://k3s.io) rather than upstream Kubernetes: a single binary, a single NixOS
module, and the same manifests apply to it.

## Layout

```txt
prod/
├── modules/                        # Reusable, parameterized, no machine-specific value
│   ├── base.nix                    # Users, SSH, Nix settings, bootloader, locale
│   ├── hardening.nix               # fail2ban, sysctl, firewall defaults
│   ├── monitoring.nix              # Node Exporter
│   ├── k3s.nix                     # Settings shared by both cluster roles
│   ├── k3s-server.nix              # Control plane
│   └── k3s-agent.nix               # Worker
└── hosts/                          # One directory per machine, holding only its own facts
    └── <hostname>/
        ├── default.nix             # Role, network interface, state version
        ├── disko.nix               # Disk layout
        └── hardware-configuration.nix
```

The machines themselves are declared in the repository's root [flake.nix](../../../flake.nix),
which imports `modules/base.nix`, `modules/hardening.nix` and `modules/monitoring.nix` into every
node, then the host directory. A node therefore appears as `nixosConfigurations.<hostname>`, and
shares its `flake.lock` with the development shell: the nixpkgs running in production is the one
pinned in git.

## Conventions

- **One directory per machine** under `hosts/`, named after its hostname. The directory name is
  the attribute name in `nixosConfigurations`, and `networking.hostName` is set from it.
- **Host files only hold what differs between machines**: role, interface names, addresses, disks,
  state version. Anything shared belongs in a module.
- **Modules never mention a machine.** A module needing a machine-specific value declares an
  option for it (`app.node.privateInterface` in `base.nix`) and the host sets it.
- **Never fork a module to change one value**, set the option from the host file instead.
- `hardware-configuration.nix` is **generated on the machine**, never hand-written:

    ```sh
    nixos-generate-config --show-hardware-config
    ```

  The file committed here is a placeholder for a virtualized x86-64 guest, which is what most VPS
  providers give you. Replace it with the real output after the first install.
- **`system.stateVersion` is set once per host and never changed.** It records the NixOS release
  the machine was installed with, not the release it runs.
- **No automatic upgrade timer.** Upgrades happen when `flake.lock` is bumped and the node is
  deployed, so a rollback is a previous generation rather than an investigation.

## Node Roles

Three identities, kept apart:

| Identity | Where it is set | Used for |
| --- | --- | --- |
| Machine name | `networking.hostName`, from the host directory name | Deploying, SSH, `kubectl get nodes` |
| Cluster role | The `k3s-server.nix` or `k3s-agent.nix` import | Control plane or worker |
| Scheduling | `services.k3s.extraFlags` node labels | `nodeSelector` in the Kubernetes overlays |

The first server also sets `services.k3s.clusterInit`, which bootstraps etcd. Every other server
and every agent joins it through `services.k3s.serverAddr`. Both roles read the same join token.

[node-01](./hosts/node-01) is the control plane, [node-02](./hosts/node-02) a worker. Adding a
machine means copying a host directory, adjusting it, and adding its name to `nodes` in the root
flake.

## Network

`app.node.privateInterface` names the interface carrying cluster traffic. Ports needed between
nodes are opened **on that interface only**, so nothing internal is reachable from the internet:

| Port | Interface | Role | Service |
| --- | --- | --- | --- |
| 22 | all | both | SSH |
| 80, 443 | all | both | Ingress, through the k3s load balancer |
| 6443 | private | server | Kubernetes API |
| 2379, 2380 | private | server | etcd |
| 10250 | private | both | Kubelet |
| 8472/udp | private | both | Flannel VXLAN |
| 9100 | private | both | Node Exporter, scraped by Prometheus |

Leaving `app.node.privateInterface` unset opens none of the internal ports: a single-node cluster
works, a multi-node one does not. This is deliberate — the alternative default would expose etcd
and the kubelet to the public interface.

Unexposed ports are dropped rather than rejected, which is the NixOS firewall default: a port scan
has to wait for a timeout on every port instead of getting an immediate refusal.

Traefik is disabled because the Kubernetes overlays use the nginx ingress controller and
cert-manager, see [../../kubernetes](../../kubernetes).

## Secrets

The Nix store is world readable, so **a secret written in a `.nix` file is readable by every
process on the machine**. Secrets are files on the node, provisioned out of band, and referenced
by path:

```sh
install -Dm400 -o root -g root /dev/stdin /var/lib/secrets/k3s-token <<< "$TOKEN"
```

`/var/lib/secrets/k3s-token` holds the cluster join token, the same value on every node. Generate
it once with `openssl rand -hex 32`.

Once there is more than one secret, use [sops-nix](https://github.com/Mic92/sops-nix) or
[agenix](https://github.com/ryantm/agenix): encrypted files committed to the repository, decrypted
at activation time into `/run/secrets` with real ownership.

## Build and Install

Nothing below has been run yet: the configurations evaluate against nixpkgs but no machine exists.

Before anything else, write your SSH public keys into `authorized_keys` in this directory, one per
line, `#` starting a comment:

```sh
cat ~/.ssh/id_ed25519.pub >> infrastructure/nix/prod/authorized_keys
```

[modules/base.nix](./modules/base.nix) reads that file into
`users.users.admin.openssh.authorizedKeys.keys`, and an assertion fails the build while it is
missing or empty, because a node with no key and no password login is a node nobody can log into.
[scripts/test_lint_nix.sh](../../../scripts/test_lint_nix.sh) evaluates the nodes with a lint-only
key so that the `pre-push` hook still checks the rest of the manifests without it, which is why a
green hook does not mean a node is deployable.

The file is yours, not the repository's: every machine provisions its own, and nothing commits it.
It is not in `.gitignore` on purpose, because a flake only sees the files git knows about plus the
untracked ones, and an ignored file would be invisible to `nix flake check`.

Build a node without a machine, to check it evaluates and compiles:

```sh
nix build .#nixosConfigurations.node-01.config.system.build.toplevel
```

Run it locally in QEMU, with a throwaway disk:

```sh
nixos-rebuild build-vm --flake .#node-01
./result/bin/run-node-01-vm
```

Install onto a fresh server. [nixos-anywhere](https://github.com/nix-community/nixos-anywhere)
takes over a machine running any Linux the provider installed, partitions it following
`disko.nix`, and reboots into NixOS. Only SSH access as root is needed:

```sh
nix run github:nix-community/nixos-anywhere -- --flake .#node-01 root@<address>
```

Deploy a change to a running node:

```sh
nixos-rebuild switch --flake .#node-01 --target-host admin@<address> --use-remote-sudo
```

It builds locally, copies only what changed, and keeps the previous generation in the bootloader.
`nixos-rebuild --rollback` goes back.

## Rules

- This directory is part of the `infrastructure` and does not know about the rest of the repository.
- A node runs the operating system, the cluster agent and the per-node services only. Application
  workloads belong to [../../kubernetes](../../kubernetes).
- Never commit a secret, an SSH private key or a join token. Secrets are files on the node.
- Never write a machine-specific value in `modules/`, nor shared configuration in `hosts/`.
- Always pin the nixpkgs revision through the root `flake.lock`, never through a channel.
