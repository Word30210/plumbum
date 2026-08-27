use assert_cmd::Command;
use assert_fs::{TempDir, prelude::*};
use plumbum::manifest::{Manifest, ToolManager};

#[test]
fn init_writes_a_manifest_that_round_trips() {
    let temp = TempDir::new().unwrap();

    Command::cargo_bin("plumbum")
        .unwrap()
        .current_dir(&temp)
        .args([
            "init",
            "--no-input",
            "myproj",
            "--scope",
            "acme",
            "--tool-manager",
            "mise",
        ])
        .assert()
        .success();

    let raw = std::fs::read_to_string(temp.child("plumbum.toml").path()).unwrap();
    let manifest: Manifest =
        toml::from_str(&raw).expect("plumbum.toml should deserialize into Manifest");

    assert_eq!(
        manifest,
        Manifest {
            name: "myproj".to_string(),
            scope: "acme".to_string(),
            version: "0.1.0".to_string(),
            tool_manager: ToolManager::Mise,
        }
    );
}

#[test]
fn init_defaults_the_tool_manager_to_rokit() {
    let temp = TempDir::new().unwrap();

    Command::cargo_bin("plumbum")
        .unwrap()
        .current_dir(&temp)
        .args(["init", "--no-input", "myproj", "--scope", "acme"])
        .assert()
        .success();

    let raw = std::fs::read_to_string(temp.child("plumbum.toml").path()).unwrap();
    let manifest: Manifest = toml::from_str(&raw).unwrap();

    assert_eq!(manifest.tool_manager, ToolManager::Rokit);
}
