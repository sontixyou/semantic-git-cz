use semantic_git_cz::{semver::SemverType, commit_types::CommitType};

#[test]
fn test_semver_display() {
    assert_eq!(format!("{}", SemverType::Major), "major");
    assert_eq!(format!("{}", SemverType::Minor), "minor");
    assert_eq!(format!("{}", SemverType::Patch), "patch");
}

#[test]
fn test_commit_type_display() {
    assert_eq!(format!("{}", CommitType::Feat), "feat");
    assert_eq!(format!("{}", CommitType::Fix), "fix");
    assert_eq!(format!("{}", CommitType::Docs), "docs");
}

#[test]
fn test_commit_type_emoji() {
    assert_eq!(CommitType::Feat.emoji(), "✨");
    assert_eq!(CommitType::Fix.emoji(), "🐛");
    assert_eq!(CommitType::Docs.emoji(), "📚");
}