mod todo;
use clap::{App, Arg, SubCommand};
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufRead, BufReader};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use time::Tm;
use todo::*;
fn main() {
    let mut config = false;
    // Write
    let mut tmpfile: File = tempfile::tempfile().unwrap();
    write!(tmpfile, "Hello World!").unwrap();

    // Seek to start
    tmpfile.seek(SeekFrom::Start(0)).unwrap();

    // Read
    let mut buf = String::new();
    tmpfile.read_to_string(&mut buf).unwrap();
    assert_eq!("Hello World!", buf);

    // todos will be saved here
    if !Path::new("todo.txt").exists() {
        File::create("todo.txt").unwrap();
    }

    // index will be saved here
    if !Path::new("index.txt").exists() {
        let mut ofile = File::create("index.txt").expect("unable to create file");
        let s = "0".to_string();
        ofile.write_all(s.as_bytes()).expect("unable to write");
    }

    // checking if in the index.txt files there is a i32 that represent the current index
    let current_index =
        fs::read_to_string("index.txt").expect("Something went wrong reading the file");
    match current_index.parse::<i32>() {
        Ok(_) => (),
        Err(_) => {
            let mut ofile = File::create("index.txt").expect("unable to create file");
            let s = "0".to_string();
            ofile.write_all(s.as_bytes()).expect("unable to write");
        }
    };

    let matches = App::new("rust-todo22")
        .version("1.0")
        .author("Luca Mancinelli <lucamancinelli17@gmail.com>")
        .about("rust-todo22")
        .subcommand(
            SubCommand::with_name("add")
                .about("add todo")
                .arg(Arg::with_name("text").required(true)),
        )
        .subcommand(SubCommand::with_name("list"))
        .subcommand(SubCommand::with_name("edit").arg(Arg::with_name("id").required(true)))
        .subcommand(SubCommand::with_name("remove").arg(Arg::with_name("id").required(true)))
        .subcommand(SubCommand::with_name("clear"))
        .arg(
            Arg::with_name("config")
                .short("f")
                .long("config")
                .value_name("config")
                .help("sets a custom config file")
                .requires("path")
                .index(1),
        )
        .arg(
            Arg::with_name("path")
                .help("sets the config file to use")
                .takes_value(true)
                .index(2),
        )
        .arg(Arg::with_name("-c"))
        .get_matches();

    if matches.is_present("config") {
        config = true;
        if matches.value_of("config").unwrap() == "config"
            || matches.value_of("config").unwrap() == "f"
        {
            if let Some(path) = matches.value_of("path") {
                println!("changing config file to: {}", path);
            }
        } else {
            // config is present but its value is wrong
            // user input like 'cargo run bufu'
            // 'bufu' not valid as first arg
            panic!("{} is not a command", matches.value_of("config").unwrap());
        }
    } else {
        // config arg is not present so standard configuration
    }

    // add cmd
    if let Some(matches) = matches.subcommand_matches("add") {
        if matches.is_present("text") {
            let x = fs::read_to_string("index.txt").expect("Something went wrong reading the file");

            let my_int = x.parse::<i32>().unwrap() + 1;
            let mut ofile = File::create("index.txt").expect("unable to create file");
            let s = my_int.to_string();
            ofile.write_all(s.as_bytes()).expect("unable to write");
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("todo.txt")
                .unwrap();

            let mut content: String = String::new();
            content.push_str(&x);
            content.push_str(". ");
            content.push_str(matches.value_of("text").unwrap());
            content.push_str("\n");
            file.write(content.as_bytes()).expect("failed");
            println!("added todo with id: {}", x);
        } else {
            println!("Printing normally...");
        }
    }

    // list cmd
    if let Some(_) = matches.subcommand_matches("list") {
        let f = File::open("todo.txt").expect("Unable to open file");
        let f = BufReader::new(f);
        for line in f.lines() {
            let line = line.expect("Unable to read line");
            println!("{}", line);
        }
    }

    // edit cmd
    if let Some(matches) = matches.subcommand_matches("edit") {
        if matches.is_present("id") {
            let id = matches.value_of("id").unwrap();
            let mut new_message = String::new();
            let mut new_file = String::new();
            let f = BufReader::new(File::open("todo.txt").unwrap());
            for line in f.lines() {
                match line {
                    Ok(line) => {
                        let split = line.split(".");
                        let vec = split.collect::<Vec<&str>>();
                        if vec[0].trim() == id {
                            println!("please enter new message for id: {}", id);
                            io::stdin().read_line(&mut new_message).expect("failed");
                            new_file.push_str(id);
                            new_file.push_str(". ");
                            new_file.push_str(&new_message.trim());
                            new_file.push_str(" \n");
                        } else {
                            new_file.push_str(&line);
                            new_file.push_str(" \n");
                        }
                    }

                    Err(e) => panic!("Error reading file: {}", e),
                }
            }
            let mut ofile = File::create("todo.txt").expect("unable to create file");
            ofile
                .write_all(new_file.as_bytes())
                .expect("unable to write");

            println!("{}", matches.value_of("id").unwrap());
        }
    }

    // remove cmd
    if let Some(matches) = matches.subcommand_matches("remove") {
        if matches.is_present("id") {
            let id = matches.value_of("id").unwrap();
            let f = BufReader::new(File::open("todo.txt").unwrap());
            let mut c = String::new();
            for line in f.lines() {
                match line {
                    Ok(line) => {
                        let split = line.split(".");
                        let vec = split.collect::<Vec<&str>>();
                        if !(vec[0].trim() == id) {
                            c.push_str(&line);
                            c.push_str(" \n");
                        }
                    }

                    Err(e) => panic!("Error reading file: {}", e),
                }
            }
            let mut ofile = File::create("todo.txt").expect("unable to create file");
            ofile.write_all(c.as_bytes()).expect("unable to write");
            let c = format!("{} {} {}", "item with id", id, "has been deleted");
            println!("{}", c);
        }
    }

    // clear cmd
    if let Some(matches) = matches.subcommand_matches("clear") {
        File::create("todo.txt").expect("unable to create file");
        let mut ofile = File::create("index.txt").expect("unable to create file");
        let s = "0".to_string();
        ofile.write_all(s.as_bytes()).expect("unable to write");
    }

    let time = time::now();
}

fn get_time(x: Tm) -> String {
    format!("{}:{}:{}", x.tm_hour, x.tm_min, x.tm_sec)
}
