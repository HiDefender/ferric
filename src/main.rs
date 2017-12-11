mod file;
mod daikon;

fn main() {
    file::check_or_create_ferric_folder();
    let files = file::read_cur_src().unwrap();
    for pathbuf in files.keys() {
        println!("{:?}", pathbuf);
    }
    file::create_and_write_files(&files);
}