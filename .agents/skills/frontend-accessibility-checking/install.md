# Installing oxaria

Install `oxaria` with
Cargo if available; otherwise download the latest prebuilt release binary on GitHub and place
it under `~/.local/bin`.

Source and releases: <https://github.com/BaptisteRoseau/oxaria>

## Check whether it is already installed

```bash
oxaria --version
```

If this prints a version, you are done.

## Option A — Cargo (preferred, if available)

If `cargo` is installed:

```bash
cargo install oxaria
```

Ensure `~/.cargo/bin` is on your `PATH`, then verify:

```bash
oxaria --version
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
   - `oxaria-linux-amd64.tar.gz`
   - `oxaria-linux-arm64.tar.gz`
   - `oxaria-macos-arm64.zip`
   - `oxaria-windows-amd64.zip`

   Example, Linux x86_64 (using the GitHub CLI to grab the latest release):

   ```bash
   mkdir -p ~/.local/bin /tmp/o2s && cd /tmp/o2s
   gh release download --repo BaptisteRoseau/oxaria \
     --pattern 'oxaria-linux-amd64.tar.gz'
   tar -xzf oxaria-linux-amd64.tar.gz
   install -m 700 oxaria ~/.local/bin/oxaria
   ```

   Without `gh`, download the same asset from the latest release page with `curl -L`:

   ```bash
   mkdir -p ~/.local/bin /tmp/o2s && cd /tmp/o2s
   curl -sSL -o oxaria.tar.gz \
     https://github.com/BaptisteRoseau/oxaria/releases/latest/download/oxaria-linux-amd64.tar.gz
   tar -xzf oxaria.tar.gz
   install -m 700 oxaria ~/.local/bin/oxaria
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
   oxaria --version
   ```
