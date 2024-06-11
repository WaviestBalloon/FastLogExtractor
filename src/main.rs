use std::{env, fs, path::PathBuf, process};
mod utils;
use utils::{fileprompt, extractor};

fn fix_windows_canonicalized_path(path: PathBuf) -> PathBuf { // Will not work for UNC, I hate `GetFinalPathNameByHandle`
	path.to_str().unwrap().replace("\\\\?\\", "").to_string().into()
}

fn main() {
	let current_dir = env::current_dir().unwrap();
	let path = if cfg!(windows) {
		fix_windows_canonicalized_path(current_dir.canonicalize().unwrap()) // Billions must .unwrap() - `?` never existed
	} else {
		current_dir.canonicalize().unwrap()
	};

	let args: Vec<String> = env::args().collect::<Vec<String>>();
	let mut file: Option<PathBuf> = if cfg!(windows) {
		args.get(1).map(|f| fix_windows_canonicalized_path(PathBuf::from(f)))
	} else {
		args.get(1).map(|f| PathBuf::from(f))
	};

	if args.len() != 2 || file.is_none() {
		// Let's try opening a open dialog prompt for them
		println!("Too few arguments, assuming no file was given!");
		println!("Opening a file dialog prompt for you to select a executable...");
		
		let new_location = fileprompt::prompt_to_open(path, args);
		println!("Got file: {:?}", new_location);
		file = Some(new_location);
	}
	let fileunwrapped = file.unwrap();
	if !fileunwrapped.is_file() {
		eprintln!("{} is not a file, cannot continue.", fileunwrapped.display());
		process::exit(1);
	}

	println!("Reading executable: {:?}", fileunwrapped);
	let flogs: extractor::FastLogs = extractor::inspect_executable(fileunwrapped);

	println!("Took: {:?}\nBytes read: {}\nFLogs found: {}\nDFLogs found: {}", flogs.extraction_details.time_taken, flogs.extraction_details.bytes_read, flogs.flogs.len(), flogs.dflogs.len());

	println!("\nWriting results to output.txt...");
	let mut stringflogs = String::new();
	let mut stringdflogs = String::new();
	for flog in flogs.flogs.iter() {
		stringflogs = format!("{}\n{}", stringflogs, flog.value);
	}
	for dflog in flogs.dflogs.iter() {
		stringdflogs = format!("{}\n{}", stringdflogs, dflog.value);
	}

	fs::write("output.txt", format!("--- INFO START ---\nFLogs found: {}\nDFLogs found: {}\nTime taken: {:?}secs\n--- INFO END ---\n--- FLOGS START ---{}\n--- FLOGS END ---\n--- DFLOGS START ---{}\n--- DFLOGS END ---",
		flogs.flogs.len(),
		flogs.dflogs.len(),
		flogs.extraction_details.time_taken.as_secs(),
		stringflogs,
		stringdflogs
	)).unwrap();
}
