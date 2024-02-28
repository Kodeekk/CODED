use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};

fn gen_token() -> String {
    let uppercase_letters: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
    ];

    let lowercase_letters: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
    ];

    let digits: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

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
    /*
    for chr in unicode_chars {
    	println!("{:?}", chr);
    }
    */
}

//fn main() {
//	let dict = HashMap::new();
//	for chr in gen_dict() {
//		dict.insert(chr, gen_token());
//	}
//}


fn main() -> io::Result<()> {
	let mut file = File::create("dictionary")?;
    let chars = gen_chars();
    let chars_length = chars.len();
    let mut written = 0;
    let mut percentage = 0.0;
    let mut percentage_new = 0.0;
    for chr in chars {
        writeln!(file, "{} e-> {}", gen_token(), chr)?;
        written += 1;
        percentage_new = (written as f64 / chars_length as f64) * 100.00;
        if percentage != percentage_new {
        	percentage = percentage_new;
        	println!("Progress: {:.2}%", percentage);
        }
    }
    Ok(())
}