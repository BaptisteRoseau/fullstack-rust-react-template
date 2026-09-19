# Infrastructure

This directory contains every manifests and config files used to manage infrastructure.

- [docker](./docker): Docker images definition and their assets.
- [docker-compose](./docker-compose): Docker compose manifests for local dev environments.
- [nix](./nix): Nix manifests for setting up bare metal servers
- [kubernetes](./kubernetes): Kubernetes manifests to run production services
- [configs](./configs): Services configuration files

This project is provider-agnostic: you can use your own machines or rent VMs.
Install the NixOS image, register the node into kubernetes, apply the manifests and you are good to go.
