use semantic_git_cz::{commit_types::CommitType, git, prompts, semver::SemverType};
use semantic_git_cz::{AppError, Result};
use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const HELP_MESSAGE: &str = r#"Semantic Git-CZ - A semantic commit message tool

USAGE:
    semantic-git-cz [OPTIONS]

OPTIONS:
    -h, --help      Show this help message
    -v, --version   Show version information

DESCRIPTION:
    Interactive tool for creating semantic commit messages with version prefixes.
    Format: {semver}-{type}: {emoji} {message}
    Example: patch-feat: ✨ add user authentication
"#;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "-h" | "--help" => {
                println!("{HELP_MESSAGE}");
                return Ok(());
            }
            "-v" | "--version" => {
                println!("semantic-git-cz {VERSION}");
                return Ok(());
            }
            _ => {
                eprintln!("Unknown option: {}", args[1]);
                eprintln!("Use -h or --help for usage information");
                std::process::exit(1);
            }
        }
    }

    // Check if we're in a git repository
    if !git::is_git_repository()? {
        return Err(AppError::Git("Not in a git repository".to_string()));
    }

    // Check if there are staged changes
    if !git::has_staged_changes()? {
        return Err(AppError::Git("No staged changes to commit".to_string()));
    }

    println!("Semantic Git-CZ - Create semantic commit messages\n");

    // Show staged files
    let staged_files = git::get_staged_files()?;
    if !staged_files.is_empty() {
        println!("Staged files:");
        for file in &staged_files {
            println!("  - {file}");
        }
        println!();
    }

    // Select semantic version
    let semver_types = vec![SemverType::Major, SemverType::Minor, SemverType::Patch];
    let semver_descriptions: Vec<&str> = semver_types.iter().map(|t| t.description()).collect();

    let semver_index = prompts::select_from_list(
        "Select the semantic version type:",
        &semver_types,
        Some(&semver_descriptions),
    )?;
    let selected_semver = semver_types[semver_index];

    // Select commit type
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
    let commit_descriptions: Vec<String> = commit_types
        .iter()
        .map(|t| format!("{} {}", t.emoji(), t.description()))
        .collect();
    let commit_descriptions_refs: Vec<&str> =
        commit_descriptions.iter().map(|s| s.as_str()).collect();

    let commit_index = prompts::select_from_list(
        "\nSelect the commit type:",
        &commit_types,
        Some(&commit_descriptions_refs),
    )?;
    let selected_commit = commit_types[commit_index];

    // Get commit message (subject)
    println!();
    let subject = prompts::prompt("Enter commit message (subject): ")?;

    if subject.is_empty() {
        return Err(AppError::InvalidInput(
            "Commit subject cannot be empty".to_string(),
        ));
    }

    // Get commit body (optional)
    println!();
    let body = prompts::prompt_multiline("Enter commit body (optional):")?;

    // Format the commit message
    let formatted_message = if body.is_empty() {
        format!(
            "{}-{}: {} {}",
            selected_semver,
            selected_commit,
            selected_commit.emoji(),
            subject
        )
    } else {
        format!(
            "{}-{}: {} {}\n\n{}",
            selected_semver,
            selected_commit,
            selected_commit.emoji(),
            subject,
            body
        )
    };

    // Show preview
    println!("\nCommit message preview:");
    println!("  {formatted_message}");

    // Confirm
    let confirm = prompts::prompt("\nConfirm commit? (y/n): ")?;

    if confirm.to_lowercase() == "y" || confirm.to_lowercase() == "yes" {
        git::commit(&formatted_message)?;
        println!("\n✅ Commit created successfully!");
    } else {
        println!("\n❌ Commit cancelled");
    }

    Ok(())
}
