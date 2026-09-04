#!/bin/sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
tmp_dir=$(mktemp -d)
trap 'rm -rf "$tmp_dir"' EXIT HUP INT TERM

valid_manifest="$tmp_dir/valid.toml"
missing_workspace_manifest="$tmp_dir/missing-workspace.toml"

printf '%s\n' \
    '[workspace]' \
    'members = []' \
    '' \
    '[workspace.package]' \
    'version = "0.1.0"' \
    'edition = "2021"' >"$valid_manifest"
printf '%s\n' '[package]' 'name = "example"' 'version = "0.1.0"' >"$missing_workspace_manifest"

expect_success() {
    name=$1
    shift
    if ! "$@"; then
        printf 'expected success: %s\n' "$name" >&2
        exit 1
    fi
}

expect_failure() {
    name=$1
    shift
    if "$@" >/dev/null 2>&1; then
        printf 'expected failure: %s\n' "$name" >&2
        exit 1
    fi
}

validator="$script_dir/check_release_tag.sh"
expect_success matching-version sh "$validator" v0.1.0 "$valid_manifest"
expect_failure missing-v-prefix sh "$validator" 0.1.0 "$valid_manifest"
expect_failure incomplete-version sh "$validator" v0.1 "$valid_manifest"
expect_failure mismatched-version sh "$validator" v0.1.1 "$valid_manifest"
expect_failure missing-workspace-version sh "$validator" v0.1.0 "$missing_workspace_manifest"

printf 'release tag validation tests passed\n'
