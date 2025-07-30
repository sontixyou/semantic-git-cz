use crate::{AppError, Result};
use std::process::Command;

pub fn is_git_repository() -> Result<bool> {
    let output = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()?;

    Ok(output.status.success())
}

pub fn has_staged_changes() -> Result<bool> {
    let output = Command::new("git")
        .args(["diff", "--cached", "--quiet"])
        .output()?;

    Ok(!output.status.success())
}

pub fn commit(message: &str) -> Result<()> {
    let output = Command::new("git")
        .args(["commit", "-m", message])
        .output()?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::Git(error_msg.to_string()));
    }

    Ok(())
}

pub fn get_staged_files() -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .output()?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::Git(error_msg.to_string()));
    }

    let files = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.is_empty())
        .map(|s| s.to_string())
        .collect();

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;


    #[test]
    fn test_git_error_handling() {
        let output = Command::new("git")
            .args(&["this-command-does-not-exist"])
            .output();

        if let Ok(output) = output {
            if !output.status.success() {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                let app_error = AppError::Git(error_msg.to_string());
                assert!(format!("{}", app_error).contains("Git error"));
            }
        }
    }

    #[test]
    fn test_staged_files_parsing() {
        let mock_output = "file1.rs\nfile2.rs\n\nfile3.rs\n";
        let files: Vec<String> = mock_output
            .lines()
            .filter(|line| !line.is_empty())
            .map(|s| s.to_string())
            .collect();

        assert_eq!(files.len(), 3);
        assert_eq!(files[0], "file1.rs");
        assert_eq!(files[1], "file2.rs");
        assert_eq!(files[2], "file3.rs");
    }
}
