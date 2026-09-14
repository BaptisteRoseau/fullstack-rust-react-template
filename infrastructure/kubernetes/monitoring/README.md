# Monitoring

Contains all the tools used to monitor the application:

- Grafana
- Prometheus
- Postgres Exporter

One directory per component, each with its own `kustomization.yaml`. See
[../README.md](../README.md) for the conventions these manifests follow.

Prometheus discovers its targets from pod annotations, so a component becomes a
scrape target by carrying them on its pod template — no change to the Prometheus
configuration is needed:

```yaml
annotations:
  prometheus.io/scrape: "true"
  prometheus.io/port: "9100"
  prometheus.io/path: /metrics
```

The `ClusterRole` and `ClusterRoleBinding` Prometheus needs for that discovery are
cluster-scoped: a single overlay can own them per cluster.
