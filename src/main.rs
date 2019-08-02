mod todo;
use clap::{App, Arg, SubCommand,value_t};
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;
use todo::{Data, Todo};
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

    let path = matches.value_of("config").unwrap_or_default();

    let mut data: Data = if !Path::new(path).exists() {
        File::create(path).unwrap();
        Data::new()

    }else  {
        let f = BufReader::new(File::open(path).unwrap());
        serde_json::from_reader(f).unwrap()
    };

    // add cmd
    if let Some(matches) = matches.subcommand_matches("add") {
        data.add_from_text(matches.value_of("text").unwrap());
    }

    // list cmd
    if let Some(_) = matches.subcommand_matches("list") {
        list();
    }

    // edit cmd
    if let Some(matches) = matches.subcommand_matches("edit") {
        if matches.is_present("id") {
            data.edit(value_t!(matches, "id", u32).unwrap(), matches.value_of("text").unwrap());
        }
    }

    // remove cmd
    if let Some(matches) = matches.subcommand_matches("remove") {
        if matches.is_present("id") {
            data.remove(value_t!(matches, "id", u32).unwrap());

        }
    }

    // clear cmd
    if let Some(_) = matches.subcommand_matches("clear") {
        File::create(path).expect("unable to create file");
    }
    let file = File::create(path).unwrap();
    serde_json::to_writer(file, &data);
}

fn list() {
    // for x in todos {
    //     println!("{}", x);
    // }
    let contents = fs::read_to_string("todo.json").expect("Something went wrong reading the file");
    print!("{}", contents);
}
