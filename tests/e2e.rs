use std::process::Command;

fn run(args:&[&str]){
    let exe=env!("CARGO_BIN_EXE_esprit");

    let out=Command::new(exe)
        .args(args)
        .output()
        .unwrap();

    assert!(out.status.success(),"{}",String::from_utf8_lossy(&out.stderr));
}

#[test]
fn version(){
    run(&["version"]);
}

#[test]
fn doctor(){
    run(&["doctor"]);
}

#[test]
fn config(){
    run(&["config"]);
}
