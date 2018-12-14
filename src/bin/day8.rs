use std::{io, time};
use std::io::Read;

fn main() {
	let stdin = io::stdin();
	let mut input = String::new();
	stdin.lock().read_to_string(&mut input).unwrap();
	let data: Vec<u8> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();

	let instant1 = time::Instant::now();
	let msum = metadata_sum(&data);
	let duration1 = instant1.elapsed();
	println!("The metadata sum is {}. Took {:?}.", msum, duration1);

	let instant2 = time::Instant::now();
	let value = root_value(&data);
	let duration2 = instant2.elapsed();
	println!("The root value is {}. Took {:?}.", value, duration2);
}

fn metadata_sum(data: &[u8]) -> i32 {
	fn walk_tree(data: &[u8], sum: &mut i32) -> usize {
		let child_count = data[0] as usize;

		let mut tail = &data[2..];
		let mut total_size = 2;
		for _i in 0..child_count {
			let size = walk_tree(tail, sum);
			total_size += size;
			tail = &tail[size..];
		}

		let metadata_len = data[1] as usize;
		let metadata = &tail[..metadata_len];
		*sum += metadata.iter().map(|&num| num as i32).sum::<i32>();

		total_size + metadata_len
	}
	let mut sum = 0;
	walk_tree(data, &mut sum);
	sum
}

fn root_value(data: &[u8]) -> i32 {
	fn walk_tree(data: &[u8]) -> (i32, usize) {
		let child_count = data[0] as usize;

		let mut tail = &data[2..];
		let mut total_size = 2;
		let mut sums = Vec::new();
		for _i in 0..child_count {
			let (sum, size) = walk_tree(tail);
			sums.push(sum);
			total_size += size;
			tail = &tail[size..];
		}

		let metadata_len = data[1] as usize;
		let metadata = &tail[..metadata_len];

		let sum = if child_count == 0 {
			metadata.iter().map(|&num| num as i32).sum()
		}
		else {
			metadata.iter().map(|&num| *sums.get((num - 1) as usize).unwrap_or(&0)).sum()
		};

		(sum, total_size + metadata_len)
	}
	walk_tree(data).0
}


#[cfg(test)]
static TEST_DATA: [u8; 16] = [2,3,0,3,10,11,12,1,1,0,1,99,2,1,1,2];
#[test]
fn test_metadata_sum() {
	assert_eq!(138, metadata_sum(&TEST_DATA));
}
#[test]
fn test_root_value() {
	assert_eq!(66, root_value(&TEST_DATA));
}
