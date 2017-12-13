mod file;
mod daikon;
#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    file::ferric_clean();
    //file::check_or_create_ferric_folder();
    let files = file::read_cur_src().unwrap();
    for pathbuf in files.keys() {
        println!("{:?}", pathbuf);
    }
    file::create_and_write_files(&files);
}