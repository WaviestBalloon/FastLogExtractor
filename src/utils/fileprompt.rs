use std::{path::PathBuf, process};
use native_dialog::{FileDialog, MessageDialog, MessageType};

pub fn prompt_to_open(path: PathBuf, args: Vec<String>) -> PathBuf {
	let path = FileDialog::new()
		.set_location(&path)
		.add_filter("Executables", &["exe"])
		.reset_filename()
		.show_open_single_file()
		.unwrap_or_default(); // Unsure about compatability on this part

	match path {
		Some(path) => path,
		None => {
			MessageDialog::new()
				.set_type(MessageType::Error)
				.set_title("Error")
				.set_text("No file was given. Cannot extract FastFlags from thin air.")
				.show_alert()
				.unwrap();

			eprintln!("No file was given.\nUsage: {} <file>", args.get(0).unwrap());
			process::exit(1);
		},
	}
}