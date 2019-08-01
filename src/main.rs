mod todo;
use clap::{App, Arg, SubCommand};
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;
use todo::Todo;
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
                .default_value("todo.json")
                .value_name("config")
                .help("sets a custom config file"),
        )
        .get_matches();
    let mut path = "todo.json";
    if matches.is_present("config") {
        if let Some(val) = matches.value_of("config") {
            path = val;
            if !Path::new(val).exists() {
                File::create(val).unwrap();
            }
        }
    }
    let mut todos: Vec<Todo> = Vec::new();

    let f = BufReader::new(File::open(path).unwrap());
    for line in f.lines() {
        match line {
            Ok(line) => {
                if !line.trim().is_empty() {
                    let data: Todo = serde_json::from_str(&line).unwrap();
                    todos.push(Todo::new(
                        data.get_id().to_string(),
                        data.get_text().to_string(),
                    ));
                }
            }

            Err(e) => panic!("Error reading file: {}", e),
        }
    }

    // add cmd
    if let Some(matches) = matches.subcommand_matches("add") {
        add(&mut todos, path, matches.value_of("text").unwrap());
    }

    // list cmd
    if let Some(_) = matches.subcommand_matches("list") {
        list();
    }

    // edit cmd
    if let Some(matches) = matches.subcommand_matches("edit") {
        if matches.is_present("id") {
            edit(&mut todos, path, matches.value_of("id").unwrap());
        }
    }

    // remove cmd
    if let Some(matches) = matches.subcommand_matches("remove") {
        if matches.is_present("id") {
            remove(&mut todos, path, matches.value_of("id").unwrap());
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

fn add(todos: &mut Vec<Todo>, path: &str, text: &str) {
    let mut last_index: i32 = 0;
    if todos.len() >= 1 {
        last_index = todos[todos.len() - 1].get_id().parse::<i32>().unwrap() + 1;
    }
    let x = Todo::new(last_index.to_string(), text.to_string());
    let todo = serde_json::to_string(&x).unwrap();
    todos.push(x);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();
    let c = format!("{}\n", todo);
    file.write(c.as_bytes()).expect("failed");
    println!("added todo with id: {}", last_index);
}

fn list() {
    //for x in todos {
    //    println!("{}", x);
    //}
    let contents = fs::read_to_string("todo.json").expect("Something went wrong reading the file");
    println!("{}", contents);
}

fn edit(todos: &mut Vec<Todo>, path: &str, id: &str) {
    let mut new_message = String::new();
    println!("please enter new message for id: {}", id);
    io::stdin().read_line(&mut new_message).expect("failed");
    let index: usize = id.parse::<usize>().unwrap();
    todos[index].set_text(new_message.trim().to_string());
    let mut ofile = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap();

    for x in todos {
        let c = format!("{}\n", serde_json::to_string(&x).unwrap());
        ofile.write(c.as_bytes()).expect("unable to write");
    }
}

fn remove(todos: &mut Vec<Todo>, path: &str, id: &str) {
    todos.retain(|bufu| bufu.get_id() != id);
    let mut ofile = File::create(path).expect("unable to create file");

    for x in todos {
        let c = format!("{}\n", serde_json::to_string(&x).unwrap());
        ofile.write(c.as_bytes()).expect("unable to write");
    }
    //ofile.write_all(c.as_bytes()).expect("unable to write");
}
