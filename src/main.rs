use std::fs;

#[macro_use]
extern crate clap;
use clap::{Arg, App};

fn main() {
    let options = App::new("mcp")
        .version(crate_version!())
        .author(crate_authors!())
        .about("copy to multiple files.")
        .arg(Arg::with_name("inputs")
             .multiple(true)
             .required(true))
        .get_matches();

    let filenames: Vec<&str> = options.values_of("inputs").unwrap().collect();

    if let Some((first, files)) = filenames.split_first() {
        for file in files {
            let _ = fs::copy(first, file);
        }
    }
}
