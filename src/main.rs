extern crate fnv;
#[macro_use]
extern crate clap;

mod file;
mod daikon;

use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand_name() {
        Some("daikon") => {
            file::check_or_create_ferric_folder().expect("Unexpected error while creating the ferric folder.");
            let files = file::read_cur_src().unwrap();
            for pathbuf in files.keys() {
                println!("{:?}", pathbuf);
            }
            file::create_and_write_files(&files).expect("Unexpected error while writing instrumented code.");
        },
        Some("clean") => file::ferric_clean().expect("ferric clean failed"),
        None        => println!("Please use a subcommand. Try \"ferric help\""),
        _           => unreachable!(),
    }
}