use assert_cmd::prelude::*;
use serde_json::Value;
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
        let data: Value = serde_json::from_str(&content).unwrap();
        assert_eq!(data["todos"][0]["text"], "bufu");
        assert_eq!(data["todos"][0]["id"], 0);
    }
}

#[test]
fn it_list() {
    let mut temp_file = get_temp_file();
    //writeln!(temp_file, "0. bufu").unwrap();
    writeln!(temp_file, "{{\"next_id\":1,\"todos\":[{{\"id\":0,\"text\":\"bufu\",\"state\":\"DONE\",\"priority\":\"MEDIUM\",\"creation_date\":\"2019-08-02 10:39:34\",\"last_updated_date\":\"2019-08-02 10:40:27\"}}]}}").unwrap();
    let cmd = Command::cargo_bin("rust-todo22")
        .unwrap()
        .arg("-f")
        .arg(temp_file.path())
        .arg("list")
        .unwrap();
    cmd.assert().stdout("0. bufu DONE MEDIUM\n\n");
}

#[test]
fn it_edit() {
    let mut temp_file = get_temp_file();
    //writeln!(temp_file, "0. bufu").unwrap();
    writeln!(temp_file, "{{\"next_id\":1,\"todos\":[{{\"id\":0,\"text\":\"bufu\",\"state\":\"TODO\",\"priority\":\"MEDIUM\",\"creation_date\":\"2019-08-02 10:39:34\",\"last_updated_date\":\"2019-08-02 10:40:27\"}}]}}").unwrap();
    let cmd = Command::cargo_bin("rust-todo22")
        .unwrap()
        .arg("-f")
        .arg(temp_file.path())
        .arg("edit")
        .arg("0")
        .arg("bufu")
        .unwrap();
    let content = fs::read_to_string(temp_file.path()).expect("Failed to open file");
    let data: Value = serde_json::from_str(&content).unwrap();
    assert_eq!(data["todos"][0]["text"], "bufu");
    assert_eq!(data["todos"][0]["id"], 0);
    assert_ne!(
        data["todos"][0]["last_updated_date"].to_string(),
        String::from("2019-08-02 10:40:27")
    );
}

#[test]
fn it_remove() {
    let mut temp_file = get_temp_file();
    //writeln!(temp_file, "0. bufu").unwrap();
    writeln!(temp_file, "{{\"next_id\":1,\"todos\":[{{\"id\":0,\"text\":\"bufu\",\"state\":\"TODO\",\"priority\":\"MEDIUM\",\"creation_date\":\"2019-08-02 10:39:34\",\"last_updated_date\":\"2019-08-02 10:40:27\"}}]}}").unwrap();
    let cmd = Command::cargo_bin("rust-todo22")
        .unwrap()
        .arg("-f")
        .arg(temp_file.path())
        .arg("remove")
        .arg("0")
        .unwrap();
    cmd.assert().stdout("removed todo with id: 0\n");
    let content = fs::read_to_string(temp_file.path()).expect("Failed to open file");
    let data: Value = serde_json::from_str(&content).unwrap();
    assert_eq!(data["next_id"], 1);
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
    assert_eq!(todo_content, "{\"next_id\":0,\"todos\":[]}");
}
