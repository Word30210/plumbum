/// PTY-backed integration tests for the interactive `plumbum init` prompts.
///
/// These tests spawn the binary inside a real pseudo-terminal so that
/// `std::io::stdin().is_terminal()` returns `true` and the inquire prompts
/// actually render. Each test drives the prompts with `rexpect` and then
/// asserts the resulting manifest.
///
/// NOTE: PTY tests are UNIX-only.
#[cfg(unix)]
mod pty {
    use assert_fs::{TempDir, prelude::*};
    use predicates::prelude::*;
    use rexpect::session::spawn_command;
    use std::process::Command;

    /// Resolve the path to the `plumbum` test binary built by Cargo.
    fn bin() -> std::path::PathBuf {
        assert_cmd::cargo::cargo_bin("plumbum")
    }

    // ── happy-path ───────────────────────────────────────────────────────────

    /// Answer every prompt interactively and verify the resulting manifest.
    #[test]
    fn interactive_answers_all_prompts() {
        let dir = TempDir::new().unwrap();

        let mut cmd = Command::new(bin());
        cmd.current_dir(&dir).args(["init"]);

        let mut p = spawn_command(cmd, Some(5_000)).expect("spawn");

        // name prompt – type a custom name
        p.exp_string("name of the project").expect("name prompt");
        p.send_line("myproj").expect("send name");

        // scope prompt – type a valid scope
        p.exp_string("pesde scope").expect("scope prompt");
        p.send_line("acme").expect("send scope");

        // tool-manager prompt – choose the first option (rokit) with Enter
        p.exp_string("tool manager").expect("tool-manager prompt");
        p.send_line("").expect("send tool-manager");

        p.exp_eof().expect("process exit");

        dir.child("plumbum.toml")
            .assert(predicate::str::contains("name = \"myproj\""));
        dir.child("plumbum.toml")
            .assert(predicate::str::contains("scope = \"acme\""));
    }

    /// The name prompt pre-fills the directory name as its default; pressing
    /// Enter without typing accepts that default.
    #[test]
    fn interactive_accepts_default_name() {
        let dir = TempDir::new().unwrap();
        let expected_name = dir.file_name().unwrap().to_str().unwrap().to_owned();

        let mut cmd = Command::new(bin());
        cmd.current_dir(&dir).args(["init"]);

        let mut p = spawn_command(cmd, Some(5_000)).expect("spawn");

        // Accept the default name.
        p.exp_string("name of the project").expect("name prompt");
        p.send_line("").expect("accept default name");

        // Provide a valid scope.
        p.exp_string("pesde scope").expect("scope prompt");
        p.send_line("acme").expect("send scope");

        // Accept the default tool-manager.
        p.exp_string("tool manager").expect("tool-manager prompt");
        p.send_line("").expect("accept default tool-manager");

        p.exp_eof().expect("process exit");

        dir.child("plumbum.toml")
            .assert(predicate::str::contains(format!(
                "name = \"{expected_name}\""
            )));
    }

    // ── validation ───────────────────────────────────────────────────────────

    /// Entering an invalid scope at the prompt should show an error message and
    /// re-display the prompt. A subsequent valid answer completes the flow.
    #[test]
    fn interactive_rejects_invalid_scope_then_accepts_valid_one() {
        let dir = TempDir::new().unwrap();

        let mut cmd = Command::new(bin());
        cmd.current_dir(&dir).args(["init", "myproj"]);

        let mut p = spawn_command(cmd, Some(5_000)).expect("spawn");

        // The name was already supplied as a positional argument, so we jump straight to scope.
        p.exp_string("pesde scope").expect("scope prompt");
        p.send_line("INVALID!").expect("send bad scope");

        // The validator should reject it.
        p.exp_string("lowercase letters").expect("validation error");

        // Inquire keeps the previous input when re-prompting; clear it with
        // enough backspaces and then supply a valid scope.
        // "INVALID!" is 8 characters; send a few extra to be safe.
        p.send("\x7f\x7f\x7f\x7f\x7f\x7f\x7f\x7f\x7f\x7f")
            .expect("backspace to clear");
        p.send_line("acme").expect("send valid scope");

        // Accept the default tool-manager.
        p.exp_string("tool manager").expect("tool-manager prompt");
        p.send_line("").expect("accept tool-manager");

        p.exp_eof().expect("process exit");

        dir.child("plumbum.toml")
            .assert(predicate::str::contains("scope = \"acme\""));
    }

    // ── cancellation ────────────────────────────────────────────────────────

    /// Sending Ctrl-C (SIGINT) at the name prompt must cause the process to
    /// exit with a non-zero status and must not leave a partial manifest.
    #[test]
    fn interactive_ctrl_c_aborts_and_leaves_no_manifest() {
        let dir = TempDir::new().unwrap();

        let mut cmd = Command::new(bin());
        cmd.current_dir(&dir).args(["init"]);

        let mut p = spawn_command(cmd, Some(5_000)).expect("spawn");

        p.exp_string("name of the project").expect("name prompt");
        p.send_control('c').expect("send ctrl-c");

        // The process should terminate (EOF on the PTY side).
        p.exp_eof().expect("process terminates after Ctrl-C");

        dir.child("plumbum.toml").assert(predicate::path::missing());
    }
}
