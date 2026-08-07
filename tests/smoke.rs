use std::process::Command;

#[test]
fn esprit_version_runs() {
    let exe = env!("CARGO_BIN_EXE_esprit");

    let out = Command::new(exe)
        .arg("version")
        .output()
        .expect("failed to execute esprit");

    assert!(out.status.success());
}

#[test]
fn esprit_doctor_runs() {
    let exe = env!("CARGO_BIN_EXE_esprit");

    let out = Command::new(exe)
        .arg("doctor")
        .output()
        .expect("failed to execute esprit");

    assert!(out.status.success());
}
