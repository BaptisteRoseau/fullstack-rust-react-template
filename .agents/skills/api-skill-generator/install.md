# Installing openapi2skill

Install `openapi2skill` with
Cargo if available; otherwise download the latest prebuilt release binary on GitHub and place
it under `~/.local/bin`.

Source and releases: https://github.com/BaptisteRoseau/openapi2skill

## Check whether it is already installed

```bash
openapi2skill --version
```

If this prints a version, you are done.

## Option A — Cargo (preferred, if available)

If `cargo` is installed:

```bash
cargo install openapi2skill
```

Ensure `~/.cargo/bin` is on your `PATH`, then verify:

```bash
openapi2skill --version
```

## Option B — Download the latest release binary

Use this when `cargo` is not available.

1. Make sure `~/.local/bin` exists (create it if not):

   ```bash
   mkdir -p ~/.local/bin
   ```

2. Download and extract the asset matching your OS/architecture. Determine your
   architecture with `uname -m` (`x86_64` → `amd64`, `aarch64`/`arm64` → `arm64`).

   The release assets are named:
   - `openapi2skill-linux-amd64.tar.gz`
   - `openapi2skill-linux-arm64.tar.gz`
   - `openapi2skill-macos-arm64.zip`
   - `openapi2skill-windows-amd64.zip`

   Example, Linux x86_64 (using the GitHub CLI to grab the latest release):

   ```bash
   mkdir -p ~/.local/bin /tmp/o2s && cd /tmp/o2s
   gh release download --repo BaptisteRoseau/openapi2skill \
     --pattern 'openapi2skill-linux-amd64.tar.gz'
   tar -xzf openapi2skill-linux-amd64.tar.gz
   install -m 700 openapi2skill ~/.local/bin/openapi2skill
   ```

   Without `gh`, download the same asset from the latest release page with `curl -L`:

   ```bash
   mkdir -p ~/.local/bin /tmp/o2s && cd /tmp/o2s
   curl -sSL -o openapi2skill.tar.gz \
     https://github.com/BaptisteRoseau/openapi2skill/releases/latest/download/openapi2skill-linux-amd64.tar.gz
   tar -xzf openapi2skill.tar.gz
   install -m 700 openapi2skill ~/.local/bin/openapi2skill
   ```

   For macOS/Windows, download the corresponding `.zip` and extract with `unzip`
   instead of `tar`.

3. Ensure `~/.local/bin` is on your `PATH`. If it is not, add it to your shell
   profile:

   ```bash
   export PATH="$HOME/.local/bin:$PATH"
   ```

4. Verify:

   ```bash
   openapi2skill --version
   ```
