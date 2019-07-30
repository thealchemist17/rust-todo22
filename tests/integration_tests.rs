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

fn it_add_todo(){
    
}
