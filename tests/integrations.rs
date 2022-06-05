use assert_cmd::{assert::Assert, Command};
use color_eyre::eyre::Result;

#[test]
/// Make sure --help runs. This indicated the binary works
fn test_helper() -> Result<()> {
    let mut cmd: Command = Command::cargo_bin("garden")?;
    let assert: Assert = cmd.arg("--help").assert();
    
    assert.success().stderr("");
    
    Ok(())
}

#[test]
/// Make sure we have a write command by running `garden write --help`.
fn test_write_help() {
    let mut cmd: Command = Command::cargo_bin("garden").unwrap();
    let assert: Assert = cmd
        .arg("write")
        .arg("--help")
        .assert();

    assert.success().stderr("");
}

#[test]
/// execute the write command, saving a file out.
fn test_write() {
    let mut cmd: Command = Command::cargo_bin("garden").unwrap();
    let assert: Assert = cmd.arg("write")
        .arg("-t")
        .arg("My Digital Garden")
        .assert();

    assert.success().stderr("");
}
