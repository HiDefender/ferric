use std::env;
use std::fs;
use std::io;
use std::process::Command;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::Read;

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

pub fn read_cur_src() -> Result<Vec<String>, io::Error> {
    let src_path_buf = env::current_dir()?.push("src");
    let src_path = src_path_buf.as_path();
    println!("{:?}", src_path.path());

    let files = Vec<String>::new();
    Ok(files)
}

fn open_and_read_file(path: &String) -> String {
	// Create path to the desired file.
	let path = Path::new(&path);
	
	// Open the file
	let mut file = match File::open(&path) {
		// The 'description' method of 'io::Error' returns a string that
		// describes the error.
		Err(why) => {
                        panic!("Couldn't open {}: {}", path.display(), why.description());
		}
		Ok(file) => file,
	};
    let mut buf = String::new();
    file.read_to_string(&mut buf);
    buf
}

#[cfg(test)]
mod tests {
	use super::open_and_read_file;

	#[test]
	fn open_and_read_file_pass() {
		let s = open_and_read_file(&String::from("test/file/open_and_read_file.txt"));
        assert_eq!("Hello World!\n", s.as_str());
	}

	#[test]
	#[should_panic]
	fn open_and_read_file_panic() {
		open_and_read_file(&String::from("fail"));
	}

}
