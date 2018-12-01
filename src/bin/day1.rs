use std::{io, time};
use std::io::BufRead;

fn main() {
	let stdin = io::stdin();
	let input: Vec<i32> = stdin.lock()
		.lines()
		.map(|line| line.unwrap().parse().unwrap())
		.collect();

	let sum = sum_freq(&input);
	println!("The sum of frequencies is {}.", sum);

	let instant2 = time::Instant::now();
	let repeat_freq = repeat_freq(&input);
	let duration2 = instant2.elapsed();
	println!("Frequency first seen twice is {}. Took {:?}.", repeat_freq, duration2);
}

fn sum_freq(input: &[i32]) -> i32 {
	input.iter().sum()
}

#[test]
fn test_sum_freq() {
	assert_eq!(3, sum_freq(&[1, 1, 1]));
	assert_eq!(0, sum_freq(&[1, 1, -2]));
	assert_eq!(-6, sum_freq(&[-1, -2, -3]));
}

fn repeat_freq(input: &[i32]) -> i32 {
	let mut seen_freq = vec![0; 1 << 29];
	// let mut seen_freq = Vec::with_capacity(1 << 29);
	// unsafe { seen_freq.set_len(1 << 29) };
	fn insert(cache: &mut [u8], freq: i32) -> bool {
		let i = freq as u32 / 8;
		let mask = 1 << (freq as u32 % 8);
		let bit = cache[i as usize] & mask != 0;
		cache[i as usize] |= mask;
		bit
	}
	let mut current_freq = 0;
	for &change in input.iter().cycle() {
		if insert(&mut seen_freq, current_freq) {
			return current_freq;
		}
		current_freq += change;
	}
	unreachable!()
}

#[test]
fn test_repeat_freq() {
	assert_eq!(0, repeat_freq(&[1, -1]));
	assert_eq!(10, repeat_freq(&[3, 3, 4, -2, -4]));
	assert_eq!(5, repeat_freq(&[-6, 3, 8, 5, -6]));
	assert_eq!(14, repeat_freq(&[7, 7, -2, -7, -4]));
}
