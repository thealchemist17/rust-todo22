use clap::{App, Arg, SubCommand};
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
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
                .default_value("todo.txt")
                .value_name("config")
                .help("sets a custom config file"),
        )
        .get_matches();
    let mut path = "todo.txt";
    if matches.is_present("config") {
        if let Some(val) = matches.value_of("config") {
            path = val;
            if !Path::new(val).exists() {
                File::create(val).unwrap();
            }
        }
    }
    // add cmd
    if let Some(matches) = matches.subcommand_matches("add") {
        add(path, matches.value_of("text").unwrap());
    }

    // list cmd
    if let Some(_) = matches.subcommand_matches("list") {
        let f = File::open(path).expect("Unable to open file");
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
            let f = BufReader::new(File::open(path).unwrap());
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
                            new_file.push_str("\n");
                        } else {
                            new_file.push_str(&line);
                            new_file.push_str("\n");
                        }
                    }

                    Err(e) => panic!("Error reading file: {}", e),
                }
            }
            let mut ofile = File::create(path).expect("unable to create file");
            ofile
                .write_all(new_file.as_bytes())
                .expect("unable to write");
        }
    }

    // remove cmd
    if let Some(matches) = matches.subcommand_matches("remove") {
        if matches.is_present("id") {
            let id = matches.value_of("id").unwrap();
            let f = BufReader::new(File::open(path).unwrap());
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
            let mut ofile = File::create(path).expect("unable to create file");
            ofile.write_all(c.as_bytes()).expect("unable to write");

            println!("removed todo with id: {}", id);
        }
    }

    // clear cmd
    if let Some(_) = matches.subcommand_matches("clear") {
        File::create(path).expect("unable to create file");
    }
}

// fn get_time(x: Tm) -> String {
//     format!("{}:{}:{}", x.tm_hour, x.tm_min, x.tm_sec)
// }

fn add(path: &str, text: &str) {
    let f = BufReader::new(File::open(path).unwrap());
    let mut id = 0;
    for line in f.lines() {
        match line {
            Ok(_) => {
                id = id + 1;
            }
            Err(_) => (),
        };
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();
    let content = format!("{}. {}\n", id, text);
    file.write(content.as_bytes()).expect("failed");

    println!("added todo with id: {}", id);
}
