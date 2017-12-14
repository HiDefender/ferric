use std::collections::HashMap;
use std::path::PathBuf;
use fnv::FnvHashMap;
use daikon::types::{VarKind, DecType, RepType};

mod types;
mod map;

// Need to instrument source code.
// Need to create a .decls file.

struct Instrumentor {
    compare_cntr: usize,
    src_to_dectype: FnvHashMap<&'static str, DecType>,
    dectype_to_reptype: FnvHashMap<DecType, RepType>,
    decls: String,
}

impl Instrumentor {
    pub fn new() -> Instrumentor {
        Instrumentor {
            compare_cntr: 0,
            src_to_dectype: map::map_src_to_dectype(),
            dectype_to_reptype: map::map_dectype_to_reptype(),
            decls: String::from(Instrumentor::gen_header()),
        }
    }
    pub fn instrument_file(&mut self, file: &mut String) {
        unimplemented!()
    }
    pub fn get_decls(&self) -> String {
        unimplemented!()
    }
    fn gen_header<'a>() -> &'a str {
        "input-language Rust\ndecl-version 2.0\nvar-comparability implicit\n"
    }
}

struct PPT {
    fn_name: String, //"..square(int,\_bool):::"
    vars: Vec<Variable>,
    exit: usize,
}
//This impl has multiple String gen ("generating") functions.
//Note: It is assumed that all String printing statements are called
//      via println not print.
impl PPT {
    fn gen_enter(&self) -> String {
        self.fn_name.clone() + "ENTER"
    }
    fn gen_exit(&self, num: usize) -> String {
        self.fn_name.clone() + "EXIT" + num.to_string().as_str()
    }
}

struct Variable {
    name: String,
    var_kind: VarKind,
    rep_type: RepType,
    dec_type: DecType,
    is_param: bool, //For simplicities sake "is_param" is the only flag
                    //  offered. Ideally all flags would be written as
                    //  an enum in types.rs. This variable would then be
                    //  replaced by a Vec<Flags>.
    compare: usize,
}
impl Variable {
    pub fn to_string(&self) -> String {
        let s = String::from("\tvariable ");
        s + self.name.as_str() + "\n" +
        "\t\tvar-kind " + self.var_kind.as_str() + "\n" +
        "\t\trep-type " + self.rep_type.as_str() + "\n" +
        "\t\tdec-type " + self.dec_type.as_str() + "\n" +
        self.flags_to_strings().as_str() + "\n" +
        "\t\tcomparability " + self.compare.to_string().as_str()

    }
    //Note, for simplicities sake this only prints "is_param".
    //  This function should be expanded to encompass all flags.
    fn flags_to_strings(&self) -> String {
        if self.is_param {
            return String::from("\t\tflags is_param")
        }
        String::new()
    }
}