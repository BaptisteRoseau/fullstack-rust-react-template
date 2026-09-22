# Dev

Packages the development shell needs and that nixpkgs does not provide.

The shell itself is declared in the repository's root [flake.nix](../../../flake.nix). Enter it
with `nix develop`, which puts the whole toolchain — Rust, Bun, the linters, the infrastructure
tools — on the `PATH` without installing anything system-wide.

## Conventions

- **One file per package**, named after it, written as a function of its dependencies so that
  `pkgs.callPackage` can fill them in.
- **A package committed here is one nixpkgs does not ship.** Check with `nix search nixpkgs <name>`
  before adding a file: everything already packaged belongs in the shell's list, not here.
- **Both hashes are pinned.** `src.hash` is the checksum of the downloaded archive; the dependency
  set is pinned by a lock file committed beside the package rather than by a `cargoHash`, so a
  build never has to be run twice to discover a hash.

## sql-gen

[sql-gen.nix](./sql-gen.nix) builds the CLI generating Rust models from the database schema, used
by [build_database_rust_models.sh](../../../scripts/build_database_rust_models.sh).

`sql-gen-Cargo.lock` is the lock file shipped inside the published crate, copied verbatim. Bumping
the version means three changes together:

```sh
version=<new version>
nix store prefetch-file --refresh \
    "https://static.crates.io/crates/sql-gen/sql-gen-$version.crate"
curl -sL "https://static.crates.io/crates/sql-gen/sql-gen-$version.crate" | tar xz
cp "sql-gen-$version/Cargo.lock" infrastructure/nix/dev/sql-gen-Cargo.lock
```

then `version` and `src.hash` in `sql-gen.nix`.
