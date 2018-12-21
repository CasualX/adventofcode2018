use std::{io, time};
use std::io::Read;

fn main() {
	let stdin = io::stdin();
	let mut input = String::new();
	stdin.lock().read_to_string(&mut input).unwrap();

	let (width, height, tracks, mut carts) = parse_input(&input);
	let tracks = Tracks { width, height, tracks: &tracks };

	let crash = update_until_crash(&tracks, &mut carts);
	println!("Crashed at {:?}", crash);
	println!("{}", render(&tracks, &carts));
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
enum Track {
	S, // Space
	V, // Vertical line
	H, // Horizontal line
	F, // Forward slash
	B, // Backslash
	X, // Intersection
}
use self::Track::*;
impl Track {
	fn tile(self) -> u8 {
		match self {
			S => b' ',
			V => b'|',
			H => b'-',
			F => b'/',
			B => b'\\',
			X => b'+',
		}
	}
	fn parse(tile: u8) -> Option<Track> {
		match tile {
			b' ' => Some(Track::S),
			b'|' => Some(Track::V),
			b'-' => Some(Track::H),
			b'/' => Some(Track::F),
			b'\\' => Some(Track::B),
			b'+' => Some(Track::X),
			b'>' | b'<' => Some(Track::H),
			b'^' | b'v' => Some(Track::V),
			_ => None
		}
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
enum Dir {
	Up, Right, Down, Left,
}
impl Dir {
	fn left(self) -> Dir {
		match self {
			Dir::Up => Dir::Left,
			Dir::Right => Dir::Up,
			Dir::Down => Dir::Right,
			Dir::Left => Dir::Down,
		}
	}
	fn right(self) -> Dir {
		match self {
			Dir::Up => Dir::Right,
			Dir::Right => Dir::Down,
			Dir::Down => Dir::Left,
			Dir::Left => Dir::Up,
		}
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
enum Steer {
	Left, Straight, Right,
}
impl Steer {
	fn next(self) -> Steer {
		match self {
			Steer::Left => Steer::Straight,
			Steer::Straight => Steer::Right,
			Steer::Right => Steer::Left,
		}
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Point {
	x: i32,
	y: i32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Cart {
	pos: Point,
	crashed: bool,
	dir: Dir,
	steer: Steer,
}
impl Cart {
	// Sets the direction without moving based on the track
	fn turn(&mut self, track: Track) {
		match track {
			Track::S => self.crashed = true,
			Track::V => match self.dir {
				Dir::Up | Dir::Down => (),
				Dir::Left | Dir::Right => self.crashed = true,
			},
			Track::H => match self.dir {
				Dir::Left | Dir::Right => (),
				Dir::Up | Dir::Down => self.crashed = true,
			},
			Track::F => self.dir = match self.dir {
				Dir::Up => Dir::Right,
				Dir::Right => Dir::Up,
				Dir::Down => Dir::Left,
				Dir::Left => Dir::Down,
			},
			Track::B => self.dir = match self.dir {
				Dir::Up => Dir::Left,
				Dir::Left => Dir::Up,
				Dir::Down => Dir::Right,
				Dir::Right => Dir::Down,
			},
			Track::X => {
				match self.steer {
					Steer::Left => self.dir = self.dir.left(),
					Steer::Straight => (),
					Steer::Right => self.dir = self.dir.right(),
				}
				self.steer = self.steer.next();
			},
		}
	}
	// Moves the cart based on the direction by 1
	fn next(&mut self) {
		match self.dir {
			Dir::Left => self.pos.x -= 1,
			Dir::Right => self.pos.x += 1,
			Dir::Up => self.pos.y -= 1,
			Dir::Down => self.pos.y += 1,
		}
	}
	fn tile(&self) -> u8 {
		match self.crashed {
			true => b'X',
			false => match self.dir {
				Dir::Left => b'<',
				Dir::Right => b'>',
				Dir::Up => b'^',
				Dir::Down => b'v',
			}
		}
	}
}

struct Tracks<'a> {
	width: i32,
	height: i32,
	tracks: &'a [Track],
}
impl Tracks<'_> {
	fn get_track(&self, point: Point) -> Track {
		if point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height {
			let i = (point.y * self.width + point.x) as usize;
			self.tracks[i]
		}
		else {
			panic!("Out of bounds! {:?}", point);
		}
	}
}

fn update(tracks: &Tracks, carts: &mut [Cart]) -> Option<usize> {
	let mut crashed_cart = None;
	for i in 0..carts.len() {
		if !carts[i].crashed {
			carts[i].next();
			let track = tracks.get_track(carts[i].pos);
			carts[i].turn(track);
			for j in 0..carts.len() {
				if i != j && carts[i].pos == carts[j].pos {
					carts[i].crashed = true;
					crashed_cart = Some(i);
				}
			}
		}
	}
	crashed_cart
}
fn update_until_crash(tracks: &Tracks, carts: &mut [Cart]) -> Point {
	loop {
		if let Some(crashed_cart) = update(tracks, carts) {
			return carts[crashed_cart].pos;
		}
	}
}
fn render(tracks: &Tracks, carts: &[Cart]) -> String {
	let len = ((tracks.width + 1) * tracks.height) as usize;
	let mut canvas = vec![b' '; len];
	for y in 0..tracks.height {
		let i = (y * (tracks.width + 1)) as usize;
		for x in 0..tracks.width {
			canvas[i + x as usize] = tracks.get_track(Point { x, y }).tile();
		}
		canvas[i + tracks.width as usize] = b'\n';
	}
	for cart in carts {
		let i = (cart.pos.y * (tracks.width + 1) + cart.pos.x) as usize;
		canvas[i] = cart.tile();
	}
	unsafe { String::from_utf8_unchecked(canvas) }
}
fn parse_input(s: &str) -> (i32, i32, Vec<Track>, Vec<Cart>) {
	let mut width = 0;
	let mut height = 0;
	let mut tracks = Vec::new();
	let mut carts = Vec::new();
	for line in s.lines() {
		if width == 0 {
			width = line.len() as i32;
		}
		else if width != line.len() as i32 {
			panic!("inconsistent tracks width!");
		}
		let mut x = 0;
		let y = height;
		height += 1;
		for byte in line.bytes() {
			tracks.push(Track::parse(byte).unwrap());
			if byte == b'>' || byte == b'<' || byte == b'^' || byte == b'v' {
				carts.push(Cart {
					pos: Point { x, y },
					crashed: false,
					dir: match byte { b'>' => Dir::Right, b'<' => Dir::Left, b'^' => Dir::Up, b'v' => Dir::Down, _ => unreachable!() },
					steer: Steer::Left,
				});
			}
			x += 1;
		}
	}
	(width, height, tracks, carts)
}

#[cfg(test)]
static TEST_TRACKS: [Track; 78] = [
	F,H,H,H,B,S,S,S,S,S,S,S,S,
	V,S,S,S,V,S,S,F,H,H,H,H,B,
	V,S,F,H,X,H,H,X,H,B,S,S,V,
	V,S,V,S,V,S,S,V,S,V,S,S,V,
	B,H,X,H,F,S,S,B,H,X,H,H,F,
	S,S,B,H,H,H,H,H,H,F,S,S,S,
];

#[test]
fn test_update() {
	let tracks = Tracks {
		width: 13,
		height: 6,
		tracks: &TEST_TRACKS,
	};
	let mut carts = [
		Cart {
			pos: Point { x: 2, y: 0 },
			crashed: false,
			dir: Dir::Right,
			steer: Steer::Left,
		},
		Cart {
			pos: Point { x: 9, y: 3 },
			crashed: false,
			dir: Dir::Down,
			steer: Steer::Left,
		}
	];
	assert_eq!(Point { x: 7, y: 3 }, update_until_crash(&tracks, &mut carts));
}
