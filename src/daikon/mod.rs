use fnv::FnvHashMap;
use daikon::types::{VarKind, DecType, RepType, PPTType};

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
            decls: String::from(Instrumentor::decls_header()),
        }
    }
    pub fn instrument_file(&mut self, file: &mut String) {
        unimplemented!()
    }
    pub fn get_decls(&self) -> String {
        unimplemented!()
    }
    fn decls_header<'a>() -> &'a str {
        "input-language Rust\ndecl-version 2.0\nvar-comparability implicit\n"
    }
    fn dtrace_header<'a>() -> &'a str {
        "eprintln!(\"input-language Rust\ndecl-version 2.0\nvar-comparability implicit\n\");"
    }
    fn dtrace_let_return_daikon_unique<'a>() -> &'a str {
        "let return_daikon_unique = "
    }
    fn dtrace_return_daikon_unique<'a>() -> &'a str {
        "return_daikon_unique"
    }
}

struct PPT {
    fn_name: String, //"..square(int,\_bool):::"
    vars: Vec<Variable>,
    exit: usize,
}
//This impl has multiple String functions.
//Note: It is assumed that all String printing statements are called
//      via println not print.
impl PPT {
    //Input: Takes a fn_line such as:
    //==================================
    //fn squar(x: i32, y: bool) -> i32 {
    //==================================
    //  Ignores whitespace, so the first two characters should be "fn",
    //  and the last character '{'
    pub fn new(fn_line: &str, src_to_dectype: FnvHashMap<&str, DecType>) -> PPT {
        let fn_line = fn_line.trim();
        let len = fn_line.len();
        assert_eq!(Some("{"), fn_line.get(len..));
        let mut fn_name = String::from("..");
        let mut iter = fn_line.split(|c: char| c.is_whitespace() || c == ':' || c == ',' || c == '(' || c == ')')
            .filter(|&s| !s.is_empty()).enumerate();
        while let Some((i, word)) = iter.next() {
            
            match (i, word) {
                (0, "fn") => {}
                (0, a) => panic!(String::from("First word of fn_line is not \"fn\", but: ") + a),
                (1, a) => {
                    fn_name.push_str(a);
                    fn_name.push_str("(");
                }
                (_, "->") => {  //Has a return type. Check if it is a support type.
                    if let Some((_, return_type)) = iter.next() {
                        if let Some((_, word)) = iter.next() {
                            if word == "{" { //For the time being we only support simple return types.
                                             //  This guards against tuples.
                                
                                //Check if return_type is in map_src_to_dectype()
                                if src_to_dectype.contains_key(return_type) {
                                    // name: String,
                                    // var_kind: VarKind,
                                    // rep_type: RepType,
                                    // dec_type: DecType,
                                    // is_param: bool, //For simplicities sake "is_param" is the only flag
                                    //                 //  offered. Ideally all flags would be written as
                                    //                 //  an enum in types.rs. This variable would then be
                                    //                 //  replaced by a Vec<Flags>.
                                    // compare: usize,

                                }
                            } else {
                                break;
                            }
                        } else {
                            panic!(String::from("No word after: ") + return_type);
                        }
                    } else {
                        panic!("No word after \"->\"");
                    }
                }
                (_, a) => unimplemented!(),
            }
            println!("{}", word);
        }
        //Create Vec<Variable>

        //Form PPT
        unimplemented!()
    }
    pub fn decls_to_string(&self, ppt_type: &PPTType) -> String {
        let mut s = String::from("ppt ") + self.fn_name.as_str();
        s = match ppt_type {
            &PPTType::enter => s + "ENTER" + "\n\tppt-type enter",
            &PPTType::subexit(num) => s + "EXIT" + num.to_string().as_str() +
            "\n\tppt-type subexit",
        };
        for var in &self.vars {
            match (var.name != "return", ppt_type) {
                (true, _) | (_, &PPTType::subexit(_)) => s = s + "\n" + var.decls_to_string().as_str(),
                _ => {}
            }
        }
        s + "\n"
    }
    pub fn dtrace_to_string(&self, ppt_type: &PPTType) -> String {
        let mut s = String::from("eprintln!(\"") + self.fn_name.as_str();
        s = match ppt_type {
            &PPTType::enter => s + "ENTER",
            &PPTType::subexit(num) => s + "EXIT" + num.to_string().as_str(),
        };
        let mut tail = String::from("\"");
        for var in &self.vars {
            match (var.name != "return", ppt_type) {
                (true, _) | (_, &PPTType::subexit(_)) => s = s + "\\n" + var.dtrace_to_string(&mut tail).as_str(),
                _ => {}
            }
        }
        s + tail.as_str() + ");"
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
//This impl has multiple String functions.
//Note: It is assumed that all String printing statements are called
//      via println not print.
impl Variable {
    pub fn dtrace_to_string(&self, tail: &mut String) -> String {
        //If the original source code looks like this:
        //===================================
        // fn squar(x: i32, y: bool) -> i32 {
        //     x*x;
        // }
        //===================================
        //The instrumented source code should look like this:
        //Note: return_daikon_unique
        //===================================
        // fn squar(x: i32, y: bool) -> i32 {
        //     eprintln!("..square(int,\\_bool):::ENTER\nx\n{}\n1\n\ny\n{}\n1\n", x ,y);
        //     let return_daikon_unique = x*x;
        //     eprintln!("..square(int,\\_bool):::EXIT0\nx\n{}\n1\n\ny\n{}\n1\nreturn\n{}\n1\n", x, y, return_daikon_unique);
        //     return_daikon_unique
        // }
        //===================================
        tail.push_str(", ");
        if self.name == "return" {
            tail.push_str("return_daikon_unique");
        } else {
            tail.push_str(self.name.as_str());
        }
        let s = self.name.clone();
        //Note: The hardcoded '1' could be replaced to change the modified flag.
        s + "\\n{}\\n1"
    }
    pub fn decls_to_string(&self) -> String {
        let s = String::from("\tvariable ");
        s + self.name.as_str() + "\n" +
        "\t\tvar-kind " + self.var_kind.as_str() + "\n" +
        "\t\trep-type " + self.rep_type.as_str() + "\n" +
        "\t\tdec-type " + self.dec_type.as_str() + "\n" +
        self.flags_to_string().as_str() + "\n" +
        "\t\tcomparability " + self.compare.to_string().as_str()

    }
    //Note, for simplicities sake this only prints "is_param".
    //  This function should be expanded to encompass all flags.
    fn flags_to_string(&self) -> String {
        if self.is_param {
            return String::from("\t\tflags is_param")
        }
        String::new()
    }
}