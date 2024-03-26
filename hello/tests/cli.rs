use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    let output = cmd.output().expect("Failed to output cmd");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF8");
    assert_eq!(stdout, "Hello, world!\n");
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
