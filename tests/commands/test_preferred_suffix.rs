use anyhow::Result;
use assert_cmd::Command;
use insta::assert_snapshot;
use sugars::hmap;

use crate::{helpers::{get_temp_dir, list_dir, normalize_console_output, parse_output},
            to_str};

#[test]
fn test_no_rule() -> Result<()> {
    // Arrange
    let temp_dir = get_temp_dir(hmap! {
        "my-file.txt" => "",
    });

    // Act
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    let assert = cmd
        .arg("--no-colors")
        .arg("preferred-suffix")
        .args(&["my-file.txt"])
        .assert();

    // Assert
    let result = assert.success().code(0);
    let (stdout, stderr) = parse_output(result.get_output());
    assert_snapshot!(normalize_console_output(
        stdout,
        hmap! {
            to_str!(temp_dir.path()) => "<temp_dir>"
        }
    ));
    assert_eq!(stderr, "");
    assert_eq!(list_dir(temp_dir.path()), &["my-file.txt"]);
    Ok(())
}
