use std::{cmp, time};

const GRID_WIDTH: i32 = 300;
const GRID_HEIGHT: i32 = 300;

// It's much faster to only scan a subgrid!
const MAX_SUBGRID_SIZE: i32 = 20;

fn main() {
	let grid_serial = 8868;
	part1(grid_serial);
	part2(grid_serial);
}
fn part1(grid_serial: i32) {
	let instant = time::Instant::now();
	let (x, y, power) = largest_power_3x3(grid_serial);
	let duration = instant.elapsed();
	println!("At {},{} the largest total power is {}. Took {:?}.", x, y, power, duration);
}
fn part2(grid_serial: i32) {
	let instant = time::Instant::now();
	let (x, y, size, power) = largest_power_any(grid_serial);
	let duration = instant.elapsed();
	println!("At {},{},{} the largest total power is {}. Took {:?}.", x, y, size, power, duration);
}

fn power_level_cell(x: i32, y: i32, grid_serial: i32) -> i32 {
	let rack_id = x + 10;
	let power_level = (rack_id * y + grid_serial) * rack_id;
	let digit = (power_level / 100) % 10;
	digit - 5
}

#[test]
fn test_power_level_cell() {
	assert_eq!(4, power_level_cell(3, 5, 8));
	assert_eq!(-5, power_level_cell(122, 79, 57));
	assert_eq!(0, power_level_cell(217, 196, 39));
	assert_eq!(4, power_level_cell(101, 153, 71));
}

// Progressively calculates the power levels starting from size = 1
fn power_level_sum(x: i32, y: i32, size: i32, grid_serial: i32, sum: &mut i64) {
	for x in x..x + size {
		*sum += power_level_cell(x, y + size - 1, grid_serial) as i64;
	}
	for y in y..y + size - 1 {
		*sum += power_level_cell(x + size - 1, y, grid_serial) as i64;
	}
}

fn largest_power_3x3(grid_serial: i32) -> (i32, i32, i64) {
	let mut total_x = 0;
	let mut total_y = 0;
	let mut total_power = i64::min_value();
	for y in 1..GRID_HEIGHT - 2 {
		for x in 1..GRID_WIDTH - 2 {
			let mut power = 0;
			power_level_sum(x, y, 1, grid_serial, &mut power);
			power_level_sum(x, y, 2, grid_serial, &mut power);
			power_level_sum(x, y, 3, grid_serial, &mut power);
			if power > total_power {
				total_x = x;
				total_y = y;
				total_power = power;
			}
		}
	}
	(total_x, total_y, total_power)
}
fn largest_power_any(grid_serial: i32) -> (i32, i32, i32, i64) {
	let mut total_x = 0;
	let mut total_y = 0;
	let mut total_size = 0;
	let mut total_power = i64::min_value();
	for y in 1..=GRID_HEIGHT {
		for x in 1..=GRID_WIDTH {
			let mut power = 0;
			for size in 1..=cmp::min(MAX_SUBGRID_SIZE, cmp::min(GRID_WIDTH - x, GRID_HEIGHT - y)) {
				power_level_sum(x, y, size, grid_serial, &mut power);
				if power > total_power {
					total_x = x;
					total_y = y;
					total_size = size;
					total_power = power;
				}
			}
		}
	}
	(total_x, total_y, total_size, total_power)
}

#[test]
fn test_largest_power_3x3() {
	assert_eq!((33, 45, 29), largest_power_3x3(18));
	assert_eq!((21, 61, 30), largest_power_3x3(42));
}
#[test]
fn test_largest_power_any() {
	assert_eq!((90,269,16,113), largest_power_any(18));
	assert_eq!((232,251,12,119), largest_power_any(42));
}
