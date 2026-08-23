# Storage

The blob and file interface. See [crates/README.md](../README.md) for the service-crate
shape this crate follows.

One backend implements the [`Storage`](src/storage.rs) trait: `backends::s3::S3`, an
S3-compatible client.

Every `save` goes through [compressor](../compressor) first, driven by the caller-supplied
`CompressionParameters`: images can be resized and converted format, and any blob can be
gzip-compressed. `Storage::save` takes the parameters explicitly, so the caller decides
the trade-off per file rather than the crate deciding for everyone. `load` reverses the
gzip step on its own: the backend stores the compression as the object's content-type and
reads it back from there, so a caller never has to remember what it saved with.

## Skills

- [backend-feature-gating](../../.claude/skills/backend-feature-gating/SKILL.md)
- [backend-trait-test](../../.claude/skills/backend-trait-test/SKILL.md)
