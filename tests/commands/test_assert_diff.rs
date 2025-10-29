use std::{collections::HashMap, fs::create_dir};

use anyhow::Result;
use assert_cmd::Command;
use insta::assert_snapshot;
use sugars::hmap;

use crate::{helpers::{get_temp_dir, list_dir, normalize_console_output, parse_output},
            to_str};

/// Test command with an empty directory.
#[test]
fn test_empty_directory() -> Result<()> {
    // Arrange
    let temp_dir = get_temp_dir(hmap! {});
    let dir_path = temp_dir.path();
    create_dir(dir_path.join("target"))?;

    // Act
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    let assert = cmd
        .arg("--no-colors")
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
    assert_eq!(list_dir(&dir_path.to_path_buf()), &[] as &[&str]);
    Ok(())
}
