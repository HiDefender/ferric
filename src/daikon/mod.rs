use std::collections::HashMap;
use std::path::PathBuf;

mod types;
mod map;

// Need to instrument source code.
// Need to create a .decls file.

pub fn instrument_files(files: HashMap<PathBuf, String>) {
}

struct Instrumentor {
    compare_cntr: usize,
}

struct PPT {
    fn_name: String, //"..square(int,\_bool):::"

}

//This impl has multiple String gen ("generating") functions.
//Note: It is assumed that all String printing statements are called
//      via println not print.
impl PPT {
    fn gen_header() -> String {
        String::from("input-language Rust\ndecl-version 2.0\nvar-comparability implicit\n")
    }
    fn gen_enter(&self) -> String {
        unimplemented!();
    }
    fn gen_exit(&self, num: usize) {
        unimplemented!();
    }
}