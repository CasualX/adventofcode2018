use std::{fmt, io, time};
use std::io::{Read};

fn main() {
	let stdin = io::stdin();
	let mut input = String::new();
	stdin.lock().read_to_string(&mut input).unwrap();
	let lines: Vec<&str> = input.lines().collect();

	let instant1 = time::Instant::now();
	let checksum = checksum(&lines);
	let duration1 = instant1.elapsed();
	println!("The checksum is {}. Took {:?}.", checksum, duration1);

	let instant2 = time::Instant::now();
	let matched = find_box_ids(&lines);
	let duration2 = instant2.elapsed();
	println!("The match is {}. Took {:?}.", matched, duration2);
}

fn checksum(input: &[&str]) -> i32 {
	let mut twos = 0;
	let mut threes = 0;
	for s in input {
		count(s, &mut twos, &mut threes);
	}
	twos * threes
}
fn count(string: &str, twos: &mut i32, threes: &mut i32) {
	let mut letter_freq = [0u8; 26];
	let (n2, n3) = string.bytes().fold((0, 0), |(n2, n3), chr| {
		let i = (chr - b'a') as usize;
		if let Some(count) = letter_freq.get_mut(i) {
			*count += 1;
			match *count {
				2 => (n2 + 1, n3),
				3 => (n2 - 1, n3 + 1),
				4 => (n2, n3 - 1),
				_ => (n2, n3),
			}
		}
		else {
			(n2, n3)
		}
	});
	if n2 >= 1 {
		*twos += 1;
	}
	if n3 >= 1 {
		*threes += 1;
	}
}

#[test]
fn test_checksum() {
	const INPUT: &[&str] = &[
		"abcdef",
		"bababc",
		"abbcde",
		"abcccd",
		"aabcdd",
		"abcdee",
		"ababab",
	];
	assert_eq!(12, checksum(INPUT));
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct BoxIds<'a>(&'a str, &'a str);
impl<'a> fmt::Display for BoxIds<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		assert_eq!(self.0.len(), self.1.len());
		for (chl, chr) in Iterator::zip(self.0.bytes(), self.1.bytes()) {
			if chl == chr {
				write!(f, "{}", chr as char)?;
			}
		}
		Ok(())
	}
}
fn find_box_ids<'a>(input: &[&'a str]) -> BoxIds<'a> {
	for i in 0..input.len() - 1 {
		for j in i + 1..input.len() {
			if eq_box_ids(input[i], input[j]) {
				return BoxIds(input[i], input[j]);
			}
		}
	}
	panic!("No match found.");
}
fn eq_box_ids(a: &str, b: &str) -> bool {
	// Box ids must compare equal in length
	assert_eq!(a.len(), b.len());

	// Count the number of different characters
	let mut ndiff = 0;
	for (a, b) in Iterator::zip(a.bytes(), b.bytes()) {
		if a != b {
			if ndiff >= 1 {
				return false;
			}
			ndiff += 1;
		}
	}
	ndiff == 1
}

#[test]
fn test_find_match() {
	const INPUT: &[&str] = &[
		"abcde",
		"fghij",
		"klmno",
		"pqrst",
		"fguij",
		"axcye",
		"wvxyz",
	];
	assert_eq!("fgij", find_box_ids(INPUT).to_string());
}
