# Changelog

All notable changes to `trace` are documented in this file.

## Unreleased

### Added

- Source revision, dirty-state, and build-date metadata in `trace --version` and exported databases.
- Explicit database schema-version metadata.
- Validated, version-tagged GitHub releases alongside the rolling `master-latest` prerelease.
- OpenHarmony IPC proxy-to-stub call edges for matching `SendRequest` methods.

### Changed

- Database schema v2 introduced `analysis_run.schema_version`; databases written by earlier
  versions have no such column and report themselves through the existing stale-schema errors.
- Database schema is now **v3**: `call_edges.call_site_id` is nullable for synthetic edges and
  `call_edges.caller_fn_id` records their caller independently of a source call site. Inspecting
  call data from an older database reports an actionable re-analysis message.
