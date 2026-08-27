use assert_cmd::Command;
use assert_fs::{TempDir, prelude::*};
use predicates::prelude::*;

#[test]
fn init_creates_skeleton() {
    let temp = TempDir::new().unwrap();

    Command::cargo_bin("plumbum")
        .unwrap()
        .current_dir(&temp)
        .args(["init", "--no-input", "myproj", "--scope", "acme"])
        .assert()
        .success();

    temp.child("plumbum.toml")
        .assert(predicate::path::is_file());
    temp.child(".plumbum/configs/places")
        .assert(predicate::path::is_dir());
    temp.child(".plumbum/configs/packages")
        .assert(predicate::path::is_dir());
    temp.child(".plumbum/configs/scripts")
        .assert(predicate::path::is_dir());
    temp.child("places").assert(predicate::path::is_dir());
    temp.child("packages").assert(predicate::path::is_dir());
    temp.child("scripts").assert(predicate::path::is_dir());
    temp.child("plumbum.toml")
        .assert(predicate::str::contains("name = \"myproj\""));
}

#[test]
fn init_uses_dir_name_when_name_omitted() {
    let temp = TempDir::new().unwrap();
    let os_name = temp.file_name().unwrap();
    let name = os_name.to_str().unwrap();

    Command::cargo_bin("plumbum")
        .unwrap()
        .current_dir(&temp)
        .args(["init", "--no-input", "--scope", "acme"])
        .assert()
        .success();

    temp.child("plumbum.toml")
        .assert(predicate::str::contains(format!("name = \"{name}\"")));
}

#[test]
fn init_fails_in_a_plumbum_project() {
    let temp = TempDir::new().unwrap();
    temp.child("plumbum.toml").write_str("hi").unwrap();

    Command::cargo_bin("plumbum")
        .unwrap()
        .current_dir(&temp)
        .args(["init", "--no-input", "--scope", "acme"])
        .assert()
        .failure();
}

#[test]
fn init_rejects_an_invalid_scope() {
    let temp = TempDir::new().unwrap();

    Command::cargo_bin("plumbum")
        .unwrap()
        .current_dir(&temp)
        .args(["init", "--no-input", "myproj", "--scope", "Hello, world!"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("lowercase letters"));

    temp.child("plumbum.toml")
        .assert(predicate::path::missing());
    temp.child(".plumbum").assert(predicate::path::missing());
}

#[test]
fn init_does_not_leave_a_manifest_when_scaffolding_fails() {
    let temp = TempDir::new().unwrap();
    temp.child("places").write_str("").unwrap();

    Command::cargo_bin("plumbum")
        .unwrap()
        .current_dir(&temp)
        .args(["init", "--no-input", "--scope", "acme"])
        .assert()
        .failure();

    temp.child("plumbum.toml")
        .assert(predicate::path::missing());
}

/// Arguments given on the command line answer their question outright, so no
/// prompt is shown. The test harness has no TTY, so any prompt would fail here.
#[test]
fn init_skips_prompts_when_every_answer_is_given() {
    let temp = TempDir::new().unwrap();

    Command::cargo_bin("plumbum")
        .unwrap()
        .current_dir(&temp)
        .args([
            "init",
            "myproj",
            "--scope",
            "acme",
            "--tool-manager",
            "mise",
        ])
        .assert()
        .success();

    temp.child("plumbum.toml")
        .assert(predicate::str::contains("tool_manager = \"mise\""));
}

/// A --scope that skipped the prompt still has to be validated.
#[test]
fn init_validates_a_scope_that_skipped_the_prompt() {
    let temp = TempDir::new().unwrap();

    Command::cargo_bin("plumbum")
        .unwrap()
        .current_dir(&temp)
        .args([
            "init",
            "myproj",
            "--scope",
            "Hello, world!",
            "--tool-manager",
            "rokit",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("lowercase letters"));

    temp.child("plumbum.toml")
        .assert(predicate::path::missing());
}
