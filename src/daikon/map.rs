use daikon::types::{DecType, RepType};

use fnv::FnvHashMap;

pub fn map_src_to_dectype() -> FnvHashMap<&'static str, DecType> {
    let mut map = FnvHashMap::with_capacity_and_hasher(19, Default::default());
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
    map
}