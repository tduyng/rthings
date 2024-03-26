use assert_cmd::Command;
use predicates::*;
use std::error::Error;
use std::fs;

#[test]
fn dies_no_args() -> Result<(), Box<dyn Error>> {
    Command::cargo_bin("echop")?
        .assert()
        .failure()
        .stderr(predicates::str::contains("Usage"));

    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> Result<(), Box<dyn Error>> {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin("echop")?
        .args(args)
        .output()
        .expect("fail");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert_eq!(stdout, expected);

    Ok(())
}

#[test]
fn hello1() -> Result<(), Box<dyn Error>> {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> Result<(), Box<dyn Error>> {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> Result<(), Box<dyn Error>> {
    run(&["Hello there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> Result<(), Box<dyn Error>> {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
