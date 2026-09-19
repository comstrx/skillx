use std::process::Command;

#[test]
fn hello_world () {

    let output = Command::new(env!("CARGO_BIN_EXE_skillx")).output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.trim(), "hello world");

}
