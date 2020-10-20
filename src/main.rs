#![warn(missing_docs)]

use clap::{App, Arg};
use std::fs;
use stretch::start_main_loop;

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let arg_results = App::new("Stretch Game Engine")
        .version(version)
        .author("Mortoc <mortoc@protonmail.com>")
        .about("A data-driven WebGPU game engine built in rust")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .takes_value(true)
                .about("URI for the game config json file"),
        )
        .get_matches();

    let config_path = arg_results
        .value_of("config")
        .expect("--config {config_file} is required");

    let config_string = match fs::read_to_string(config_path) {
        Ok(s) => s,
        Err(e) => panic!("{:?}", e),
    };

    let config = serde_json::from_str(&config_string).expect("Unable to parse config json");

    start_main_loop(&config);
}
