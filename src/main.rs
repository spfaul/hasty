use std::fs;
use std::time::SystemTime;
use std::collections::HashMap;
use std::process::Command;
use clap::{Arg, App};
use shellwords;

fn main() -> std::io::Result<()> {
	let cli = App::new("Hasty")
					.version("0.1.0")
					.author("Author: t0a5ted")
					.arg(Arg::with_name("watch_folder")
						.help("Relative or absolute path of folder to watch")
						.required(true)
						.takes_value(true)
						.value_name("FOLDER"))
					.arg(Arg::with_name("command")
						.help("Shell command to be run every reload")
						.short("c")
						.required(true)
						.takes_value(true)
						.value_name("COMMAND"))
					.get_matches();

	let folder_path: &str = cli.value_of("watch_folder").unwrap();
	let mut command: Command = str_to_command(cli.value_of("command").unwrap());
	
	watch(folder_path, &mut command);

	Ok(())
}

fn str_to_command(text: &str) -> Command {
	let command_tokens = shellwords::split(text).unwrap();
	let mut reload_command = Command::new(&command_tokens[0]);
	let mut temp = command_tokens.clone();
	temp.remove(0);
	reload_command.args(temp);

	reload_command
}

#[allow(unused_variables, dead_code)]
fn watch(folder_path: &str, reload_command: &mut Command) {
	let mut path_to_modified_map: HashMap<String, u64> = HashMap::new();

	// populate hashmap
	for (file_path, last_mod) in get_folder_modified(folder_path).unwrap() {
		path_to_modified_map.insert(file_path, last_mod);
	}

	loop {
		if let Ok(folder_info) = get_folder_modified(folder_path) {
			for (file_path, last_mod) in folder_info {
				if !path_to_modified_map.contains_key(&file_path) {
					// new file created
					println!("[INFO] {} created", file_path);
					path_to_modified_map.insert(file_path, last_mod);
					(*reload_command).spawn().expect("Command failed to start!");
				} else if last_mod > *path_to_modified_map.get(&file_path).unwrap() {
					// existing file updated
					println!("[INFO] {} changed", file_path);
					path_to_modified_map.insert(file_path, last_mod);
					(*reload_command).spawn().expect("Command failed to start!");
				}
			}
		} else {
			println!("[ERROR] Recursive File metadata fetch failed!");
		}
	}
}

fn get_folder_modified(folder_path: &str) -> std::io::Result<Vec<(String, u64)>> {
	let mut v: Vec<(String, u64)> = Vec::new();

	match fs::read_dir(folder_path) {
		Ok(contents) => {
			for file in contents {
				let file = file?;
				let file_path: String = file.path().display().to_string();

				// println!("{}", file_path);
				
				if file.file_type()?.is_dir() {
					let mut subfolder_v: Vec<(String, u64)> = get_folder_modified(file_path.as_str())?;
					v.append(&mut subfolder_v);
				} else {
					let last_mod: u64 = get_last_modified(file_path.as_str())?;
					v.push((file_path, last_mod));
				}
			}
		},
		Err(_) => panic!("Provided path doesn't exist, or process lacks perms to view contents, or path is not a directory")
	}
	
	Ok(v)
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