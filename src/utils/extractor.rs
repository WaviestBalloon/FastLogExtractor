use std::{char, fs::{self, File}, io::{self, BufRead}, path::PathBuf, time};

#[derive(Debug)]
pub struct FastLog { // could possibly add more info about the log here, e.g. if it has `%s`
	pub value: String,
}
#[derive(Debug)]
pub struct ExtractionDetails {
	pub bytes_read: usize,
	pub bytes_in_file: usize,
	pub time_taken: time::Duration,
}
#[derive(Debug)]
pub struct FastLogs {
	pub flogs: Vec<FastLog>,
	pub dflogs: Vec<FastLog>,
	pub extraction_details: ExtractionDetails,
}

fn calculate_percentage(current: f64, total: f64) -> f64 {
	((current / total) * 100.0).round()
}

fn convert_bytes_to_ascii(bytes: Vec<u8>) -> String {
	let mut ascii = String::new();

	for byte in bytes.iter() {
		let character = *byte as char;

		if character.is_ascii_alphanumeric() || character.is_ascii_punctuation() || character.is_ascii_whitespace() || character.is_ascii_control() {
			ascii.push(character);
		}
	}

	ascii
}

pub fn inspect_executable(executable_path: PathBuf) -> FastLogs {
	let file = File::open(executable_path.clone()).unwrap();
	let bytes_in_file = fs::metadata(executable_path).unwrap().len();
	let mut reader = io::BufReader::new(file);

	println!("Processing bytes to find FastLogs...");
	let start_timer = time::Instant::now();
	let mut flogs = Vec::new();
	let mut dflogs = Vec::new();

	let mut bytes_processing_buffer: Vec<u8> = Vec::new();
	let mut bytes_read = 0;
	loop {
		reader.read_until(0x0, &mut bytes_processing_buffer).unwrap();

		let bytes_to_ascii = convert_bytes_to_ascii(bytes_processing_buffer.clone());
		if bytes_to_ascii.len() > 0 {
			let current_byte_position = bytes_read + 1;

			if bytes_to_ascii.split("[FLog").count() > 1 {
				println!("Found FLog starting at byte: {} ({}% read)", current_byte_position, calculate_percentage(current_byte_position as f64, bytes_in_file as f64));
				flogs.push(FastLog {
					value: bytes_to_ascii,
				});
			} else if bytes_to_ascii.split("[DFLog").count() > 1 {
				println!("Found DFLog starting at byte: {} ({}% read)", current_byte_position, calculate_percentage(current_byte_position as f64, bytes_in_file as f64));
				dflogs.push(FastLog {
					value: bytes_to_ascii,
				});
			}
		}

		#[cfg(debug_assertions)] println!("{} bytes of {} bytes ({}% read)", bytes_read, bytes_in_file, calculate_percentage(bytes_read as f64, bytes_in_file as f64));

		bytes_read += bytes_processing_buffer.len();
		bytes_processing_buffer = Vec::new();
		if bytes_read >= bytes_in_file as usize {
			break;
		}
	}

	FastLogs {
		flogs,
		dflogs,
		extraction_details: ExtractionDetails {
			bytes_read,
			bytes_in_file: bytes_in_file as usize,
			time_taken: start_timer.elapsed(),
		},
	}
}
