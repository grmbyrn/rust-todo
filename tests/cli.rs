use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn test_add_and_list() {
    // Add a task
    let mut cmd = Command::cargo_bin("rust-todo").unwrap();
    cmd.args(&["add", "Integration test task"])
        .assert()
        .success()
        .stdout(contains("Added task:"));

    // List tasks and check output
    let mut cmd = Command::cargo_bin("rust-todo").unwrap();
    cmd.args(&["list"])
        .assert()
        .success()
        .stdout(contains("Integration test task"));
}