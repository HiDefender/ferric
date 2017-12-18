extern crate fnv;
#[macro_use]
extern crate clap;

mod file;
mod daikon;

use clap::App;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

fn main() {
    let matches = clap_app!(myapp =>
        (name: "ferric")
        (version: "0.1")
        (author: "Jared Soundy. <jared_soundy@gmail.com>")
        (about: "A Rust front-end for Daikon")
        (@subcommand daikon =>
            (about: "invokes daikon on trace files generated by \"cargo test\"")
        )
        (@subcommand clean =>
            (about: "removes the ferric folder")
        )
    ).get_matches();


    match matches.subcommand_name() {
        Some("daikon") => {
            file::check_or_create_ferric_folder().expect("Unexpected error while creating the ferric folder.");
            let files = file::read_cur_src().unwrap();
            let mut instumentor  = daikon::Instrumentor::new();
            let mut instr_files: HashMap<PathBuf, String> = HashMap::new();
            for (pathbuf, file) in &files {
                println!("{:?}", pathbuf);
                let file = instumentor.instrument_file(file);
                instr_files.insert(pathbuf.clone(), file);
            }
            file::create_and_write_files(&instr_files).expect("Unexpected error while writing instrumented code.");
        },
        Some("clean") => file::ferric_clean().expect("ferric clean failed"),
        None        => println!("Please use a subcommand. Try \"ferric help\""),
        _           => unreachable!(),
    }
}