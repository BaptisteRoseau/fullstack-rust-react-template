# Monitoring

Contains all the tools used to monitor the application:

- Grafana
- Postgres Exporter

One directory per component, each with its own `kustomization.yaml`. See
[../README.md](../README.md) for the conventions these manifests follow.

Prometheus is **not** here. It runs on every node from the [NixOS image](../../nix), so it is a
node service rather than a cluster workload. Grafana's datasource therefore points at the node's
Prometheus, not at a Service — replace the placeholder host in
[grafana/configmap.yaml](./grafana/configmap.yaml) with the cluster's own before the first apply.

The node Prometheus discovers its targets from pod annotations, so a component becomes a scrape
target by carrying them on its pod template — no change to the Prometheus configuration is needed:

```yaml
annotations:
  prometheus.io/scrape: "true"
  prometheus.io/port: "9100"
  prometheus.io/path: /metrics
```

That discovery needs a `ClusterRole` and `ClusterRoleBinding` granting the node's Prometheus read
access to the API server. They are cluster-scoped and provisioned with the node, not from this
directory.
