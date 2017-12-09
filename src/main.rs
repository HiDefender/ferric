mod file;

fn main() {
    file::check_or_create_ferric_folder();
    file::ferric_clean();
}