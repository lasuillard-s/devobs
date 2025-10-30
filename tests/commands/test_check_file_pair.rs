use anyhow::Result;
use insta::assert_snapshot;
use sugars::hmap;

use crate::{helpers::{first_line, get_cmd, get_temp_dir, list_dir, normalize_console_output,
                      parse_output},
            to_str};

/// Test that an empty directory does not produce an error or output.
#[test]
fn test_empty_directory_no_error_no_output() -> Result<()> {
    // Arrange
    let temp_dir = get_temp_dir(hmap! {});
    let dir_path = temp_dir.path();

    // Act
    let mut cmd = get_cmd();
    let assert = cmd
        .arg("check-file-pair")
        .args(&["--from", to_str!(dir_path.join("src"))])
        .args(&["--to", to_str!(dir_path.join("tests"))])
        .assert();

    // Assert
    let result = assert.success().code(0);
    let (stdout, stderr) = parse_output(result.get_output());
    assert_snapshot!(normalize_console_output(
        stdout,
        hmap! {
            to_str!(dir_path) => "<temp_dir>"
        }
    ));
    assert_eq!(stderr, "");
    assert_eq!(list_dir(dir_path), &[] as &[&str]);
    Ok(())
}

/// Test that files are correctly matched in forward direction
#[test]
fn test_forward_matching() -> Result<()> {
    // Arrange
    let temp_dir = get_temp_dir(hmap! {
        "src/__init__.py" => "",
        "src/main.py" => "",
        "src/utils/logger.py" => "",
        "src/utils/slack/template.py" => "",
    });
    let dir_path = temp_dir.path();

    // Act
    let mut cmd = get_cmd();
    let assert = cmd
        .arg("check-file-pair")
        .args(&["--from", to_str!(dir_path.join("src"))])
        .args(&["--to", to_str!(dir_path.join("tests"))])
        .args(&["--include", "**/*.py"])
        .args(&["--expect", "{to}/{relative_from}/test_{filename}"])
        .assert();

    // Assert
    let result = assert.failure().code(1);
    let (stdout, stderr) = parse_output(result.get_output());
    assert_snapshot!(normalize_console_output(
        stdout,
        hmap! {
            to_str!(dir_path) => "<temp_dir>"
        }
    ));
    assert_eq!(
        first_line(stderr),
        "Error: There are 4 missing files. Use `--create-if-not-exists` to create them."
    );
    assert_eq!(
        list_dir(dir_path),
        &[
            "src/__init__.py",
            "src/main.py",
            "src/utils/logger.py",
            "src/utils/slack/template.py",
        ]
    );
    Ok(())
}

/// Test that files are correctly matched in backward direction
#[test]
fn test_backward_matching() -> Result<()> {
    // Arrange
    let temp_dir = get_temp_dir(hmap! {
        "src/__init__.py" => "",
        "src/main.py" => "",
        "tests/test_main.py" => "",
        "tests/conftest.py" => "",
        "tests/_helpers.py" => "",
        "tests/utils/slack/test_template.py" => "",
        "tests/utils/test_logger.py" => "",
    });
    let dir_path = temp_dir.path();

    // Act
    let mut cmd = get_cmd();
    let assert = cmd
        .arg("check-file-pair")
        .args(&["--from", to_str!(dir_path.join("tests"))])
        .args(&["--to", to_str!(dir_path.join("src"))])
        .args(&["--include", "**/*.py"])
        .args(&["--filename-regex", "^test_(?P<filename>.*)$"])
        .assert();

    // Assert
    let result = assert.failure().code(1);
    let (stdout, stderr) = parse_output(result.get_output());
    assert_snapshot!(normalize_console_output(
        stdout,
        hmap! {
            to_str!(dir_path) => "<temp_dir>"
        }
    ));
    assert_eq!(
        first_line(stderr),
        "Error: There are 4 missing files. Use `--create-if-not-exists` to create them."
    );
    assert_eq!(
        list_dir(dir_path),
        &[
            "src/__init__.py",
            "src/main.py",
            "tests/_helpers.py",
            "tests/conftest.py",
            "tests/test_main.py",
            "tests/utils/slack/test_template.py",
            "tests/utils/test_logger.py"
        ]
    );
    Ok(())
}

