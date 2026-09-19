# Nix

This repository contains Nix manifests to build the NixOS image for production nodes.

## Conventions

If a service should be present once on every node (ex. Prometheus, Node Exporter, Kubernetes), include it into the NixOS image.

Otherwise, the service belongs to [Kubernetes manifests](../kubernetes).

## Services

Each node includes at least the following services:

- fail2ban
- Kubernetes
- Prometheus
- Node Exporter (hardware prometheus metrics)

### Network Management

Port that are not supposed to be exposed are muted: they don't answer to network calls.

This bothers `nmap` recognitions since the client has to wait for a response for each port instead of having a connection denied directly.
