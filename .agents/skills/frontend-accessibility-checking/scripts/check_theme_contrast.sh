#!/usr/bin/env bash
set -e

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Verifies the colour tokens keep WCAG contrast in the light and dark themes.
# Usage: .claude/skills/frontend-accessibility-checking/scripts/check_theme_contrast.sh
# oxaria only renders the light theme, so this is the check that covers the dark one.

THEMES=frontend/src/css/_themes.scss

python3 - "$THEMES" <<'PYTHON'
import re
import sys

# WCAG 1.4.3: 4.5:1 for normal-size text. Every pair below can carry normal-size text.
MIN_TEXT_CONTRAST = 4.5
# WCAG 1.4.11: 3:1 for the edge of a control and for a focus indicator.
MIN_NON_TEXT_CONTRAST = 3.0

TEXT_PAIRS = [
    ("text", "surface"),
    ("text", "surface-raised"),
    ("text", "background"),
    ("text-muted", "surface"),
    ("text-muted", "surface-raised"),
    ("text-muted", "background"),
    ("primary", "surface"),
    ("primary", "background"),
    ("danger", "surface"),
    ("success", "surface"),
    ("warning", "surface"),
    ("on-primary", "primary"),
    ("on-primary", "primary-hover"),
    ("on-danger", "danger"),
    ("on-danger", "danger-hover"),
]

# --color-border is the only edge of the inputs and of the switch track.
# --color-primary is the focus-ring outline of every focusable element.
NON_TEXT_PAIRS = [
    ("border", "surface"),
    ("border", "background"),
    ("primary", "surface"),
    ("primary", "background"),
]

source = open(sys.argv[1]).read()
blocks = {
    "light": re.search(r":root\s*\{(.*?)\}", source, re.S).group(1),
    "dark": re.search(r"\[data-theme='dark'\]\s*\{(.*?)\}", source, re.S).group(1),
}


def tokens(block):
    return dict(re.findall(r"--color-([\w-]+):\s*(#[0-9a-fA-F]{6})", block))


def luminance(hex_color):
    channels = [int(hex_color[i : i + 2], 16) / 255 for i in (1, 3, 5)]
    linear = [c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4 for c in channels]
    return 0.2126 * linear[0] + 0.7152 * linear[1] + 0.0722 * linear[2]


def contrast(first, second):
    lighter, darker = sorted((luminance(first), luminance(second)), reverse=True)
    return (lighter + 0.05) / (darker + 0.05)


failures = 0
for theme, block in blocks.items():
    colors = tokens(block)
    print(f"{theme}:")
    checks = [(pair, MIN_TEXT_CONTRAST, "text") for pair in TEXT_PAIRS]
    checks += [(pair, MIN_NON_TEXT_CONTRAST, "edge") for pair in NON_TEXT_PAIRS]
    for (foreground, background), minimum, kind in checks:
        ratio = contrast(colors[foreground], colors[background])
        label = f"{kind} --color-{foreground} on --color-{background}: {ratio:.2f}:1"
        if ratio >= minimum:
            print(f"  ✓ {label}")
        else:
            print(f"  ✗ {label}, below {minimum}:1; change the {theme} token in {sys.argv[1]}")
            failures += 1

if failures:
    print(f"{failures} pair(s) below their minimum contrast.")
    sys.exit(1)
print("Both themes keep the text and edge contrast.")
PYTHON
