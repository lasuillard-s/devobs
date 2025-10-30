use std::collections::HashMap;

use anyhow::Result;
use insta::assert_snapshot;
use sugars::hmap;

use crate::{helpers::{get_cmd, get_temp_dir, normalize_console_output, parse_output},
            to_str};

/// Test command with an empty directory.
#[test]
fn test_empty_directory() -> Result<()> {
    // Arrange
    let temp_dir = get_temp_dir(hmap! {
        "target/" => "",
    });
    let dir_path = temp_dir.path();

    // Act
    let mut cmd = get_cmd();
    let assert = cmd
        .arg("assert-diff")
        .args(&["--target", to_str!(dir_path.join("target"))])
        .arg("--")
        .args(&["echo", "Hello, World!"])
        .assert();

    // Assert
    let result = assert.success().code(0);
    let (stdout, stderr) = parse_output(result.get_output());
    assert_snapshot!(normalize_console_output(
        stdout,
        HashMap::<&str, &str>::new()
    ));
    assert_eq!(stderr, "");
    Ok(())
}

/// Test command with a nonexistent directory. It should exit with error.
#[test]
fn test_nonexistent_directory() -> Result<()> {
    // Arrange
    let temp_dir = get_temp_dir(hmap! {});
    let dir_path = temp_dir.path();

    // Act
    let mut cmd = get_cmd();
    let assert = cmd
        .arg("assert-diff")
        .args(&["--target", to_str!(dir_path.join("target"))])
        .arg("--")
        .args(&["echo", "Hello, World!"])
        .assert();

    // Assert
    let result = assert.failure().code(1);
    let (stdout, stderr) = parse_output(result.get_output());
    assert_eq!(stdout, "");
    assert_snapshot!(normalize_console_output(
        stderr,
        hmap! {
            to_str!(dir_path) => "<temp_dir>"
        }
    ));
    Ok(())
}

/// If user provided no command, the program should exit with error.
#[test]
fn test_no_command_to_run() -> Result<()> {
    // Arrange
    let temp_dir = get_temp_dir(hmap! {
        "target/" => "",
    });
    let dir_path = temp_dir.path();

    // Act
    let mut cmd = get_cmd();
    let assert = cmd
        .arg("assert-diff")
        .args(&["--target", to_str!(dir_path.join("target"))])
        .assert();

    // Assert
    let result = assert.failure().code(1);
    let (stdout, stderr) = parse_output(result.get_output());
    assert_eq!(stdout, "");
    assert_snapshot!(normalize_console_output(
        stderr,
        hmap! {
            to_str!(dir_path) => "<temp_dir>"
        }
    ));
    Ok(())
}

/// Test with a populated directory but no changes after command execution.
#[test]
fn test_no_changes() -> Result<()> {
    // Arrange
    let temp_dir = get_temp_dir(hmap! {
        "target/file1.txt" => "Content of file 1",
        "target/file2.txt" => "Content of file 2",
        "target/subdir/file3.txt" => "Content of file 3",
    });
    let dir_path = temp_dir.path();

    // Act
    let mut cmd = get_cmd();
    let assert = cmd
        .arg("assert-diff")
        .args(&["--target", to_str!(dir_path.join("target"))])
        .arg("--")
        .args(&["echo", "Hello, World!"])
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
    Ok(())
}

/// Test for changes (create new file) detected after command execution.
#[test]
fn test_changes_create() -> Result<()> {
    // Arrange
    let temp_dir = get_temp_dir(hmap! {
        "target/file1.txt" => "Content of file 1",
        "target/file2.txt" => "Content of file 2",
        "target/subdir/file3.txt" => "Content of file 3",
    });
    let dir_path = temp_dir.path();

    // Act
    let mut cmd = get_cmd();
    let assert = cmd
        .current_dir(dir_path)
        .arg("assert-diff")
        .args(&["--target", to_str!(dir_path.join("target"))])
        .arg("--")
        .args(&[
            "sh",
            "-c",
            r#"
echo 'New file content' > target/new_file.txt
"#
            .trim(),
        ])
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
    assert_snapshot!(normalize_console_output(
        stderr,
        hmap! {
            to_str!(dir_path) => "<temp_dir>"
        }
    ));
    Ok(())
}

