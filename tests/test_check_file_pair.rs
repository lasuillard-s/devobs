mod helpers;

use std::collections::HashMap;

use assert_cmd::Command;
use insta::assert_snapshot;

use crate::helpers::{first_line, list_dir, parse_output, temp_git_dir};

#[test]
fn test_empty_directory_no_error_no_output() {
    // Arrange
    let temp_dir = temp_git_dir(None);

    // Act
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed to create command");
    let assert = cmd
        .arg("--no-colors")
        .arg("check-file-pair")
        .args(&["--from", temp_dir.path().join("src").to_str().unwrap()])
        .args(&["--to", temp_dir.path().join("tests").to_str().unwrap()])
        .assert();

    // Assert
    let result = assert.success().code(0);
    let (stdout, stderr) = parse_output(result.get_output());
    assert_snapshot!(stdout.replace(temp_dir.path().to_str().unwrap(), "<temp_dir>"));
    assert_eq!(stderr, "");
    assert_eq!(list_dir(temp_dir.path()), vec![] as Vec<String>);
}

#[test]
fn test_forward_matching() {
    // Arrange
    let temp_dir = temp_git_dir(Some(HashMap::<_, _>::from_iter(vec![
        ("src/__init__.py", None),
        ("src/main.py", None),
        ("src/utils/logger.py", None),
        ("src/utils/slack/template.py", None),
    ])));

    // Act
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed to create command");
    let assert = cmd
        .arg("--no-colors")
        .arg("check-file-pair")
        .args(&["--from", temp_dir.path().join("src").to_str().unwrap()])
        .args(&["--to", temp_dir.path().join("tests").to_str().unwrap()])
        .args(&["--include", "**/*.py"])
        .args(&["--expect", "{to}/{relative_from}/test_{filename}"])
        .assert();

    // Assert
    let result = assert.failure().code(1);
    let (stdout, stderr) = parse_output(result.get_output());
    assert_snapshot!(stdout.replace(temp_dir.path().to_str().unwrap(), "<temp_dir>"));
    assert_eq!(
        first_line(stderr),
        "Error: There are 4 missing files. Use `--create-if-not-exists` to create them."
    );
    assert_eq!(
        list_dir(temp_dir.path()),
        vec![
            "src/__init__.py",
            "src/main.py",
            "src/utils/logger.py",
            "src/utils/slack/template.py",
        ]
    );
}

#[test]
fn test_backward_matching() {
    // Arrange
    let temp_dir = temp_git_dir(Some(HashMap::<_, _>::from_iter(vec![
        ("src/__init__.py", None),
        ("src/main.py", None),
        ("tests/test_main.py", None),
        ("tests/utils/slack/test_template.py", None),
        ("tests/utils/test_logger.py", None),
    ])));

    // Act
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed to create command");
    let assert = cmd
        .arg("--no-colors")
        .arg("check-file-pair")
        .args(&["--from", temp_dir.path().join("tests").to_str().unwrap()])
        .args(&["--to", temp_dir.path().join("src").to_str().unwrap()])
        .args(&["--include", "**/*.py"])
        .args(&["--filename-regex", "^test_(?P<filename>.*)$"])
        .assert();

    // Assert
    let result = assert.failure().code(1);
    let (stdout, stderr) = parse_output(result.get_output());
    assert_snapshot!(stdout.replace(temp_dir.path().to_str().unwrap(), "<temp_dir>"));
    assert_eq!(
        first_line(stderr),
        "Error: There are 2 missing files. Use `--create-if-not-exists` to create them."
    );
    assert_eq!(
        list_dir(temp_dir.path()),
        vec![
            "src/__init__.py",
            "src/main.py",
            "tests/test_main.py",
            "tests/utils/slack/test_template.py",
            "tests/utils/test_logger.py",
        ]
    );
}

