use assert_cmd::Command;

#[test]
fn it_echos_with_newline() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().stdout("hello\n");
}

#[test]
fn it_echos_without_newline() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.args(&["-n", "hello"]).assert().stdout("hello");
}

#[test]
fn it_echos_string_literal() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("1 2 3 4 5").assert().stdout("1 2 3 4 5\n");
}

#[test]
fn it_echos_positional_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.args(&["1", "2", "3", "4", "5"])
        .assert()
        .stdout("1 2 3 4 5\n");
}
