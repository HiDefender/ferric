use std::env;
use std::fs;
use std::io;
use std::process::Command;

pub fn check_or_create_ferric_folder() -> Result<bool, io::Error> {
    let cur_path_buf = env::current_dir()?;
    let cur_dir = cur_path_buf.as_path();
    if cur_dir.is_dir() {
        if fs::read_dir(cur_dir)?.any(|x| {
                let x = x.unwrap();
                x.file_type().unwrap().is_dir() && x.file_name().to_str().unwrap() == "ferric"
        }) {
            return Ok(true)
        } else {
        Command::new("cargo")
            .args(&["new", "ferric"])
            .output()
            .expect("\"cargo new ferric\" failed.");
        }
    }
    Ok(false)
}

pub fn ferric_clean() -> io::Result<()> {
    let cur_path_buf = env::current_dir()?;
    let cur_dir = cur_path_buf.as_path();
    if cur_dir.is_dir() {
        let del_folder = fs::read_dir(cur_dir)?.find(|x| {
                let x = x.as_ref().unwrap();
                x.file_type().unwrap().is_dir() && x.file_name().to_str().unwrap() == "ferric"
        }).unwrap()?;
        fs::remove_dir_all(del_folder.path())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
