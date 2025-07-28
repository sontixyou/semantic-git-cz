use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommitType {
    Feat,     // 新機能
    Fix,      // バグ修正
    Docs,     // ドキュメント
    Style,    // コードスタイル
    Refactor, // リファクタリング
    Test,     // テスト
    Chore,    // メンテナンス
    Ci,       // CI/CD
    Perf,     // パフォーマンス改善
}

impl fmt::Display for CommitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommitType::Feat => write!(f, "feat"),
            CommitType::Fix => write!(f, "fix"),
            CommitType::Docs => write!(f, "docs"),
            CommitType::Style => write!(f, "style"),
            CommitType::Refactor => write!(f, "refactor"),
            CommitType::Test => write!(f, "test"),
            CommitType::Chore => write!(f, "chore"),
            CommitType::Ci => write!(f, "ci"),
            CommitType::Perf => write!(f, "perf"),
        }
    }
}

impl CommitType {
    pub fn emoji(&self) -> &'static str {
        match self {
            CommitType::Feat => "✨",
            CommitType::Fix => "🐛",
            CommitType::Docs => "📚",
            CommitType::Style => "💎",
            CommitType::Refactor => "♻️",
            CommitType::Test => "🧪",
            CommitType::Chore => "🔧",
            CommitType::Ci => "🚀",
            CommitType::Perf => "⚡",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            CommitType::Feat => "A new feature",
            CommitType::Fix => "A bug fix",
            CommitType::Docs => "Documentation only changes",
            CommitType::Style => "Changes that do not affect the meaning of the code",
            CommitType::Refactor => "A code change that neither fixes a bug nor adds a feature",
            CommitType::Test => "Adding missing tests or correcting existing tests",
            CommitType::Chore => "Changes to the build process or auxiliary tools",
            CommitType::Ci => "Changes to CI configuration files and scripts",
            CommitType::Perf => "A code change that improves performance",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit_type_display() {
        assert_eq!(format!("{}", CommitType::Feat), "feat");
        assert_eq!(format!("{}", CommitType::Fix), "fix");
        assert_eq!(format!("{}", CommitType::Docs), "docs");
        assert_eq!(format!("{}", CommitType::Style), "style");
        assert_eq!(format!("{}", CommitType::Refactor), "refactor");
        assert_eq!(format!("{}", CommitType::Test), "test");
        assert_eq!(format!("{}", CommitType::Chore), "chore");
        assert_eq!(format!("{}", CommitType::Ci), "ci");
        assert_eq!(format!("{}", CommitType::Perf), "perf");
    }

    #[test]
    fn test_commit_type_emoji() {
        assert_eq!(CommitType::Feat.emoji(), "✨");
        assert_eq!(CommitType::Fix.emoji(), "🐛");
        assert_eq!(CommitType::Docs.emoji(), "📚");
        assert_eq!(CommitType::Style.emoji(), "💎");
        assert_eq!(CommitType::Refactor.emoji(), "♻️");
        assert_eq!(CommitType::Test.emoji(), "🧪");
        assert_eq!(CommitType::Chore.emoji(), "🔧");
        assert_eq!(CommitType::Ci.emoji(), "🚀");
        assert_eq!(CommitType::Perf.emoji(), "⚡");
    }

    #[test]
    fn test_commit_type_description() {
        assert_eq!(CommitType::Feat.description(), "A new feature");
        assert_eq!(CommitType::Fix.description(), "A bug fix");
        assert_eq!(CommitType::Docs.description(), "Documentation only changes");
        assert_eq!(CommitType::Style.description(), "Changes that do not affect the meaning of the code");
        assert_eq!(CommitType::Refactor.description(), "A code change that neither fixes a bug nor adds a feature");
        assert_eq!(CommitType::Test.description(), "Adding missing tests or correcting existing tests");
        assert_eq!(CommitType::Chore.description(), "Changes to the build process or auxiliary tools");
        assert_eq!(CommitType::Ci.description(), "Changes to CI configuration files and scripts");
        assert_eq!(CommitType::Perf.description(), "A code change that improves performance");
    }

    #[test]
    fn test_commit_type_equality() {
        assert_eq!(CommitType::Feat, CommitType::Feat);
        assert_ne!(CommitType::Feat, CommitType::Fix);
        assert_ne!(CommitType::Docs, CommitType::Style);
    }

    #[test]
    fn test_commit_type_clone() {
        let original = CommitType::Refactor;
        let cloned = original;
        assert_eq!(original, cloned);
        assert_eq!(original.emoji(), cloned.emoji());
    }

    #[test]
    fn test_commit_type_debug() {
        assert_eq!(format!("{:?}", CommitType::Feat), "Feat");
        assert_eq!(format!("{:?}", CommitType::Fix), "Fix");
        assert_eq!(format!("{:?}", CommitType::Docs), "Docs");
    }

    #[test]
    fn test_all_commit_types_have_unique_emojis() {
        let types = [
            CommitType::Feat, CommitType::Fix, CommitType::Docs,
            CommitType::Style, CommitType::Refactor, CommitType::Test,
            CommitType::Chore, CommitType::Ci, CommitType::Perf,
        ];
        
        let mut emojis = Vec::new();
        for commit_type in &types {
            let emoji = commit_type.emoji();
            assert!(!emojis.contains(&emoji), "Duplicate emoji found: {}", emoji);
            emojis.push(emoji);
        }
        assert_eq!(emojis.len(), 9);
    }
}