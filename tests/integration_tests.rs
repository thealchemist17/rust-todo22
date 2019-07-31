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

#[test]

fn it_add_todo_config() {
    clean();
    let cmd = Command::cargo_bin("rust-todo22")
        .unwrap()
        .arg("config")
        .arg("todo.txt")
        .arg("add")
        .arg("bufu")
        .unwrap();
    // Assert stdout
    cmd.assert()
        .stdout("changing config file to: todo.txt\nadded todo with id: 0\n");
    let content = fs::read_to_string("todo.txt").expect("Failed to open file");
    assert_eq!(content, "0. bufu\n".to_string());
    clean();
}

#[test]
fn it_list() {
    clean();
    let mut file = std::fs::File::create("todo.txt").unwrap();
    file.write("1. bufu\n".as_bytes()).expect("failed to write");
    let cmd = Command::cargo_bin("rust-todo22")
        .unwrap()
        .arg("list")
        .unwrap();
    cmd.assert().stdout("1. bufu\n");

    clean();
}
#[test]
fn it_edit() {
    clean();
    let mut file = std::fs::File::create("todo.txt").unwrap();
    file.write("0. bufu\n".as_bytes()).expect("failed to write");
    let cmd = Command::cargo_bin("rust-todo22")
        .unwrap()
        .arg("edit")
        .arg("0")
        .unwrap();
    cmd.assert().stdout("please enter new message for id: 0\n");
    let content = fs::read_to_string("todo.txt").expect("Failed to open file");
    println!("{}", content);
    assert_eq!(content, "0.  \n".to_string());
    clean();
}
#[test]
fn it_remove() {}
#[test]
fn it_clear() {
    clean();

    let cmd = Command::cargo_bin("rust-todo22")
        .unwrap()
        .arg("clear")
        .unwrap();
    let todo_content = fs::read_to_string("todo.txt").expect("Failed to open file");
    assert_eq!(todo_content, "".to_string());
    let index_content = fs::read_to_string("index.txt").expect("Failed to open file");
    assert_eq!(index_content, "0".to_string());
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
