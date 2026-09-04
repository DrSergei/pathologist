# Releasing trace

All workspace crates share the version in `[workspace.package]` in the root
`Cargo.toml`. A release tag must be exactly that version prefixed by `v`; for
example, workspace version `0.2.0` is released as `v0.2.0`.

## Release checklist

1. Update `[workspace.package].version` in `Cargo.toml`.
2. Move the relevant entries from `CHANGELOG.md`'s Unreleased section into a
   `## X.Y.Z - YYYY-MM-DD` section.
3. Verify the release tree:

   ```bash
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   sh scripts/check_release_tag.sh vX.Y.Z
   ```

4. Commit the version and changelog changes.
5. Create the matching annotated tag: `git tag -a vX.Y.Z -m "trace vX.Y.Z"`.
6. Push the release commit, then push the tag: `git push origin master` followed
   by `git push origin vX.Y.Z`.

CI rejects a tag that differs from the workspace version. A valid tag builds
the same Linux x64, Linux arm64, and Windows x64 archives as `master-latest`,
then creates a normal GitHub release with generated notes. Pushes to `master`
continue to replace the rolling `master-latest` prerelease.

## Build provenance

`trace --version` reports `VERSION (COMMIT DATE)`, with `-dirty` appended to the
commit when the build came from a modified checkout. CI sets
`TRACE_BUILD_GIT_SHA`, `TRACE_BUILD_GIT_DIRTY`, and `TRACE_BUILD_DATE`
explicitly, so released binaries always carry accurate provenance.

For local builds the flag is advisory. Cargo re-runs the build script only when
`.git/HEAD`, `.git/index`, or a file under `crates/trace-cli/` changes, so
editing another crate can leave a previously clean `-dirty` state in place. Set
the environment overrides, or touch a `crates/trace-cli/` file, when a local
build's reported provenance has to be exact.

Publishing the workspace crates to crates.io is not part of this process.
