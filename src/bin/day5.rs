use std::{io, time};
use std::io::Read;

fn main() {
	let stdin = io::stdin();
	let mut input = String::new();
	stdin.lock().read_to_string(&mut input).unwrap();

	let instant1 = time::Instant::now();
	let len1 = react_len(input.as_bytes());
	let duration1 = instant1.elapsed();
	println!("Polymer length after reaction is {}. Took {:?}.", len1, duration1);

	let instant2 = time::Instant::now();
	let len2 = optimize_len(input.as_bytes());
	let duration2 = instant2.elapsed();
	println!("Optimized polymer length is {}. Took {:?}.", len2, duration2);
}

fn react_len(polymer: &[u8]) -> usize {
	let mut polymer = polymer.to_vec();
	let mut i = 0;
	while i + 1 < polymer.len() {
		if (polymer[i] as i32 - polymer[i + 1] as i32).abs() == (b'a' - b'A') as i32 {
			polymer.remove(i);
			polymer.remove(i);
			if i >= 1 {
				i -= 1;
			}
		}
		else {
			i += 1;
		}
	}
	polymer.len()
}

fn filter(polymer: &[u8], chr1: u8, chr2: u8) -> Vec<u8> {
	polymer
		.into_iter()
		.cloned()
		.filter(|&chr| chr != chr1 && chr != chr2)
		.collect()
}
fn optimize_len(polymer: &[u8]) -> usize {
	Iterator::zip(b'A'..=b'Z', b'a'..=b'z')
		.map(|(chr_u, chr_l)| filter(polymer, chr_u, chr_l))
		.map(|test| react_len(&test))
		.min()
		.unwrap()
}

#[test]
fn test_react_len() {
	assert_eq!(10, react_len(b"dabAcCaCBAcCcaDA"));
}
#[test]
fn test_optimize() {
	assert_eq!(4, optimize_len(b"dabAcCaCBAcCcaDA"));
}
