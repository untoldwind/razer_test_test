extern crate clap;
extern crate hidapi;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;

mod cli;
mod devices;
mod errors;

use clap::{App, SubCommand};

fn main() {
    let matches = App::new("razer_test test")
        .version("0.0.1")
        .about("Tests razer devices on a very low level")
        .subcommand(SubCommand::with_name("list").about("list all recognized devices"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("list") {
        cli::list_devices().unwrap();
    }
}
