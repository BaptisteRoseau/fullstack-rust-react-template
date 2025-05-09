#!/usr/bin/env bash
set -e
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
cd "$SCRIPT_DIR"

# Using a symbolic link to allow the script to be seamlessly updated without
# manual intervention.
# Using a relative path to allow the user to copy the entire repo directory
# without affecting the git hooks execution.
shopt -s extglob
for hook in "$SCRIPT_DIR"/!(*.sh); do
    hook=$(basename "$hook")
    echo "Linking $hook"
    rm -f "$(git rev-parse --show-toplevel)/.git/hooks/$hook"
    ln -s "../../scripts/git_hooks/$hook" "$(git rev-parse --show-toplevel)/.git/hooks/$hook"
done

echo "Git hooks successfully set up"
