mod file;
mod daikon;
use std::path::PathBuf;

fn main() {
    let files = file::read_cur_src();
    let () = files;
    for pathbuf in files.keys() {
        println!("{:?}", pathbuf);
    }
}