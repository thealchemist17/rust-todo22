mod todo;
use clap::{App, Arg, SubCommand};
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use time::Tm;
use todo::*;
fn main() {
    // Write
    let mut tmpfile: File = tempfile::tempfile().unwrap();
    write!(tmpfile, "Hello World!").unwrap();

    // Seek to start
    tmpfile.seek(SeekFrom::Start(0)).unwrap();

    // Read
    let mut buf = String::new();
    tmpfile.read_to_string(&mut buf).unwrap();
    assert_eq!("Hello World!", buf);

    if !Path::new("todo.txt").exists() {
        File::create("todo.txt").unwrap();
    }

    // index will be saved here
    if !Path::new("index.txt").exists() {
        let mut ofile = File::create("index.txt").expect("unable to create file");
        let s = "0".to_string();
        ofile.write_all(s.as_bytes()).expect("unable to write");
    }

    println!("{}", buf);
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
        .get_matches();

    if matches.is_present("config") {
        if matches.value_of("config").unwrap() == "config"
            || matches.value_of("config").unwrap() == "f"
        {
            if let Some(path) = matches.value_of("path") {
                println!("changing config file to: {}", path);
            }
        } else {
            // user input like 'cargo run bufu'
            // 'bufu' not valid as first arg
            panic!("error");
        }
    } else {
        //
        println!("default.conf");
    }

    // add cmd
    if let Some(matches) = matches.subcommand_matches("add") {
        if matches.is_present("text") {
            println!("{}", matches.value_of("text").unwrap());

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
        } else {
            println!("Printing normally...");
        }
    }

    // list cmd
    if let Some(_) = matches.subcommand_matches("list") {
        println!("list command");
    }

    // edit cmd
    if let Some(matches) = matches.subcommand_matches("edit") {
        if matches.is_present("id") {
            println!("{}", matches.value_of("id").unwrap());
        }
    }

    // remove cmd
    if let Some(matches) = matches.subcommand_matches("remove") {
        if matches.is_present("id") {
            println!("{}", matches.value_of("id").unwrap());
        }
    }

    let time = time::now();
}

fn get_time(x: Tm) -> String {
    format!("{}:{}:{}", x.tm_hour, x.tm_min, x.tm_sec)
}
