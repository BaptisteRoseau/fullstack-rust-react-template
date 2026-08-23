# Logging

Sets up the global `tracing` subscriber. One function, no state, no trait: every crate
that logs depends on `tracing` directly, only the binary calls into this crate.

## Public surface

- [`init_logger(debug: bool, json: bool)`](src/lib.rs) — call once, at startup.
    - `json = true` emits newline-delimited JSON (for log aggregators); otherwise a
      compact human-readable format.
    - The level comes from `RUST_LOG` when set, otherwise `DEBUG` if `debug` else `INFO`.
    - Every log line carries the fields of its active `tracing` span, so the per-request
      span the [api](../api) crate opens (carrying `request_id`) propagates that id onto
      every line logged while handling the request, including ones from lower layers.

## Directory

```txt
logging/
└── src/
    └── lib.rs   # init_logger, the whole crate
```
