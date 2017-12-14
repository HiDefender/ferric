use daikon::types::{DecType, RepType};

use fnv::FnvHashMap;

pub fn map_src_to_dectype() -> FnvHashMap<&'static str, DecType> {
    let mut map = FnvHashMap::with_capacity_and_hasher(21, Default::default());
    map.insert("bool", DecType::Boolean);
    map.insert("char", DecType::Char);
    map.insert("i8", DecType::Byte);
    map.insert("u8", DecType::Short);
    map.insert("i16", DecType::Short);
    map.insert("u16", DecType::Int);
    map.insert("i32", DecType::Int);
    map.insert("u32", DecType::Long);
    map.insert("i64", DecType::Long);
    map.insert("isize", DecType::Long);
    map.insert("u64", DecType::Long);
    map.insert("usize", DecType::Long);
    map.insert("f32", DecType::Float);
    map.insert("f64", DecType::Double);
    map.insert("u32", DecType::Long);
    map.insert("String", DecType::JavaLangString);
    map.insert("str", DecType::JavaLangString);
    map.insert("OsString", DecType::JavaLangString);
    map.insert("OsStr", DecType::JavaLangString);
    map.insert("PathBuf", DecType::JavaLangString);
    map.insert("Path", DecType::JavaLangString);
    map
}

pub fn map_dectype_to_reptype() -> FnvHashMap<DecType, RepType> {
    let mut map = FnvHashMap::with_capacity_and_hasher(9, Default::default());
    map.insert(DecType::Boolean, RepType::Boolean);
    map.insert(DecType::Char, RepType::JavaLangString);
    map.insert(DecType::Byte, RepType::Int);
    map.insert(DecType::Short, RepType::Int);
    map.insert(DecType::Int, RepType::Int);
    map.insert(DecType::Long, RepType::Int);
    map.insert(DecType::Float, RepType::Double);
    map.insert(DecType::Double, RepType::Double);
    map.insert(DecType::JavaLangString, RepType::JavaLangString);
    map
}