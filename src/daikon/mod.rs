use fnv::FnvHashMap;
use daikon::types::{VarKind, DecType, RepType, PPTType};

mod types;
mod map;

// Need to instrument source code.
// Need to create a .decls file.

pub struct Instrumentor {
    src_to_dectype: FnvHashMap<&'static str, DecType>,
    dectype_to_reptype: FnvHashMap<DecType, RepType>,
    decls: String,
}

impl Instrumentor {
    pub fn new() -> Instrumentor {
        Instrumentor {
            src_to_dectype: map::map_src_to_dectype(),
            dectype_to_reptype: map::map_dectype_to_reptype(),
            decls: String::from(Instrumentor::decls_header()),
        }
    }
    pub fn instrument_file(&mut self, file: &String) -> String {
        //A guess at the upperbound instrumented file length.
        let mut inst_file = String::with_capacity((file.len() as f32 * 1.1) as usize);
        let mut iter = file.lines();
        while let Some(line) = iter.next() {
            inst_file.push_str(line);
            inst_file.push_str("\n");
            match (line.split_whitespace().nth(0), line.split_whitespace().nth(1), line.split_whitespace().last()) {
                (Some("fn"), _, Some("{")) | (Some("pub"), Some("fn"), Some("{")) => {
                    if !line.contains("main") {
                        let ppt = PPT::new(line, &self.src_to_dectype, &self.dectype_to_reptype);
                        inst_file.push_str(ppt.dtrace_to_string(&PPTType::Enter).as_str());
                        inst_file.push_str("\n");
                        self.decls.push_str(ppt.decls_to_string(&PPTType::Enter).as_str());
                        self.decls.push_str("\n");
                    }
                }
                _ => {
                    if line.contains("return") {
                        
                    } else {

                    }
                }
            }

        }
        inst_file
    }
    pub fn get_decls(&self) -> String {
        self.decls.clone()
    }
    fn decls_header<'a>() -> &'a str {
        "input-language Rust\ndecl-version 2.0\nvar-comparability implicit\n\n"
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
    pub fn new(fn_line: &str, src_to_dectype: &FnvHashMap<&str, DecType>,
                dectype_to_reptype: &FnvHashMap<DecType, RepType>) -> PPT {
        let fn_line = fn_line.trim();
        let mut fn_name = String::from("..");
        let mut iter = fn_line.split(|c: char| c.is_whitespace() || c == ':' || c == ',' || c == '(' || c == ')')
            .filter(|&s| !s.is_empty()).enumerate();
        let mut vars: Vec<Variable> = Vec::new();
        let mut compare: FnvHashMap<&str, usize> = FnvHashMap::default();
        let mut compare_count = 1;
        let mut var_count = 0;
        while let Some((i, word)) = iter.next() {
            match (i, word) {
                (0, "fn") => {}
                (0, "pub") => {
                    if Some((1, "fn")) != iter.next() {
                        panic!("First word of fn_line is not \"fn\"");
                    }
                    if let Some((2, a)) = iter.next() {
                        fn_name.push_str(a);
                        fn_name.push_str("(");
                    }
                }
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
                                
                                if let Some(dec_type) = src_to_dectype.get(return_type) {
                                    let rep_type = dectype_to_reptype.get(dec_type).unwrap();
                                    let compare = *compare.entry(return_type).or_insert_with(||{
                                                            let without_increment = compare_count;
                                                            compare_count += 1;
                                                            without_increment
                                                        });
                                    let var = Variable::new("return", return_type, *rep_type, *dec_type,
                                                            true, compare);
                                    vars.push(var);
                                }
                            }
                            break;
                        } else {
                            panic!(String::from("No word after: ") + return_type);
                        }
                    } else {
                        panic!("No word after \"->\"");
                    }
                }
                (_, "{") => break,
                (_, var_name) => {
                    let mut var_name = var_name;
                    if var_name == "&mut" || var_name == "&" {
                        if let Some((_, var)) = iter.next() {
                            var_name = var;
                        }
                    }
                    if var_name.contains("<") {
                        while !var_name.contains(">") {
                            if let Some((_, var)) = iter.next() {
                                var_name = var;
                            }
                        }
                        continue;
                    }
                    if var_count > 0 {
                        fn_name.push_str(",");
                    }
                    var_count += 1;
                    fn_name.push_str(var_name);
                    if let Some((_, var_type)) = iter.next() {
                        if let Some(dec_type) = src_to_dectype.get(var_type) {
                            let rep_type = dectype_to_reptype.get(dec_type).unwrap();
                            let compare = *compare.entry(var_type).or_insert_with(||{
                                                    let without_increment = compare_count;
                                                    compare_count += 1;
                                                    without_increment
                                                });
                            let var = Variable::new(var_name, var_type, *rep_type, *dec_type, true, compare);
                            vars.push(var);
                        }

                    } else {
                        panic!(String::from("No word after: ") + var_name);
                    }
                }
            }
        }
        fn_name.push_str("):::");

        PPT {
            fn_name: fn_name,
            vars: vars,
            exit: 0,
        }
    }
    pub fn decls_to_string(&self, ppt_type: &PPTType) -> String {
        let mut s = String::from("ppt ") + self.fn_name.as_str();
        s = match ppt_type {
            &PPTType::Enter => s + "ENTER" + "\n\tppt-type enter",
            &PPTType::Subexit(num) => s + "EXIT" + num.to_string().as_str() +
            "\n\tppt-type subexit",
        };
        for var in &self.vars {
            match (var.name != "return", ppt_type) {
                (true, _) | (_, &PPTType::Subexit(_)) => s = s + "\n" + var.decls_to_string().as_str(),
                _ => {}
            }
        }
        s + "\n"
    }
    pub fn dtrace_to_string(&self, ppt_type: &PPTType) -> String {
        let mut s = String::from("eprintln!(\"***Daikon@Rust***") + self.fn_name.as_str();
        s = match ppt_type {
            &PPTType::Enter => s + "ENTER",
            &PPTType::Subexit(num) => s + "EXIT" + num.to_string().as_str(),
        };
        let mut tail = String::from("\\n***Daikon@Rust***\"");
        for var in &self.vars {
            match (var.name != "return", ppt_type) {
                (true, _) | (_, &PPTType::Subexit(_)) => s = s + "\\n***Daikon@Rust***" + var.dtrace_to_string(&mut tail).as_str(),
                _ => {}
            }
        }
        // println!("{}", s.clone() + tail.as_str() + ");");
        s + tail.as_str() + ");"
    }
}

struct Variable {
    name: String,
    var_kind: VarKind,
    rust_type: String,  //The type parsed from the source
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
    pub fn new(name: &str, rust_type: &str, rep_type: RepType, dec_type: DecType,
                is_param: bool, compare: usize) -> Variable {
        Variable {
            name: String::from(name),
            var_kind: VarKind::Variable,
            rust_type: String::from(rust_type),
            rep_type: rep_type,
            dec_type: dec_type,
            is_param: is_param,
            compare: compare,
        }
    }
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
        if self.name == "***Daikon@Rust***return" {
            tail.push_str("return_daikon_unique");
        } else {
            tail.push_str(self.name.as_str());
        }
        let mut s = String::from("***Daikon@Rust***");
        s = s + self.name.as_str();
        //Note: The hardcoded '1' could be replaced to change the modified flag.
        s + "\\n***Daikon@Rust***{}\\n***Daikon@Rust***1"
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