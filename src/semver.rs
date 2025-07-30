use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemverType {
    Major, // 破壊的変更
    Minor, // 機能追加
    Patch, // バグ修正
}

impl fmt::Display for SemverType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SemverType::Major => write!(f, "major"),
            SemverType::Minor => write!(f, "minor"),
            SemverType::Patch => write!(f, "patch"),
        }
    }
}

impl SemverType {
    pub fn description(&self) -> &'static str {
        match self {
            SemverType::Major => "Breaking changes (incompatible API changes)",
            SemverType::Minor => "New features (backwards compatible)",
            SemverType::Patch => "Bug fixes (backwards compatible)",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semver_display() {
        assert_eq!(format!("{}", SemverType::Major), "major");
        assert_eq!(format!("{}", SemverType::Minor), "minor");
        assert_eq!(format!("{}", SemverType::Patch), "patch");
    }

    #[test]
    fn test_semver_description() {
        assert_eq!(
            SemverType::Major.description(),
            "Breaking changes (incompatible API changes)"
        );
        assert_eq!(
            SemverType::Minor.description(),
            "New features (backwards compatible)"
        );
        assert_eq!(
            SemverType::Patch.description(),
            "Bug fixes (backwards compatible)"
        );
    }
}
