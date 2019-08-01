use assert_cmd::prelude::*;
use std::fs;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

fn get_temp_file() -> NamedTempFile {
    NamedTempFile::new().expect("Unable o create temporary file")
}

#[test]
fn it_runs() {
    Command::cargo_bin("rust-todo22")
        .unwrap()
        .assert()
        .success();
}

#[test]
fn it_add_todo() {
    let temp_file = get_temp_file();
    // Call the command
    let cmd = Command::cargo_bin("rust-todo22")
        .unwrap()
        .arg("-f")
        .arg(temp_file.path())
        .arg("add")
        .arg("bufu")
        .unwrap();
    // Assert stdout
    let contents = "added todo with id: 0\n";
    cmd.assert().stdout(contents.to_string());
    // Check that the todo text is in the file
    let content = fs::read_to_string(temp_file.path()).expect("Failed to open file");
    //assert_eq!(content, "0. bufu\n".to_string());
    assert_eq!(content, "{\"id\":\"0\",\"text\":\"bufu\"}\n");
}

#[test]
fn it_list() {
    let mut temp_file = get_temp_file();
    //writeln!(temp_file, "0. bufu").unwrap();
    writeln!(temp_file, "{{\"id\":\"0\",\"text\":\"bufu\"}}\n").unwrap();
    let cmd = Command::cargo_bin("rust-todo22")
        .unwrap()
        .arg("-f")
        .arg(temp_file.path())
        .arg("list")
        .unwrap();
    cmd.assert().stdout("0. bufu\n");
}

#[test]
fn it_edit() {
    let mut temp_file = get_temp_file();
    //writeln!(temp_file, "0. bufu").unwrap();
    writeln!(temp_file, "{{\"id\":\"0\",\"text\":\"bufu\"}}").unwrap();
    let cmd = Command::cargo_bin("rust-todo22")
        .unwrap()
        .arg("-f")
        .arg(temp_file.path())
        .arg("edit")
        .arg("0")
        .with_stdin()
        .buffer("test")
        .unwrap();
    cmd.assert().stdout("please enter new message for id: 0\n");
    let content = fs::read_to_string(temp_file.path()).expect("Failed to open file");
    //assert_eq!(content, "0. test\n".to_string());
    assert_eq!(content, "{\"id\":\"0\",\"text\":\"test\"}\n");
}

#[test]
fn it_remove() {
    let mut temp_file = get_temp_file();
    //writeln!(temp_file, "0. bufu").unwrap();
    writeln!(temp_file, "{{\"id\":\"0\",\"text\":\"bufu\"}}").unwrap();
    let cmd = Command::cargo_bin("rust-todo22")
        .unwrap()
        .arg("-f")
        .arg(temp_file.path())
        .arg("remove")
        .arg("0")
        .unwrap();
    cmd.assert().stdout("removed todo with id: 0\n");
    let content = fs::read_to_string(temp_file.path()).expect("Failed to open file");
    assert_eq!(content, "".to_string());
}

#[test]
fn it_clear() {
    let temp_file = get_temp_file();
    Command::cargo_bin("rust-todo22")
        .unwrap()
        .arg("-f")
        .arg(temp_file.path())
        .arg("clear")
        .unwrap();
    let todo_content = fs::read_to_string(temp_file.path()).expect("Failed to open file");
    assert_eq!(todo_content, "".to_string());
}
