#!/usr/bin/env bash
set -e
shopt -s nullglob

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT"

# Verifies skills follow the ai-skill-redaction guidelines.
# Usage: .claude/skills/ai-skill-redaction/scripts/check_skill.sh [<skill-name>...]
# With no argument, checks every hand-written skill.

SKILLS_DIR=.agents/skills
KNOWN_PREFIXES="frontend backend infrastructure doc ai api"
# The Agent Skills specification limits: https://agentskills.io/specification
MAX_NAME_LENGTH=64
MAX_DESCRIPTION_LENGTH=1024
MAX_SKILL_LINES=500

failures=0

pass() { echo "  ✓ $1"; }
fail() {
    echo "  ✗ $1"
    failures=$((failures + 1))
}
warn() { echo "  ! $1"; }

frontmatter_field() {
    sed -n '2,/^---$/p' "$1" | grep -m1 "^$2:" | sed "s/^$2: *//"
}

headings_outside_code() {
    awk -v level="$2" '
        /^[[:space:]]*(```|~~~)/ {
            match($0, /(```+|~~~+)/)
            marker = substr($0, RSTART, RLENGTH)
            if (fence == "") { fence = marker; next }
            if (index(marker, fence) == 1 && $0 ~ /^[[:space:]]*(```+|~~~+)[[:space:]]*$/) { fence = ""; next }
        }
        fence == "" && index($0, level " ") == 1 { print }
    ' "$1"
}

check_skill() {
    local name=$1
    local dir="$SKILLS_DIR/$name"
    local skill="$dir/SKILL.md"

    echo "$name:"
    if [ ! -f "$skill" ]; then
        fail "$skill does not exist"
        return
    fi

    if [ "$(head -1 "$skill")" != "---" ]; then
        fail "SKILL.md does not start with a '---' frontmatter block"
        return
    fi

    local field_name description
    field_name=$(frontmatter_field "$skill" name)
    description=$(frontmatter_field "$skill" description)

    if [ "$field_name" = "$name" ]; then
        pass "name matches the directory"
    else
        fail "name is '$field_name', expected the directory name '$name'"
    fi

    if [[ "$name" =~ ^[a-z0-9]+(-[a-z0-9]+)*$ ]] && [ "${#name}" -le "$MAX_NAME_LENGTH" ]; then
        pass "name uses lowercase letters, digits and single hyphens"
    else
        fail "name must match ^[a-z0-9]+(-[a-z0-9]+)*\$ and be at most $MAX_NAME_LENGTH characters"
    fi

    local prefix=${name%%-*}
    if [[ " $KNOWN_PREFIXES " == *" $prefix "* ]]; then
        pass "prefix '$prefix-' is a known scope"
    else
        warn "prefix '$prefix-' is not one of: $KNOWN_PREFIXES (fine for a new area, add it to guidelines.md)"
    fi

    if [ -z "$description" ]; then
        fail "description is missing or not on a single line"
    elif [ "${#description}" -gt "$MAX_DESCRIPTION_LENGTH" ]; then
        fail "description is ${#description} characters, the limit is $MAX_DESCRIPTION_LENGTH"
    elif [[ "$description" =~ ^Use\ (when|after|before)\  ]]; then
        pass "description starts with 'Use when/after/before'"
    else
        fail "description must start with 'Use when', 'Use after' or 'Use before'"
    fi

    local line_count
    line_count=$(wc -l < "$skill")
    if [ "$line_count" -le "$MAX_SKILL_LINES" ]; then
        pass "SKILL.md is $line_count lines"
    else
        fail "SKILL.md is $line_count lines, the limit is $MAX_SKILL_LINES: move rare branches to a <topic>.md"
    fi

    if [ "$(headings_outside_code "$skill" '#' | wc -l)" -eq 1 ]; then
        pass "one '# ' title"
    else
        fail "SKILL.md must have exactly one '# ' title"
    fi

    if [ "$(headings_outside_code "$skill" '##' | tail -1)" = "## Checklist" ]; then
        pass "'## Checklist' is the last section"
    else
        fail "the last '## ' section must be '## Checklist'"
    fi

    local referenced
    for referenced in $(grep -oE 'Skill\([a-z0-9-]+\)' "$skill" | sed 's/Skill(\(.*\))/\1/' | sort -u); do
        [ -f "$SKILLS_DIR/$referenced/SKILL.md" ] || fail "Skill($referenced) does not exist"
    done

    local reference_file
    for reference_file in "$dir"/*.md; do
        [ "$reference_file" = "$skill" ] && continue
        local base=${reference_file##*/}
        if grep -qE "\]\((\./)?$base(#[^)]*)?\)" "$skill"; then
            pass "$base is linked from SKILL.md"
        else
            fail "$base is never linked from SKILL.md, so no agent will read it"
        fi
    done

    local script
    for script in "$dir"/scripts/*; do
        [ -f "$script" ] || continue
        local relative=${script#"$dir"/}
        if [ -x "$script" ] && [ "$(head -1 "$script")" = "#!/usr/bin/env bash" ] &&
            grep -q '^set -e' "$script"; then
            pass "$relative is executable and starts with the scripts/README.md header"
        else
            fail "$relative must be executable and start with '#!/usr/bin/env bash' and 'set -e'"
        fi
    done

    if git ls-files -- '*README.md' | xargs grep -lq "skills/$name/SKILL.md" 2>/dev/null; then
        pass "linked from a README"
    else
        warn "no README links to it: add it to the '## Skills' section of the directories it covers"
    fi
}

if [ "$#" -gt 0 ]; then
    names=("$@")
else
    names=()
    for dir in "$SKILLS_DIR"/*/; do
        name=$(basename "$dir")
        # openapi2skill output (an endpoints/ tree) is generated, not hand-written.
        [ -d "$dir/endpoints" ] && continue
        names+=("$name")
    done
fi

for name in "${names[@]}"; do
    check_skill "$name"
done

if [ "$failures" -ne 0 ]; then
    echo "$failures skill problem(s) found."
    exit 1
fi

echo "Skills follow the guidelines."
