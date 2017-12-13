//  ***Cannot be changed, must match daikon spec
//  For .decls files
//  var-kind <kind> [<relative-name>]
//  Specifies the variable kind. Possible values are: field, function,
//  array, variable, return. If field or function are specified, the
//  relative name of the field or function must be specified. For
//  example, if the variable is this.theArray, the relative name is
//  theArray. Pointers to arrays are of type field. The arrays
//  themselves (a sequence of values) are of type array. A var-kind
//  entry is required in each variable block.
pub enum VarKind {
    Array,
    Field,
    Function,
    Return,
    Variable,
}
impl VarKind {
    pub fn as_str(&self) -> &str {
        match self {
            &VarKind::Array => "array",
            &VarKind::Field => "field",
            &VarKind::Function => "function",
            &VarKind::Return => "return",
            &VarKind::Variable => "variable",
        }
    }
}

//  ***Cannot be changed, must match daikon spec
//  For .decls files
//  rep-type <daikon-type>
//  This describes what will appear in the data trace file. For instance,
//   the declared type might be char[..] but the representation type
//  might be java.lang.String. Or, the declared type might be Object
//  but the representation type might be hashcode, if the address of
//  the object is written to the data trace file. A rep-type entry is
//  required in each variable block.

//  The representation type should be one of boolean, int, hashcode,
//  double, or java.lang.String; or an array of one of those (indicated
//  by a [..] suffix).

//  hashcode is intended for unique object identifiers like memory
//  addresses (pointers) or the return value of Java’s Object.hashCode
//  method. hashcode is treated like int, except that the hashcode
//  values are considered uninteresting for the purposes of output. For
//  example, Daikon will print ‘var has only one value’ instead of
//  ‘var == 0x38E8A’.
pub enum RepType {
    Boolean,
    Double,
    Hashcode,
    Int,
    JavaLangString,
}
impl RepType {
    pub fn as_str(&self) -> &str {
        match self {
            &RepType::Boolean => "boolean",
            &RepType::Double => "double",
            &RepType::Hashcode => "hashcode",
            &RepType::Int => "int",
            &RepType::JavaLangString => "java.lang.String",
        }
    }
}

//  ***Note that these are the currently supported types. This
//      can be expanded. All primitives are Java types.

//  dec-type <language-declaration>
//  This is what the programmer used in the declaration of the variable.
//  Names for standard types should use Java’s names (e.g., int,
//  boolean, java.lang.String, etc.), but names for user-defined or
//  language-specific types can be arbitrary strings. A dec-type entry
//  is required in each variable block.
pub enum DecType {
    Boolean,
    Char,   //Unstable! Rust uses a 4 byte unicode scalar value
            //  however, Java uses 2 byte unicode characters.
            //  Rust could emit a character in a trace file that
            //  Java could not understand.
    Byte,   //i8
    Short,  //u8, i16
    Int,    //u16, i32
    Long,   //u32, i64, isize, u64 [Unstable!], usize [Unstable!]
            //  Java's largest integer is 64-signed, where Rust's
            //  is 64-unsigned. Rust could emit an interger in a
            //  trace file to large for Java. Note that usize and
            //  isize are system pointer size.
    Float,  //f32
    Double, //f64
    JavaLangString, //String
}
impl DecType {
    pub fn as_str(&self) -> &str {
        match self {
            &DecType::Boolean => "boolean",
            &DecType::Char => "char",
            &DecType::Byte => "byte",
            &DecType::Short => "short",
            &DecType::Int => "int",
            &DecType::Long => "long",
            &DecType::Float => "float",
            &DecType::Double => "double",
            &DecType::JavaLangString => "java.lang.String",
        }
    }
}
