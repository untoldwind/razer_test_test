extern crate clap;
extern crate hidapi;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate env_logger;
extern crate hex;
#[macro_use]
extern crate log;

mod cli;
mod devices;
mod errors;

use clap::{App, Arg, SubCommand};
use devices::Color;

fn main() {
    let matches = App::new("razer_test test")
        .version("0.0.1")
        .about("Tests razer devices on a very low level")
        .arg(Arg::with_name("debug").short("D").long("debug").help("Enable debug"))
        .subcommand(SubCommand::with_name("list").about("list all recognized devices"))
        .subcommand(SubCommand::with_name("get-brightness").about("get brightness"))
        .subcommand(SubCommand::with_name("set-color").about("set color").arg(Arg::with_name("color").required(true)))
        .get_matches();

    let mut log_builder = env_logger::Builder::from_default_env();

    if matches.is_present("debug") {
        log_builder.filter(None, log::LevelFilter::Debug);
    } else {
        log_builder.filter(None, log::LevelFilter::Info);
    }
    log_builder.init();

    if let Some(_) = matches.subcommand_matches("list") {
        cli::list_devices().unwrap();
    } else if let Some(_) = matches.subcommand_matches("get-brightness") {
        cli::get_brightness().unwrap();
    } else if let Some(sub_matches) = matches.subcommand_matches("set-color") {
        let color = Color::parse(sub_matches.value_of("color").unwrap()).unwrap();
        cli::set_color(color).unwrap();
    }
}
