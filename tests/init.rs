use assert_cmd::Command;
use assert_fs::{TempDir, prelude::*};
use predicates::prelude::*;

#[test]
fn init_creates_skeleton() {
    let temp = TempDir::new().unwrap();

    Command::cargo_bin("plumbum")
        .unwrap()
        .current_dir(&temp)
        .args(["init", "myproj", "--scope", "acme"])
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
        .arg("init")
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
        .arg("init")
        .assert()
        .failure();
}
