use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write, BufReader, BufRead};

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

fn gen_chars() -> Vec<char> {
	(0..=std::char::MAX as u32).filter_map(std::char::from_u32).collect()

fn write_dict() -> io::Result<()> {
	let mut file = File::create("dictionary")?;
	let chars = gen_chars();
	let chars_length = chars.len();
	let show_percentage = true;
	let show_raw_percentage = false;
	let mut written = 0;
	let mut percentage = 0.0;
	let mut percentage_new = 0.0;
	
	writeln!(file, "!!¡->¿>u_specials>>")?;
	
	for chr in chars {
		writeln!(file, "{} e-> {:?}", gen_token(), format!(r"{}", chr));
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

fn read_dict() -> io::Result<HashMap<String, HashMap<String, String>>> {
	let file = File::open("dictionary")?;
	let reader = BufReader::new(file);
	let mut dict: HashMap<String, HashMap<String, String>> = HashMap::new();
	let mut lines = vec![];
	let mut section = "";
	for line in reader.lines() {
		lines.push(line?.to_string());
	}
	for line in lines {
		let mut line_split: Vec<_> = line.split(" e-> ").collect();
		let (k, v) = (line_split[0], line_split[1]);
		if line.starts_with("!!¡->¿>") {
			if let Some(string) = line.strip_prefix("!!¡->¿>") {
				section = string;
			}
		}
		dict.get(section).insert(k.to_string(), v.to_string());
	}
	Ok(map)
}
fn main() {
	//write_dict();
	println!("{:?}", read_dict().unwrap());
}