use std::{cmp, i32, i64, io, str, time};
use std::io::Read;

fn main() {
	let stdin = io::stdin();
	let mut input = String::new();
	stdin.lock().read_to_string(&mut input).unwrap();
	let mut stars = Vec::new();
	for line in input.lines() {
		stars.push(line.parse::<Star>().expect(line));
	}

	let instant = time::Instant::now();
	let (min_time, min_area) = solve_min(&stars, 20000);
	time_steps(&mut stars, min_time);
	let duration = instant.elapsed();

	println!("Most compact at {} sec with area {} units². Took {:?}.", min_time, min_area, duration);
	println!("{}", render(&stars));
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Point {
	x: i32,
	y: i32,
}
impl str::FromStr for Point {
	type Err = ();
	fn from_str(s: &str) -> Result<Point, ()> {
		let i = s.find(",").ok_or(())?;
		let x = s[..i].trim().parse().map_err(|_| ())?;
		let y = s[i + 1..].trim().parse().map_err(|_| ())?;
		Ok(Point { x, y })
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Star {
	position: Point,
	velocity: Point,
}
impl str::FromStr for Star {
	type Err = ();
	fn from_str(s: &str) -> Result<Star, ()> {
		let i1 = s.find("<").ok_or(())?;
		let i2 = s.find(">").ok_or(())?;
		let position = s[i1 + 1..i2].parse()?;
		let i3 = s[i2 + 1..].find("<").ok_or(())? + i2 + 1;
		let i4 = s[i2 + 1..].find(">").ok_or(())? + i2 + 1;
		let velocity = s[i3 + 1..i4].parse()?;
		Ok(Star { position, velocity })
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Bounds {
	left: i32,
	top: i32,
	right: i32,
	bottom: i32,
}

// Move the stars a single second
fn time_step(stars: &mut [Star]) {
	for star in stars {
		star.position.x += star.velocity.x;
		star.position.y += star.velocity.y;
	}
}
// Move the stars a number of seconds
fn time_steps(stars: &mut [Star], t: i32) {
	for star in stars {
		let x = star.position.x + star.velocity.x * t;
		let y = star.position.y + star.velocity.y * t;
		star.position.x = x;
		star.position.y = y;
	}
}
// Calculate the bounds of all the stars
fn bounds(stars: &[Star]) -> Bounds {
	let mut rc = Bounds {
		left: i32::MAX,
		right: i32::MIN,
		top: i32::MAX,
		bottom: i32::MIN,
	};
	for star in stars {
		rc.left = cmp::min(rc.left, star.position.x);
		rc.right = cmp::max(rc.right, star.position.x);
		rc.top = cmp::min(rc.top, star.position.y);
		rc.bottom = cmp::max(rc.bottom, star.position.y);
	}
	rc
}
// Solve for time where bounds are minimized
fn solve_min(stars: &[Star], limit: i32) -> (i32, i64) {
	let mut stars = stars.to_vec();
	let mut min_area = i64::MAX;
	let mut min_time = 0;
	for time in 0..limit {
		let bounds = bounds(&stars);
		let area = (bounds.right - bounds.left) as i64 * (bounds.bottom - bounds.top) as i64;
		if area < min_area {
			min_area = area;
			min_time = time;
		}
		time_step(&mut stars);
	}
	(min_time, min_area)
}
// Render the stars to a string
fn render(stars: &[Star]) -> String {
	let left = stars.iter().map(|star| star.position.x).min().unwrap();
	let right = stars.iter().map(|star| star.position.x).max().unwrap();
	let top = stars.iter().map(|star| star.position.y).min().unwrap();
	let bottom = stars.iter().map(|star| star.position.y).max().unwrap();
	eprintln!("left:{} top:{} right:{} bottom:{}", left, top, right, bottom);
	let factor = 80.0 / (right - left) as f32;
	let width = 80;
	let height = 16;
	let mut canvas = vec![vec![b' '; width]; height];
	for star in stars {
		let x = ((star.position.x - left) as f32 * factor) as i32;
		let y = ((star.position.y - top) as f32 * factor) as i32;
		if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
			canvas[y as usize][x as usize] = b'#';
		}
	}
	String::from_utf8(canvas.as_slice().join(&b'\n')).unwrap()
}

#[cfg(test)]
static TEST_STARS: [Star; 31] = [
	Star { position: Point { x: 9, y: 1 }, velocity: Point { x: 0, y: 2 } },
	Star { position: Point { x: 7, y: 0 }, velocity: Point { x: -1, y: 0 } },
	Star { position: Point { x: 3, y: -2 }, velocity: Point { x: -1, y: 1 } },
	Star { position: Point { x: 6, y: 10 }, velocity: Point { x: -2, y: -1 } },
	Star { position: Point { x: 2, y: -4 }, velocity: Point { x: 2, y: 2 } },
	Star { position: Point { x: -6, y: 10 }, velocity: Point { x: 2, y: -2 } },
	Star { position: Point { x: 1, y: 8 }, velocity: Point { x: 1, y: -1 } },
	Star { position: Point { x: 1, y: 7 }, velocity: Point { x: 1, y: 0 } },
	Star { position: Point { x: -3, y: 11 }, velocity: Point { x: 1, y: -2 } },
	Star { position: Point { x: 7, y: 6 }, velocity: Point { x: -1, y: -1 } },
	Star { position: Point { x: -2, y: 3 }, velocity: Point { x: 1, y: 0 } },
	Star { position: Point { x: -4, y: 3 }, velocity: Point { x: 2, y: 0 } },
	Star { position: Point { x: 10, y: -3 }, velocity: Point { x: -1, y: 1 } },
	Star { position: Point { x: 5, y: 11 }, velocity: Point { x: 1, y: -2 } },
	Star { position: Point { x: 4, y: 7 }, velocity: Point { x: 0, y: -1 } },
	Star { position: Point { x: 8, y: -2 }, velocity: Point { x: 0, y: 1 } },
	Star { position: Point { x: 15, y: 0 }, velocity: Point { x: -2, y: 0 } },
	Star { position: Point { x: 1, y: 6 }, velocity: Point { x: 1, y: 0 } },
	Star { position: Point { x: 8, y: 9 }, velocity: Point { x: 0, y: -1 } },
	Star { position: Point { x: 3, y: 3 }, velocity: Point { x: -1, y: 1 } },
	Star { position: Point { x: 0, y: 5 }, velocity: Point { x: 0, y: -1 } },
	Star { position: Point { x: -2, y: 2 }, velocity: Point { x: 2, y: 0 } },
	Star { position: Point { x: 5, y: -2 }, velocity: Point { x: 1, y: 2 } },
	Star { position: Point { x: 1, y: 4 }, velocity: Point { x: 2, y: 1 } },
	Star { position: Point { x: -2, y: 7 }, velocity: Point { x: 2, y: -2 } },
	Star { position: Point { x: 3, y: 6 }, velocity: Point { x: -1, y: -1 } },
	Star { position: Point { x: 5, y: 0 }, velocity: Point { x: 1, y: 0 } },
	Star { position: Point { x: -6, y: 0 }, velocity: Point { x: 2, y: 0 } },
	Star { position: Point { x: 5, y: 9 }, velocity: Point { x: 1, y: -2 } },
	Star { position: Point { x: 14, y: 7 }, velocity: Point { x: -2, y: 0 } },
	Star { position: Point { x: -3, y: 6 }, velocity: Point { x: 2, y: -1 } },
];

#[test]
fn test_bounds() {
	let expected = Bounds { left: -6, top: -4, right: 15, bottom: 11 };
	assert_eq!(expected, bounds(&TEST_STARS));
}
#[test]
fn test_part1() {
	let mut stars = TEST_STARS.to_vec();
	time_steps(&mut stars, 3);
	println!("\n{}", render(&stars));
}
