//! Pure helpers shared by `build.rs` and its unit tests. Everything that
//! touches the environment, the clock or `git` lives in `build.rs`, so the
//! test binary that includes this file compiles none of it.

use std::path::{Path, PathBuf};

pub struct BuildMetadata {
    pub commit: String,
    pub dirty: bool,
    pub date: String,
}

pub fn format_version(package_version: &str, metadata: &BuildMetadata) -> String {
    let dirty = if metadata.dirty && metadata.commit != "unknown" {
        "-dirty"
    } else {
        ""
    };
    format!(
        "{package_version} ({}{dirty} {})",
        metadata.commit, metadata.date
    )
}

pub fn git_dir(workspace_root: &Path) -> Option<PathBuf> {
    let marker = workspace_root.join(".git");
    if marker.is_dir() {
        return Some(marker);
    }
    let contents = std::fs::read_to_string(marker).ok()?;
    let path = contents.trim().strip_prefix("gitdir: ")?;
    let path = Path::new(path);
    Some(if path.is_absolute() {
        path.to_owned()
    } else {
        workspace_root.join(path)
    })
}

pub fn parse_dirty(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes"
    )
}

// Howard Hinnant's civil calendar conversion, with 1970-01-01 as day zero.
pub fn civil_from_days(days_since_epoch: i64) -> (i64, i64, i64) {
    let days = days_since_epoch + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    (year, month, day)
}

#[cfg(test)]
mod tests {
    use super::{civil_from_days, format_version, git_dir, parse_dirty, BuildMetadata};
    use std::path::Path;

    #[test]
    fn formats_clean_build_identity() {
        let metadata = BuildMetadata {
            commit: "63e5ae3".into(),
            dirty: false,
            date: "2026-09-02".into(),
        };

        assert_eq!(
            format_version("0.1.0", &metadata),
            "0.1.0 (63e5ae3 2026-09-02)"
        );
    }

    #[test]
    fn marks_dirty_build_identity() {
        let metadata = BuildMetadata {
            commit: "63e5ae3".into(),
            dirty: true,
            date: "2026-09-02".into(),
        };

        assert_eq!(
            format_version("0.1.0", &metadata),
            "0.1.0 (63e5ae3-dirty 2026-09-02)"
        );
    }

    #[test]
    fn formats_unknown_commit_without_dirty_suffix() {
        let metadata = BuildMetadata {
            commit: "unknown".into(),
            dirty: true,
            date: "2026-09-02".into(),
        };

        assert_eq!(
            format_version("0.1.0", &metadata),
            "0.1.0 (unknown 2026-09-02)"
        );
    }

    #[test]
    fn parses_only_explicit_truthy_dirty_overrides() {
        for value in ["1", "true", "TRUE", "yes"] {
            assert!(parse_dirty(value), "{value}");
        }
        for value in ["0", "false", "no", "garbage", ""] {
            assert!(!parse_dirty(value), "{value}");
        }
    }

    #[test]
    fn converts_epoch_days_to_utc_dates() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(20_333), (2025, 9, 2));
    }

    #[test]
    fn missing_git_marker_has_no_git_directory() {
        assert_eq!(git_dir(Path::new("definitely-missing")), None);
    }
}
