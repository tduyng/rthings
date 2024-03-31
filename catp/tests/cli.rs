use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::{error::Error, fs};

const PRG: &str = "catp";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";

#[test]
fn usage() -> Result<(), Box<dyn Error>> {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("Usage"));
    }

    Ok(())
}

#[allow(dead_code)]
fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

fn run(args: &[&str], expected_file: &str) -> Result<(), Box<dyn Error>>{
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(PRG)?.args(args).output().unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert_eq!(stdout, expected);

    Ok(())
}

fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> Result<(), Box<dyn Error>>{
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;

    let output = Command::cargo_bin(PRG)?.write_stdin(input).args(args).output().unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert_eq!(stdout, expected);

    Ok(())
}

#[test]
fn skips_bad_file() -> Result<(), Box<dyn Error>> {
    let bad = gen_bad_file();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

#[test]
fn bustle_stdin() -> Result<(), Box<dyn Error>>{
    run_stdin(BUSTLE, &["-"], "tests/expected/the-bustle.txt.stdin.out")
}

#[test]
fn bustle_stdin_n() -> Result<(), Box<dyn Error>> {
    run_stdin(
        BUSTLE,
        &["-n", "-"],
        "tests/expected/the-bustle.txt.n.stdin.out",
    )
}

#[test]
fn bustle_stdin_b() -> Result<(), Box<dyn Error>> {
    run_stdin(
        BUSTLE,
        &["-b", "-"],
        "tests/expected/the-bustle.txt.b.stdin.out",
    )
}

// --------------------------------------------------
#[test]

fn empty_n() -> Result<(), Box<dyn Error>>  {
    run(&["-n", EMPTY], "tests/expected/empty.txt.n.out")
}

#[test]
fn empty_b() -> Result<(), Box<dyn Error>>  {
    run(&["-b", EMPTY], "tests/expected/empty.txt.b.out")
}

#[test]
fn fox() -> Result<(), Box<dyn Error>>  {
    run(&[FOX], "tests/expected/fox.txt.out")
}

#[test]
fn fox_n() -> Result<(), Box<dyn Error>>  {
    run(&["-n", FOX], "tests/expected/fox.txt.n.out")
}

#[test]
fn fox_b() -> Result<(), Box<dyn Error>>  {
    run(&["-b", FOX], "tests/expected/fox.txt.b.out")
}

#[test]
fn spiders() -> Result<(), Box<dyn Error>>  {
    run(&[SPIDERS], "tests/expected/spiders.txt.out")
}

#[test]
fn spiders_n() -> Result<(), Box<dyn Error>>  {
    run(&["--number", SPIDERS], "tests/expected/spiders.txt.n.out")
}

#[test]
fn spiders_b() -> Result<(), Box<dyn Error>>  {
    run(
        &["--number-nonblank", SPIDERS],
        "tests/expected/spiders.txt.b.out",
    )
}

#[test]
fn bustle() -> Result<(), Box<dyn Error>>  {
    run(&[BUSTLE], "tests/expected/the-bustle.txt.out")
}

#[test]
fn bustle_n() -> Result<(), Box<dyn Error>>  {
    run(&["-n", BUSTLE], "tests/expected/the-bustle.txt.n.out")
}

#[test]
fn bustle_b() -> Result<(), Box<dyn Error>>  {
    run(&["-b", BUSTLE], "tests/expected/the-bustle.txt.b.out")
}

#[test]
fn all() -> Result<(), Box<dyn Error>>  {
    run(&[FOX, SPIDERS, BUSTLE], "tests/expected/all.out")
}

#[test]
fn all_n() -> Result<(), Box<dyn Error>>  {
    run(&[FOX, SPIDERS, BUSTLE, "-n"], "tests/expected/all.n.out")
}

#[test]
fn all_b() -> Result<(), Box<dyn Error>>  {
    run(&[FOX, SPIDERS, BUSTLE, "-b"], "tests/expected/all.b.out")
}
