use clap::{App, Arg, SubCommand};
fn main() {
    let matches = App::new("rust-todo22")
        .version("1.0")
        .author("Luca Mancinelli <lucamancinelli17@gmail.com>")
        .about("rust-todo")
        .arg(Arg::with_name("add").requires("todo"))
        .arg(Arg::with_name("todo").help("todo"))
        .arg(Arg::with_name("list"))
        .arg(Arg::with_name("edit").requires("id"))
        .arg(Arg::with_name("remove").requires("id"))
        .arg(Arg::with_name("clear"))
        .arg(Arg::with_name("id"))
        .get_matches();

    if matches.is_present("add") {
        if let Some(matches) = matches.value_of("todo") {
            println!("{}", matches);
        }
    }
}
