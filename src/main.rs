mod todo;
use clap::{App, Arg, SubCommand};
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufRead, BufReader};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use tempfile::Builder;
use tempfile::NamedTempFile;
use time::Tm;
use todo::*;
fn main() {
    let mut config = false;
    let mut tmp_index_file = NamedTempFile::new().unwrap();
    let mut tmp_file = NamedTempFile::new().unwrap();
    // [test]
    // let mut tmpfile = NamedTempFile::new().unwrap();
    // write!(tmpfile, "abcde").unwrap();
    // tmpfile.seek(SeekFrom::Start(0)).unwrap();
    // let mut buf = String::new();s
    // tmpfile.read_to_string(&mut buf).unwrap();
    // assert_eq!("abcde", buf);

    // EXAMPLE
    // Create a named temporary file and open an independent file handle:
    // let text = "Brian was here. Briefly.";
    // // Create a file inside of `std::env::temp_dir()`.
    // let mut file1 = NamedTempFile::new().unwrap();

    // // Re-open it.
    // let mut file2 = file1.reopen().unwrap();

    // // Write some test data to the first handle.
    // file1.write_all(text.as_bytes()).unwrap();

    // // Read the test data using the second handle.
    // let mut buf = String::new();
    // file2.read_to_string(&mut buf).unwrap();
    // assert_eq!(buf, text);

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
                // temporary data will be saved here
                let mut tmp_file = Builder::new()
                    .prefix(path)
                    .suffix(".txt")
                    .rand_bytes(5)
                    .tempfile()
                    .unwrap();
                //write!(tmp_file, "abcde").unwrap();
                //tmp_file.seek(SeekFrom::Start(0)).unwrap();
                //let mut buf = String::new();
                //tmp_file.read_to_string(&mut buf).unwrap();
                //assert_eq!("abcde", buf);

                // path of the temporary file
                println!("changing config file to: {}", path);
                // temporary index will be saved here
                tmp_index_file = Builder::new()
                    .prefix("tmp_index")
                    .suffix(".txt")
                    .tempfile()
                    .unwrap();

                // initialize the content of tmp_index_file
                tmp_index_file.seek(SeekFrom::Start(0)).unwrap();

                write!(tmp_index_file, "0").unwrap();
                tmp_index_file.seek(SeekFrom::Start(0)).unwrap();
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
            if !config {
                let x =
                    fs::read_to_string("index.txt").expect("Something went wrong reading the file");

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
                // getting index
                let mut buf = String::new();
                tmp_index_file.read_to_string(&mut buf).unwrap();
                println!("{}", buf);
                let z = matches.value_of("text").unwrap();
                println!("{}", z);

                // getting todo message
                // write on a NamedTempFile add

                let mut content = String::new();
                content.push_str(&buf);
                content.push_str(". ");
                content.push_str(z);
                content.push_str("\n");
                tmp_file.seek(SeekFrom::Start(0)).unwrap();

                write!(tmp_file, "{}", content).unwrap();
                tmp_file.seek(SeekFrom::Start(0)).unwrap();
                let mut buf3 = String::new();
                tmp_file.read_to_string(&mut buf3).unwrap();
                assert_eq!("0. bufu\n", buf3);

                println!("added todo with id: {}", buf);
            }
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
