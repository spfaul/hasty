use std::fs;
use std::time::SystemTime;

fn main() -> std::io::Result<()> {
	// watch("./dummy_dir"); // TBD

	print_file_struct("./dummy_dir/".to_string())?;

	get_last_modified("dummy_dir/file.rs")?;

	Ok(())
}

fn print_file_struct(folder_path: String) -> std::io::Result<()> {
	match fs::read_dir(folder_path) {
		Ok(contents) => {
			for file in contents {
				let file = file?;
				println!("{}", file.path().display());
				if file.file_type()?.is_dir() {
					print_file_struct(file.path().display().to_string())?;
				}
			}
		},
		Err(_) => panic!("Provided path doesn't exist, or process lacks perms to view contents, or path is not a directory")
	}
	Ok(())
}


/// Gets the last modified time in secs since UNIX epoch of a file  
fn get_last_modified(file_name: &str) -> std::io::Result<u64> {
	let metadata = fs::metadata(file_name)?;
	match metadata.modified() {
		Ok(time) => {
			match time.duration_since(SystemTime::UNIX_EPOCH) {
				Ok(n) => Ok(n.as_secs()),
				Err(_) => panic!("SystemTime before UNIX EPOCH???")
			}
		},
		Err(_) => panic!("Modified metadata not supported on this platform!")
	}	
}