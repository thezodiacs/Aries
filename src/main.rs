extern crate clap;
extern crate pretty_env_logger;

#[macro_use]
extern crate log;

use clap::{App, Arg, SubCommand};
use std::error::Error;

mod gamma;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("indus")
        .version("0.1.0")
        .author("Avi Srivastava")
        .about("Generate gamma matrices for multimodal data.")
        .subcommand(
            SubCommand::with_name("gamma")
                .about("A subcommand to generate gamma fields.")
                .arg(
                    Arg::with_name("ipaths")
                        .long("ipaths")
                        .short("i")
                        .takes_value(true)
                        .required(true)
                        .multiple(true)
                        .help("path to the parent folders of matrices."),
                )
                .arg(
                    Arg::with_name("output")
                        .long("output")
                        .short("o")
                        .takes_value(true)
                        .required(true)
                        .help("path to the output path file."),
                ),
        )
        .get_matches();
    pretty_env_logger::init_timed();

    if let Some(sub_m) = matches.subcommand_matches("stats") {
        gamma::callback(&sub_m)?
    }

    Ok(())
}
