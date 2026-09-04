# Changelog

All notable changes to `trace` are documented in this file.

## Unreleased

### Added

- Source revision, dirty-state, and build-date metadata in `trace --version` and exported databases.
- Explicit database schema-version metadata.
- Validated, version-tagged GitHub releases alongside the rolling `master-latest` prerelease.

### Changed

- Database schema is now **v2**: `analysis_run` carries a `schema_version` column. Databases
  written by earlier versions have no such column and report themselves through the existing
  stale-schema errors.
