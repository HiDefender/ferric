use std::env;
use std::fs;
use std::io;
use std::process::Command;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Read, Write, BufWriter, BufReader};
use std::ffi::{OsStr, OsString};
use std::collections::HashMap;

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

pub fn read_cur_src() -> Result<HashMap<PathBuf, String>, io::Error> {
    let mut src_path_buf = env::current_dir()?;
    src_path_buf.push("src");
    let mut files = HashMap::new();
    if src_path_buf.is_dir() {
        visit_dirs(&src_path_buf, &mut files);
    }
    Ok(files)
}

fn visit_dirs(dir: &Path, files: &mut HashMap<PathBuf, String>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, files)?;
            } else if path.as_path().extension().unwrap_or(OsStr::new("fail")) == "rs" { //if file is .rs

                let file = open_and_read_file(&path.as_path());
                let folder_path = env::current_dir()?;
                let mut path = path.clone();
                let mut top_of_path: Vec<OsString> = Vec::new();
                while path != folder_path {
                    top_of_path.push(path.file_name().unwrap().to_os_string());
                    path.pop();
                }
                path.push("ferric");
                for x in top_of_path.iter().rev() {
                    path.push(x);
                }
                files.insert(path, file);
            }
        }
    }
    Ok(())
}

fn open_and_read_file(path: &Path) -> String {
	// Open the file
	let file = match File::open(&path) {
		// The 'description' method of 'io::Error' returns a string that
		// describes the error.
		Err(why) => {
                        panic!("Couldn't open {}: {}", path.display(), why.description());
		}
		Ok(file) => file,
	};
    let mut file = BufReader::new(file);
    let mut buf = String::new();
    file.read_to_string(&mut buf);
    buf
}

pub fn create_and_write_files(files: &HashMap<PathBuf, String>) -> io::Result<()> {
    //Taken from https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x
    for (path, file) in files {
        let mut folder_path = path.clone();
        folder_path.pop();
        if !folder_path.is_dir() {
            fs::create_dir_all(folder_path)?;
        }
        let f = File::create(path).expect("Unable to create file");
        let mut f = BufWriter::new(f);
        f.write_all(file.as_bytes()).expect("Unable to write data");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
	use super::open_and_read_file;
    use std::path::PathBuf;

	#[test]
	fn open_and_read_file_pass() {
		let s = open_and_read_file(&PathBuf::from("test/file/open_and_read_file.txt"));
        assert_eq!("Hello World!\n", s.as_str());
	}

	#[test]
	#[should_panic]
	fn open_and_read_file_panic() {
		open_and_read_file(&PathBuf::from("fail"));
	}

}
