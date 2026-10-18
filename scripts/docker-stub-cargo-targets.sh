#!/bin/sh
# Dependency-fetch scaffolding only; never compile these empty sources.
# Read the manifest's target tables, not a second list of target paths.
set -eu

manifest=${1:?usage: docker-stub-cargo-targets.sh /absolute/path/to/Cargo.toml}
root=$(dirname "$manifest")

# Cargo.toml uses one-line target headers and unescaped double-quoted name/path
# values. Reject other value forms rather than silently omitting a target.
paths=$(awk '
function emit() {
    if (!kind) return
    if (!name) {
        print "target table lacks name" > "/dev/stderr"
        failed = 1
        exit 1
    }
    if (!path) {
        if (kind == "bin") path = "src/bin/" name ".rs"
        else if (kind == "test") path = "tests/" name ".rs"
        else if (kind == "example") path = "examples/" name ".rs"
        else path = "benches/" name ".rs"
    }
    print path
}
/^[[:space:]]*\[/ {
    emit()
    kind = name = path = ""
    header = $0
    sub(/[[:space:]]*#.*/, "", header)
    gsub(/[[:space:]]/, "", header)
    if (header ~ /^\[\[(bin|test|example|bench)\]\]$/) {
        gsub(/\[|\]/, "", header)
        kind = header
    }
    next
}
kind && /^[[:space:]]*(name|path)[[:space:]]*=/ {
    key = $0
    sub(/=.*/, "", key)
    gsub(/[[:space:]]/, "", key)
    value = $0
    sub(/^[^=]*=[[:space:]]*/, "", value)
    if (value !~ /^"[^"\\]+"[[:space:]]*(#.*)?$/) {
        print "unsupported target " key " at line " NR > "/dev/stderr"
        failed = 1
        exit 1
    }
    sub(/^"/, "", value)
    sub(/".*/, "", value)
    if (key == "name") name = value
    else path = value
}
END { if (!failed) emit() }
' "$manifest")

printf '%s\n' "$paths" | while IFS= read -r path; do
    [ -n "$path" ] || continue
    mkdir -p "$(dirname "$root/$path")"
    # Preserve any real source already copied for a workspace/dependency.
    if [ ! -e "$root/$path" ]; then
        : > "$root/$path"
    fi
    printf '%s\n' "$path"
done
