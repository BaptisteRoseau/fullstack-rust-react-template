# Monitoring

This crate's purpose is to provide common helper for Prometheus metrics.

For simple metrics, prefer using the [`metrics`](https://docs.rs/crate/metrics) crate macros.

For metrics that are copied in at least two places, prefer writing a helper here, see for example the OnConnection helper.
