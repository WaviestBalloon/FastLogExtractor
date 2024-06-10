use std::{char, fs::File, io::{self, Read}, path::PathBuf}; // Todo, multithreading

#[derive(Debug)]
pub struct FastLog { // could possibly add more info about the log here, e.g. if it has `%s`
	pub value: String,
}
#[derive(Debug)]
pub struct FastLogs {
	pub flogs: Vec<FastLog>,
	pub dflogs: Vec<FastLog>,
}

fn calculate_percentage(current: f64, total: f64) -> f64 {
	((current / total) * 100.0).round()
}

fn find_terminator_position(ascii: Vec<char>, current_index: usize) -> i32 {
	#[cfg(debug_assertions)] let start_timer = std::time::Instant::now();

	for (index, char) in ascii[current_index..ascii.len()].iter().enumerate() {
		#[cfg(debug_assertions)] println!("Shifting to find terminator, current index at {}, char is {} (0x{})", index, char, *char as u8);
		if *char as u8 == 0x0 { // is_ascii_control failed me
			#[cfg(debug_assertions)] println!("Took {:?} to find terminator", start_timer.elapsed());
			return (current_index + index) as i32;
		}
	}

	0
}

pub fn inspect_executable(executable_path: PathBuf) -> FastLogs {
	let file = File::open(&executable_path).unwrap();
	let mut reader = io::BufReader::new(file);

	let mut buffer = Vec::new();
	reader.read_to_end(&mut buffer).unwrap();

	println!("Parsing bytes...");

	let mut ascii: Vec<char> = Vec::new();
	for byte in buffer.iter() {
		let character = *byte as char;

		if character.is_ascii_alphanumeric() || character.is_ascii_punctuation() || character.is_ascii_whitespace() {
			ascii.push(character);
		} else if character.is_ascii_control() { // For endings
			ascii.push(character);
		}
	}

	println!("Processing bytes to find FastLogs...");

	let mut flogs = Vec::new();
	let mut dflogs = Vec::new();

	for i in 0..ascii.len() - 7 { // miss out 7 bytes at the end since we don't want to go out of range and we don't want to constantly check if we are going out of range
		if ascii[i..i + 7].iter().collect::<String>() == "[FLog::" {
			println!("Found FLog starting at byte: {} ({}% read)", i, calculate_percentage(i as f64, ascii.len() as f64));
			let shift_to = find_terminator_position(ascii.clone(), i);

			flogs.push(FastLog {
				value: ascii[i..shift_to as usize].iter().collect::<String>(),
			});
			continue;
		}
		if ascii[i..i + 8].iter().collect::<String>() == "[DFLog::" {
			println!("Found DFLog starting at byte: {} ({}% read)", i, calculate_percentage(i as f64, ascii.len() as f64));
			let shift_to = find_terminator_position(ascii.clone(), i);

			dflogs.push(FastLog {
				value: ascii[i..shift_to as usize].iter().collect::<String>(),
			});
			continue;
		}
	}

	println!("Bytes processed: {}", ascii.len());
	
	FastLogs {
		flogs,
		dflogs,
	}
}