/// Test for changes (modify existing file) detected after command execution.
#[test]
fn test_changes_modify() -> Result<()> {
    // Arrange
    let temp_dir = get_temp_dir(hmap! {
        "target/file1.txt" => "Content of file 1",
        "target/file2.txt" => "Content of file 2",
        "target/subdir/file3.txt" => "Content of file 3",
    });
    let dir_path = temp_dir.path();

    // Act
    let mut cmd = get_cmd();
    let assert = cmd
        .current_dir(dir_path)
        .arg("assert-diff")
        .args(&["--target", to_str!(dir_path.join("target"))])
        .arg("--")
        .args(&[
            "sh",
            "-c",
            r#"
echo 'Modified content' > target/file2.txt;
"#
            .trim(),
        ])
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
    assert_snapshot!(normalize_console_output(
        stderr,
        hmap! {
            to_str!(dir_path) => "<temp_dir>"
        }
    ));
    Ok(())
}

/// Test for changes (delete existing file) detected after command execution.
#[test]
fn test_changes_delete() -> Result<()> {
    // Arrange
    let temp_dir = get_temp_dir(hmap! {
        "target/file1.txt" => "Content of file 1",
        "target/file2.txt" => "Content of file 2",
        "target/subdir/file3.txt" => "Content of file 3",
    });
    let dir_path = temp_dir.path();

    // Act
    let mut cmd = get_cmd();
    let assert = cmd
        .current_dir(dir_path)
        .arg("assert-diff")
        .args(&["--target", to_str!(dir_path.join("target"))])
        .arg("--")
        .args(&[
            "sh",
            "-c",
            r#"
rm target/subdir/file3.txt;
"#
            .trim(),
        ])
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
    assert_snapshot!(normalize_console_output(
        stderr,
        hmap! {
            to_str!(dir_path) => "<temp_dir>"
        }
    ));
    Ok(())
}

/// Test for on-command-error propagation (default behavior). The program
/// should exit with the original command's exit code.
#[test]
fn test_on_command_error_propagate() -> Result<()> {
    // Arrange
    let temp_dir = get_temp_dir(hmap! {
        "target/" => "",
    });
    let dir_path = temp_dir.path();

    // Act
    let mut cmd = get_cmd();
    let assert = cmd
        .current_dir(dir_path)
        .arg("assert-diff")
        .args(&["--target", to_str!(dir_path.join("target"))])
        .args(&["--on-command-error", "propagate"])
        .arg("--")
        .args(&["sh", "-c", "exit 42"])
        .assert();

    // Assert
    let result = assert.failure().code(42);
    let (stdout, stderr) = parse_output(result.get_output());
    assert_snapshot!(normalize_console_output(
        stdout,
        hmap! {
            to_str!(dir_path) => "<temp_dir>"
        }
    ));
    assert_eq!(stderr, "");
    Ok(())
}

/// Test for ignore on-command-error ignore. The program won't stop on error
/// in original command, keep running the program.
#[test]
fn test_on_command_error_ignore() -> Result<()> {
    // Arrange
    let temp_dir = get_temp_dir(hmap! {
        "target/" => "",
    });
    let dir_path = temp_dir.path();

    // Act
    let mut cmd = get_cmd();
    let assert = cmd
        .current_dir(dir_path)
        .arg("assert-diff")
        .args(&["--target", to_str!(dir_path.join("target"))])
        .args(&["--on-command-error", "ignore"])
        .arg("--")
        .args(&["sh", "-c", "exit 42"])
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
    Ok(())
}
