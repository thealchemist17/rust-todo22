use assert_cmd::prelude::*;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process::Command;
#[test]
fn it_runs() {
    Command::cargo_bin("rust-todo22")
        .unwrap()
        .assert()
        .success();
}
#[test]
fn it_add_todo() {
    // init test file
    clean();
    // Call the command
    let cmd = Command::cargo_bin("rust-todo22")
        .unwrap()
        .arg("add")
        .arg("bufu")
        .unwrap();
    // Assert stdout
    let contents = "added todo with id: 0\n";
    cmd.assert().stdout(contents.to_string());
    // Check that the todo text is in the file
    let content = fs::read_to_string("todo.txt").expect("Failed to open file");
    assert_eq!(content, "0. bufu\n".to_string());
    // teardown
    clean();
}

fn clean() {
    match fs::remove_file("todo.txt") {
        Ok(_) => (),
        _ => (),
    }
    let mut ofile = File::create("index.txt").expect("unable to create file");
    let s = "0".to_string();
    ofile.write_all(s.as_bytes()).expect("unable to write");
}
