use std::{io, str, time};
use std::io::Read;

fn main() {
	let stdin = io::stdin();
	let mut input = String::new();
	stdin.lock().read_to_string(&mut input).unwrap();
	let mut points = Vec::new();
	for line in input.lines() {
		points.push(line.parse::<Point>().expect(line));
	}

	let instant1 = time::Instant::now();
	let area = largest_area(&points, 0, 0, 400, 400);
	let duration1 = instant1.elapsed();
	println!("Largest non-infinite area is {}. Took {:?}.", area, duration1);

	let instant2 = time::Instant::now();
	let safe_area = safest_area(&points, 10000, 0, 0, 400, 400);
	let duration2 = instant2.elapsed();
	println!("Safest area size is {}. Took {:?}.", safe_area, duration2);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Point {
	x: i32,
	y: i32,
}
impl str::FromStr for Point {
	type Err = ();
	fn from_str(s: &str) -> Result<Point, ()> {
		let i = s.find(", ").ok_or(())?;
		let x = s[..i].parse().map_err(|_| ())?;
		let y = s[i + 2..].parse().map_err(|_| ())?;
		Ok(Point { x, y })
	}
}
fn dist_hat(p1: Point, p2: Point) -> i32 {
	(p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn find_closest(points: &[Point], p: Point) -> Option<usize> {
	let mut closest_index = None;
	let mut closest_dist = 0x7fffffff;
	let mut closest_count = 0;
	for (i, &point) in points.iter().enumerate() {
		let dist = dist_hat(point, p);
		if dist < closest_dist {
			closest_index = Some(i);
			closest_dist = dist;
			closest_count = 1;
		}
		else if dist == closest_dist {
			closest_index = None;
			closest_count += 1;
		}
	}
	closest_index
}

fn largest_area(points: &[Point], left: i32, top: i32, right: i32, bottom: i32) -> i32 {
	// Given area coverd by the point with the same index
	let mut areas = vec![0; points.len()];
	// Detect 'infinite' areas by marking any point which touches the side of the rectangle
	for x in left - 1..right + 1 {
		if let Some(i) = find_closest(points, Point { x, y: top }) {
			areas[i] = -1;
		}
		if let Some(i) = find_closest(points, Point { x, y: bottom }) {
			areas[i] = -1;
		}
	}
	for y in top - 1..bottom + 1 {
		if let Some(i) = find_closest(points, Point { x: left, y }) {
			areas[i] = -1;
		}
		if let Some(i) = find_closest(points, Point { x: right, y }) {
			areas[i] = -1;
		}
	}
	// Find the largest areas for each point
	// Ignoring points which have infinite area
	let mut largest_area = 0;
	for y in top..bottom {
		for x in left..right {
			if let Some(i) = find_closest(points, Point { x, y }) {
				if areas[i] >= 0 {
					areas[i] += 1;
					if areas[i] > largest_area {
						largest_area = areas[i];
					}
				}
			}
		}
	}
	largest_area
}
fn safest_area(points: &[Point], safe_dist: i32, left: i32, top: i32, right: i32, bottom: i32) -> i32 {
	let mut safe_area = 0;
	for y in top..bottom {
		for x in left..right {
			let dist_sum: i32 = points.iter()
				.map(|&p| dist_hat(Point { x, y }, p))
				.sum();
			if dist_sum < safe_dist {
				safe_area += 1;
			}
		}
	}
	safe_area
}

#[cfg(test)]
static POINTS: [Point; 6] = [
	Point { x: 1, y: 1 },
	Point { x: 1, y: 6 },
	Point { x: 8, y: 3 },
	Point { x: 3, y: 4 },
	Point { x: 5, y: 5 },
	Point { x: 8, y: 9 },
];

#[test]
fn test_largest_area() {
	assert_eq!(17, largest_area(&POINTS, 0, 0, 10, 10));
}
#[test]
fn test_safest_area() {
	assert_eq!(16, safest_area(&POINTS, 32, 0, 0, 10, 10));
}