#[test]
fn test_on_fully_populated_directory() {
    // Arrange
    let temp_dir = temp_git_dir(Some(HashMap::<_, _>::from_iter(vec![
        ("src/__init__.py", None),
        ("src/apps/posts/migrations/__init__.py", None),
        ("src/apps/posts/migrations/0001_initial.py", None),
        ("src/main.py", None),
        ("src/utils/logger.py", None),
        ("src/utils/slack/template.py", None),
        ("tests/test_main.py", None),
        ("tests/utils/slack/test_template.py", None),
        ("tests/utils/test_logger.py", None),
    ])));

    // Act
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed to create command");
    let assert = cmd
        .arg("--no-colors")
        .arg("check-file-pair")
        .args(&["--from", temp_dir.path().join("tests").to_str().unwrap()])
        .args(&["--to", temp_dir.path().join("src").to_str().unwrap()])
        .args(&["--include", "**/*.py"])
        .args(&["--exclude", "**/migrations/*.py"])
        .args(&["--filename-regex", "^test_(?P<filename>.*)$"])
        .assert();

    // Assert
    let result = assert.success().code(0);
    let (stdout, stderr) = parse_output(result.get_output());
    assert_snapshot!(stdout.replace(temp_dir.path().to_str().unwrap(), "<temp_dir>"));
    assert_eq!(stderr, "");
    assert_eq!(
        list_dir(temp_dir.path()),
        vec![
            "src/__init__.py",
            "src/apps/posts/migrations/0001_initial.py",
            "src/apps/posts/migrations/__init__.py",
            "src/main.py",
            "src/utils/logger.py",
            "src/utils/slack/template.py",
            "tests/test_main.py",
            "tests/utils/slack/test_template.py",
            "tests/utils/test_logger.py",
        ]
    );
}

#[test]
fn test_create_if_not_exists() {
    // Arrange
    let temp_dir = temp_git_dir(Some(HashMap::<_, _>::from_iter(vec![
        ("src/__init__.py", None),
        ("src/main.py", None),
        ("src/utils/logger.py", None),
        ("src/utils/slack/template.py", None),
        ("tests/test_main.py", None),
    ])));

    // Act
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed to create command");
    let assert = cmd
        .arg("--no-colors")
        .arg("check-file-pair")
        .args(&["--from", temp_dir.path().join("src").to_str().unwrap()])
        .args(&["--to", temp_dir.path().join("tests").to_str().unwrap()])
        .args(&["--include", "**/*.py"])
        .args(&["--exclude", "**/_*.py"])
        .args(&["--expect", "{to}/{relative_from}/test_{filename}"])
        .args(&["--create-if-not-exists"])
        .assert();

    // Assert
    let result = assert.failure().code(1);
    let (stdout, stderr) = parse_output(result.get_output());
    assert_snapshot!(stdout.replace(temp_dir.path().to_str().unwrap(), "<temp_dir>"));
    assert_eq!(first_line(stderr), "Error: Created 2 missing files.");
    assert_eq!(
        list_dir(temp_dir.path()),
        vec![
            "src/__init__.py",
            "src/main.py",
            "src/utils/logger.py",
            "src/utils/slack/template.py",
            "tests/test_main.py",
            "tests/utils/slack/test_template.py",
            "tests/utils/test_logger.py",
        ]
    );
}

/// Test for `--create-if-not-exists` with `--dry-run` option.
///
/// If the files do not exist, they should not be created,
/// but the output should indicate they would be created.
#[test]
fn test_create_if_not_exists_dry_run() {
    // Arrange
    let temp_dir = temp_git_dir(Some(HashMap::<_, _>::from_iter(vec![
        ("src/__init__.py", None),
        ("src/main.py", None),
        ("src/utils/logger.py", None),
        ("src/utils/slack/template.py", None),
        ("tests/test_main.py", None),
    ])));

    // Act
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("Failed to create command");
    let assert = cmd
        .arg("--no-colors")
        .arg("--dry-run")
        .arg("check-file-pair")
        .args(&["--from", temp_dir.path().join("src").to_str().unwrap()])
        .args(&["--to", temp_dir.path().join("tests").to_str().unwrap()])
        .args(&["--include", "**/*.py"])
        .args(&["--exclude", "**/_*.py"])
        .args(&["--expect", "{to}/{relative_from}/test_{filename}"])
        .args(&["--create-if-not-exists"])
        .assert();

    // Assert
    let result = assert.failure().code(1);
    let (stdout, stderr) = parse_output(result.get_output());
    assert_snapshot!(stdout.replace(temp_dir.path().to_str().unwrap(), "<temp_dir>"));
    assert_eq!(first_line(stderr), "Error: Created 2 missing files.");
    assert_eq!(
        list_dir(temp_dir.path()),
        vec![
            "src/__init__.py",
            "src/main.py",
            "src/utils/logger.py",
            "src/utils/slack/template.py",
            "tests/test_main.py",
        ]
    );
}
