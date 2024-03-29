use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write, BufReader, BufRead};

enum MODE {
	ASCII,
	AsciiWithCyrillic,
	UNICODE,
}

fn gen_token() -> String {
	let uppercase_letters: [char; 26] = [
		'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
	];
	let lowercase_letters: [char; 26] = [
		'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
	];
	let digits: [char; 10] = [
		'0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
	];

	let mut rng = thread_rng();
	let random_uppercase1 = uppercase_letters.choose(&mut rng).unwrap();
	let random_lowercase1 = lowercase_letters.choose(&mut rng).unwrap();
	let random_digit1 = digits.choose(&mut rng).unwrap();
	let random_lowercase2 = lowercase_letters.choose(&mut rng).unwrap();
	let random_uppercase2 = uppercase_letters.choose(&mut rng).unwrap();
	let random_digit2 = digits.choose(&mut rng).unwrap();
	let random_uppercase3 = uppercase_letters.choose(&mut rng).unwrap();
	let random_lowercase3 = lowercase_letters.choose(&mut rng).unwrap();
	let random_digit3 = digits.choose(&mut rng).unwrap();
	let random_lowercase4 = lowercase_letters.choose(&mut rng).unwrap();
	
	String::from(format!("{}{}{}{}{}{}{}{}{}{}", 
		random_uppercase1, 
		random_lowercase1, 
		random_digit1, 
		random_lowercase2, 
		random_uppercase2,
		random_digit2,
		random_uppercase3,
		random_lowercase3,
		random_digit3,
		random_lowercase4))
}
// fn gen_chars(mode: MODE) -> Vec<char> {
	// match mode {
		// MODE::ASCII => {
			// (0..=127 as u32).filter_map(std::char::from_u32).collect()
		// }
		// MODE::AsciiWithCyrillic => {
			// // for c in 0..=127 { characters.push(c as u8 as char); }
			// // for c in 1040..=1103 { characters.push(c as u8 as char); }
			// // for c in 1024..=1279 { characters.push(c as u16 as char); }
			// (
					// (0..=127 as u32).filter_map(std::char::from_u32).collect().iter()
				// .chain(
					// (1040..=1103 as u32).filter_map(std::char::from_u32).collect().iter()
				// )
				// .chain(
					// (1024..=1279 as u32).filter_map(std::char::from_u32).collect().iter()
				// )
			// ).cloned().collect()
		// }
		// MODE::UNICODE => {
			// (0..=std::char::MAX as u32).filter_map(std::char::from_u32).collect()
		// }
	// }
// }

fn gen_chars(mode: MODE) -> io::Result<Vec<char>> {
    match mode {
        MODE::ASCII => {
            Ok((0..=127 as u32).filter_map(std::char::from_u32).collect::<Vec<_>>())
        }
        MODE::AsciiWithCyrillic => {
            Ok((0..=127 as u32)
                .filter_map(std::char::from_u32)
                .chain((1040..=1103 as u32).filter_map(std::char::from_u32))
                .chain((1024..=1279 as u32).filter_map(std::char::from_u32))
                .collect::<Vec<_>>())
        }
        MODE::UNICODE => {
            Ok((0..=std::char::MAX as u32).filter_map(std::char::from_u32).collect::<Vec<_>>())
        }
    }
}

fn write_dict(dictionary_filename: &str) -> io::Result<()> {
	let mut file = File::create(dictionary_filename)?;
	let chars = gen_chars(MODE::ASCII).unwrap();
	let chars_length = chars.len();
	let show_percentage = true;
	let show_raw_percentage = false;
	let mut written = 0;
	let mut percentage = 0.0;
	let mut percentage_new = 0.0;
	
	for chr in chars {
		writeln!(file, "{} e-> {:?}", gen_token(), chr);
		written += 1;
		percentage_new = (written as f64 / chars_length as f64) * 100.00;
		if percentage != percentage_new {
			percentage = percentage_new;
			if show_percentage {
				println!("Progress: {:.2}%", percentage);
			} else if show_raw_percentage {
				println!("Progress: {:?}%", percentage);
			}
		}
	}
	Ok(())
}

fn read_dict(dictionary_filename: &str) -> io::Result<HashMap<String, String>> {
	let file = File::open(dictionary_filename)?;
	let reader = BufReader::new(file);
	let mut dict: HashMap<String, String> = HashMap::new();
	let mut lines = vec![];
	
	for line in reader.lines() { lines.push(line?.to_string()); }
	
	for line in &lines {
		let line_split: Vec<_> = line.split(" e-> ").collect();
		let (k, v) = (line_split[1], line_split[0]);
		//println!("k: {}, v: {}", k, v);
		dict.insert(k.to_string(), v.to_string());
	}
	Ok(dict)
}

fn file_to_chars(filepath: &str) -> io::Result<Vec<String>> {
	let file = File::open(filepath)?;
	let reader = BufReader::new(file);
	let mut lines = vec![];
	let mut chars = vec![];
	for line in reader.lines() { lines.push(line?.to_string()); }
	for line in lines {
		for c in line.chars() {
			chars.push(c.to_string());
		}
	}
	Ok(chars)
}

fn encode(filepath: &str, dictionary_filename: &str) -> io::Result<()> {
	let dict = read_dict(dictionary_filename).unwrap();
	let file = file_to_chars(filepath).unwrap();
	let mut result = String::from("");
	println!("{:?}", dict);
	for chr in file {
		println!("{}", chr);
		match chr {
			Some(v) => {
				result.push_str(
					dict.get( &v )
				);
			}
			None => {
				println!("failed to find token by symbol");
			}
		}
	}
	let output_file = File::create(format!("{}.cdd", filepath));
	writeln!(output_file?, "{}", result);
	Ok(())
}

fn main() -> io::Result<()> {
	write_dict("dictionary");
	encode("example.txt", "dictionary");
	//println!("{:?}", read_dict("dictionary").unwrap());
	//read_dict("dictionary").unwrap();
	Ok(())
}