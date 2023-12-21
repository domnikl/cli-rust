use std::process::Command;

#[test]
fn works() {
    let mut cmd = Command::new("ls");
    let res = cmd.output();

    assert!(res.is_ok());
}
