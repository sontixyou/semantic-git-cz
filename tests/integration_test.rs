use semantic_git_cz::{commit_types::CommitType, semver::SemverType};

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

#[test]
fn test_commit_type_description_with_emoji() {
    let commit_types = vec![
        CommitType::Feat,
        CommitType::Fix,
        CommitType::Docs,
        CommitType::Style,
        CommitType::Refactor,
        CommitType::Test,
        CommitType::Chore,
        CommitType::Ci,
        CommitType::Perf,
    ];

    let descriptions: Vec<String> = commit_types
        .iter()
        .map(|t| format!("{} {}", t.emoji(), t.description()))
        .collect();

    assert_eq!(descriptions[0], "✨ A new feature");
    assert_eq!(descriptions[1], "🐛 A bug fix");
    assert_eq!(descriptions[2], "📚 Documentation only changes");
    assert_eq!(
        descriptions[3],
        "💎 Changes that do not affect the meaning of the code"
    );
    assert_eq!(
        descriptions[4],
        "♻️ A code change that neither fixes a bug nor adds a feature"
    );
    assert_eq!(
        descriptions[5],
        "🧪 Adding missing tests or correcting existing tests"
    );
    assert_eq!(
        descriptions[6],
        "🔧 Changes to the build process or auxiliary tools"
    );
    assert_eq!(
        descriptions[7],
        "🚀 Changes to CI configuration files and scripts"
    );
    assert_eq!(
        descriptions[8],
        "⚡ A code change that improves performance"
    );
}
