use assert_cmd::Command;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(args).assert().success().stdout(expected);
    Ok(())
}

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage"));

    Ok(())
}

#[test]
fn runs_with_newline() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello").assert().success().stdout("hello\n");
    Ok(())
}

#[test]
fn runs_with_out_newline() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello")
        .arg("world")
        .assert()
        .success()
        .stdout("hello world\n");
    Ok(())
}

#[test]
fn compare_hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn compare_hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn compare_hello1_no_newline() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn compare_hello2_no_newline() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
