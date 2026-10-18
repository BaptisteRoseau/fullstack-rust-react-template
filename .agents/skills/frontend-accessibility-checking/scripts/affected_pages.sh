#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Prints the URL of every page that renders one of the given frontend files.
# Usage: .claude/skills/frontend-accessibility-checking/scripts/affected_pages.sh [--base <url>] [<file>...]
# With no file, uses the files changed since HEAD, untracked ones included.

BASE_URL=""
if [ "$1" = "--base" ]; then
    BASE_URL=${2:?usage: affected_pages.sh [--base <url>] [<file>...]}
    shift 2
fi

if [ $# -eq 0 ]; then
    mapfile -t files < <(
        git diff --name-only HEAD -- frontend/src
        git ls-files --others --exclude-standard -- frontend/src
    )
else
    files=("$@")
fi

python3 - "$BASE_URL" "${files[@]}" <<'PYTHON'
import os
import re
import sys
from collections import deque

SRC = "frontend/src"
ROUTES = f"{SRC}/router/routes.tsx"
CONSTANTS = f"{SRC}/router/constants.ts"
EXTENSIONS = ("", ".ts", ".tsx", ".scss", "/index.ts", "/index.tsx")
# These are loaded by every page without an import a graph could follow: global SCSS
# through load paths, the catalogs through Lingui, the router and layouts by main.tsx.
SHARED = ("css/", "i18n/", "router/", "layouts/", "App.tsx", "main.tsx", "Context.tsx")
# The `*` route matches any unknown path.
NOT_FOUND_PATH = "/does-not-exist"
IMPORT = re.compile(r"""(?:from\s+|import\s*\(\s*|import\s+)['"]([^'"]+)['"]""")

base_url = sys.argv[1]
changed = [os.path.normpath(path) for path in sys.argv[2:]]


def read_paths():
    paths, stack = {}, []
    for line in open(CONSTANTS):
        if match := re.match(r"\s*(\w+):\s*\{", line):
            stack.append(match.group(1))
        elif match := re.match(r"\s*(\w+):\s*'([^']*)'", line):
            paths[".".join(stack + [match.group(1)])] = match.group(2)
        elif re.match(r"\s*\}", line) and stack:
            stack.pop()
    return paths


def read_page_urls():
    paths, urls, current = read_paths(), {}, None
    for token in re.finditer(r"path:\s*PATHS\.([\w.]+)|import\('@/pages/(\w+)'\)", open(ROUTES).read()):
        if token.group(1):
            current = paths[token.group(1)]
        elif current is not None:
            url = NOT_FOUND_PATH if current == "*" else current
            urls.setdefault(token.group(2), set()).add(url)
    return urls


def resolve(importer, specifier):
    if specifier.startswith("@/"):
        base = os.path.join(SRC, specifier[2:])
    elif specifier.startswith("."):
        base = os.path.normpath(os.path.join(os.path.dirname(importer), specifier))
    else:
        return None
    for extension in EXTENSIONS:
        if os.path.isfile(base + extension):
            return base + extension
    return None


def read_importers():
    importers = {}
    for root, _, names in os.walk(SRC):
        for name in names:
            if not name.endswith((".ts", ".tsx")) or re.search(r"\.(test|stories)\.tsx?$", name):
                continue
            importer = os.path.join(root, name)
            for specifier in IMPORT.findall(open(importer).read()):
                if target := resolve(importer, specifier):
                    importers.setdefault(target, set()).add(importer)
    return importers


def page_of(path):
    match = re.match(rf"{SRC}/pages/(\w+)/", path)
    return match.group(1) if match else None


page_urls = read_page_urls()
importers = read_importers()
pages, every_page = set(), False
queue, seen = deque(path for path in changed if path.startswith(SRC + "/")), set()

while queue:
    path = queue.popleft()
    if path in seen:
        continue
    seen.add(path)
    if path[len(SRC) + 1 :].startswith(SHARED):
        every_page = True
        break
    page = page_of(path)
    if page:
        pages.add(page)
    for importer in importers.get(path, ()):
        if page is None or page_of(importer) == page:
            queue.append(importer)

urls = set()
for page, page_url_set in page_urls.items():
    if every_page or page in pages:
        urls |= page_url_set

if not urls:
    print("No page renders these files.", file=sys.stderr)
for url in sorted(urls):
    print(f"{base_url.rstrip('/')}{url}")
PYTHON
