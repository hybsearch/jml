extern crate phylip_format;
use phylip_format::Phylip;
use phylip_format::PhylipSample;
use std::env;
use std::fs::File;
use std::io::prelude::*;
extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("phylip_validator")
        .version("1.0")
        .about("Parse/dump/validate a PHYLIP file")
        .author("Hawken Rives")
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Should we print the file as Rust structs?"),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let debug: bool = matches.is_present("debug");
    let filename = matches.value_of("INPUT").unwrap();
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    match Phylip::from_str(contents) {
        Ok(phy) => {
            if debug {
                println!("{:?}", phy)
            } else {
                println!("{}", phy)
            }
        }
        Err(err) => println!("{:?}", err),
    }
}