/// Test on a fully populated directory structure
#[test]
fn test_on_fully_populated_directory() -> Result<()> {
    // Arrange
    let temp_dir = get_temp_dir(hmap! {
        "src/__init__.py" => "",
        "src/apps/posts/migrations/__init__.py" => "",
        "src/apps/posts/migrations/0001_initial.py" => "",
        "src/main.py" => "",
        "src/utils/logger.py" => "",
        "src/utils/slack/template.py" => "",
        "tests/test_main.py" => "",
        "tests/conftest.py" => "",
        "tests/_helpers.py" => "",
        "tests/utils/slack/test_template.py" => "",
        "tests/utils/test_logger.py" => "",
    });
    let dir_path = temp_dir.path();

    // Act
    let mut cmd = get_cmd();
    let assert = cmd
        .arg("check-file-pair")
        .args(&["--from", to_str!(dir_path.join("src"))])
        .args(&["--to", to_str!(dir_path.join("tests"))])
        .args(&["--include", "**/*.py"])
        .args(&["--exclude", "**/migrations/*.py", "**/_*.py"])
        .args(&["--expect", "{to}/{relative_from}/test_{filename}"])
        .assert();

    // Assert
    let result = assert.success().code(0);
    let (stdout, stderr) = parse_output(result.get_output());
    assert_snapshot!(normalize_console_output(
        stdout,
        hmap! {
            to_str!(dir_path) => "<temp_dir>"
        }
    ));
    assert_eq!(stderr, "");
    assert_eq!(
        list_dir(dir_path),
        &[
            "src/__init__.py",
            "src/apps/posts/migrations/0001_initial.py",
            "src/apps/posts/migrations/__init__.py",
            "src/main.py",
            "src/utils/logger.py",
            "src/utils/slack/template.py",
            "tests/_helpers.py",
            "tests/conftest.py",
            "tests/test_main.py",
            "tests/utils/slack/test_template.py",
            "tests/utils/test_logger.py"
        ]
    );
    Ok(())
}

/// Test for `--create-if-not-exists` option.
#[test]
fn test_create_if_not_exists() -> Result<()> {
    // Arrange
    let temp_dir = get_temp_dir(hmap! {
        "src/__init__.py" => "",
        "src/main.py" => "",
        "src/utils/logger.py" => "",
        "src/utils/slack/template.py" => "",
        "tests/test_main.py" => "",
    });
    let dir_path = temp_dir.path();

    // Act
    let mut cmd = get_cmd();
    let assert = cmd
        .arg("check-file-pair")
        .args(&["--from", to_str!(dir_path.join("src"))])
        .args(&["--to", to_str!(dir_path.join("tests"))])
        .args(&["--include", "**/*.py"])
        .args(&["--exclude", "**/_*.py"])
        .args(&["--expect", "{to}/{relative_from}/test_{filename}"])
        .args(&["--create-if-not-exists"])
        .assert();

    // Assert
    let result = assert.failure().code(1);
    let (stdout, stderr) = parse_output(result.get_output());
    assert_snapshot!(normalize_console_output(
        stdout,
        hmap! {
            to_str!(dir_path) => "<temp_dir>"
        }
    ));
    assert_eq!(first_line(stderr), "Error: Created 2 missing files.");
    assert_eq!(
        list_dir(dir_path),
        &[
            "src/__init__.py",
            "src/main.py",
            "src/utils/logger.py",
            "src/utils/slack/template.py",
            "tests/test_main.py",
            "tests/utils/slack/test_template.py",
            "tests/utils/test_logger.py",
        ]
    );
    Ok(())
}

/// Test for `--create-if-not-exists` with `--dry-run` option.
///
/// If the files do not exist, they should not be created,
/// but the output should indicate they would be created.
#[test]
fn test_create_if_not_exists_dry_run() -> Result<()> {
    // Arrange
    let temp_dir = get_temp_dir(hmap! {
        "src/__init__.py" => "",
        "src/main.py" => "",
        "src/utils/logger.py" => "",
        "src/utils/slack/template.py" => "",
        "tests/test_main.py" => "",
    });
    let dir_path = temp_dir.path();

    // Act
    let mut cmd = get_cmd();
    let assert = cmd
        .arg("--dry-run")
        .arg("check-file-pair")
        .args(&["--from", to_str!(dir_path.join("src"))])
        .args(&["--to", to_str!(dir_path.join("tests"))])
        .args(&["--include", "**/*.py"])
        .args(&["--exclude", "**/_*.py"])
        .args(&["--expect", "{to}/{relative_from}/test_{filename}"])
        .args(&["--create-if-not-exists"])
        .assert();

    // Assert
    let result = assert.failure().code(1);
    let (stdout, stderr) = parse_output(result.get_output());
    assert_snapshot!(normalize_console_output(
        stdout,
        hmap! {
            to_str!(dir_path) => "<temp_dir>"
        }
    ));
    assert_eq!(first_line(stderr), "Error: Created 2 missing files.");
    assert_eq!(
        list_dir(dir_path),
        &[
            "src/__init__.py",
            "src/main.py",
            "src/utils/logger.py",
            "src/utils/slack/template.py",
            "tests/test_main.py",
        ]
    );
    Ok(())
}
