use assert_cmd::{assert::Assert, Command};
use assert_fs::prelude::*;
use color_eyre::eyre::Result;
use predicates::prelude::*;

fn setup_command() -> (Command, assert_fs::TempDir) {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd: Command = Command::cargo_bin("garden").unwrap();
    let fake_editor_path = std::env::current_dir()
        .expect("expect to be in a dir")
        .join("tests")
        .join("fake_editor.sh");

    if !fake_editor_path.exists() {
        panic!("fake editor shell script could not be found")
    }

    cmd
    .env("EDITOR", fake_editor_path.into_os_string())
    .env("GARDEN_PATH", temp_dir.path());

    (cmd, temp_dir)
}

#[test]
/// Make sure --help runs. This indicates the binary works
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

    let assert: Assert = cmd.arg("write").arg("--help").assert();

    assert.success().stderr("");
}

#[test]
/// execute the write command, saving a file out.
fn test_write_with_title() {
    let (mut cmd, temp_dir) = setup_command();
    let assert: Assert = cmd
        .arg("write")
        .arg("-t")
        .arg("atitle")
        .write_stdin("N\n".as_bytes())
        .assert();

    assert.success();

    temp_dir
        .child("atitle.md")
        .assert(predicate::path::exists());
}

#[test]
/// execute the write command, saving a file out.
fn test_write_with_written_title() {
    let (mut cmd, temp_dir) = setup_command();
    let assert: Assert = cmd
        .arg("write")
        .write_stdin("N\n".as_bytes())
        .assert();

    assert.success();

    temp_dir
        .child("testing.md")
        .assert(predicate::path::exists());
}
