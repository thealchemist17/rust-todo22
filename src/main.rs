mod todo;
use clap::{value_t, App, Arg, SubCommand};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use todo::Data;
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
        .subcommand(
            SubCommand::with_name("edit")
                .arg(Arg::with_name("id").required(true))
                .arg(Arg::with_name("text").required(true)),
        )
        .subcommand(SubCommand::with_name("remove").arg(Arg::with_name("id").required(true)))
        .subcommand(SubCommand::with_name("clear"))
        .subcommand(
            SubCommand::with_name("set_p")
                .arg(Arg::with_name("id").required(true))
                .arg(Arg::with_name("priority").required(true)),
        )
        .subcommand(
            SubCommand::with_name("set_s")
                .arg(Arg::with_name("id").required(true))
                .arg(Arg::with_name("state").required(true)),
        )
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
    } else {
        let f = BufReader::new(File::open(path).unwrap());
        match serde_json::from_reader(f) {
            Ok(json) => json,
            Err(_) => {
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(path)
                    .unwrap();

                let init = String::from("{{\"next_id\":0,\"todos\":[]}}");
                file.write_all(init.as_bytes()).expect("failed");
                Data::new()
            }
        }
    };

    // add cmd
    if let Some(matches) = matches.subcommand_matches("add") {
        data.add_from_text(matches.value_of("text").unwrap());
        println!("added todo with id: {}", data.get_last_id());
    }

    // list cmd
    if let Some(_) = matches.subcommand_matches("list") {
        println!("{}", data);
    }

    // edit cmd
    if let Some(matches) = matches.subcommand_matches("edit") {
        if matches.is_present("id") {
            data.edit(
                value_t!(matches, "id", u32).unwrap(),
                matches.value_of("text").unwrap(),
            );
        }
    }

    // remove cmd
    if let Some(matches) = matches.subcommand_matches("remove") {
        if matches.is_present("id") {
            data.remove(value_t!(matches, "id", u32).unwrap());
            println!(
                "removed todo with id: {}",
                value_t!(matches, "id", u32).unwrap()
            );
        }
    }

    // clear cmd
    if let Some(_) = matches.subcommand_matches("clear") {
        data = Data::new();
        File::create(path).expect("unable to create file");
    }
    if let Some(matches) = matches.subcommand_matches("set_p") {
        if matches.is_present("id") && matches.is_present("priority") {
            let mut value = todo::Priority::MEDIUM;
            match matches.value_of("priority").unwrap() {
                "HIGH" => (value = todo::Priority::HIGH),
                "MEDIUM" => (value = todo::Priority::MEDIUM),
                "LOW" => (value = todo::Priority::LOW),
                _ => (),
            };
            data.set_priority(value_t!(matches, "id", u32).unwrap(), value);
        }
    }

    if let Some(matches) = matches.subcommand_matches("set_s") {
        if matches.is_present("id") && matches.is_present("state") {
            let mut value = todo::State::TODO;
            match matches.value_of("state").unwrap() {
                "TODO" => (value = todo::State::TODO),
                "PROGRESS" => (value = todo::State::PROGRESS),
                "DONE" => (value = todo::State::DONE),
                _ => (),
            };
            data.set_state(value_t!(matches, "id", u32).unwrap(), value);
        }
    }

    let file = File::create(path).unwrap();
    serde_json::to_writer(file, &data).unwrap();
}
