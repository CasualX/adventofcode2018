use std::{io, str, time};
use std::io::Read;

fn main() {
	let stdin = io::stdin();
	let mut input = String::new();
	stdin.lock().read_to_string(&mut input).unwrap();
	let mut claims = Vec::new();
	for line in input.lines() {
		claims.push(line.parse::<Claim>().expect(line));
	}

	let instant1 = time::Instant::now();
	let area = overclaimed(&claims, 1000, 1000);
	let duration1 = instant1.elapsed();
	println!("The overclaimed area is {}. Took {:?}.", area, duration1);

	let instant2 = time::Instant::now();
	let unclaimed_id = find_unclaimed(&claims, 1000, 1000);
	let duration2 = instant2.elapsed();
	println!("The unclaimed id is {}. Took {:?}.", unclaimed_id, duration2);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Claim {
	id: i32,
	x: i32,
	y: i32,
	w: i32,
	h: i32,
}
impl str::FromStr for Claim {
	type Err = ();
	fn from_str(mut s: &str) -> Result<Claim, ()> {
		fn take_str(iter: &mut &str, s: &str) -> Result<(), ()> {
			if iter.starts_with(s) {
				*iter = &(*iter)[s.len()..];
				Ok(())
			}
			else {
				Err(())
			}
		}
		fn take_num(iter: &mut &str) -> Result<i32, ()> {
			let s = iter.as_bytes();
			let mut i = 0;
			while i < s.len() && s[i] >= b'0' && s[i] <= b'9' {
				i += 1;
			}
			let n = (*iter)[..i].parse::<i32>();
			if n.is_err() {
				println!("{}", *iter);
			}
			*iter = &(*iter)[i..];
			n.map_err(|_| ())
		}

		take_str(&mut s, "#")?;
		let id = take_num(&mut s)?;
		take_str(&mut s, " @ ")?;
		let x = take_num(&mut s)?;
		take_str(&mut s, ",")?;
		let y = take_num(&mut s)?;
		take_str(&mut s, ": ")?;
		let w = take_num(&mut s)?;
		take_str(&mut s, "x")?;
		let h = take_num(&mut s)?;

		Ok(Claim { id, x, y, w, h })
	}
}

#[derive(Clone)]
pub struct Fabric {
	width: i32,
	height: i32,
	fabric: Vec<u8>,
}
impl Fabric {
	pub fn new(width: i32, height: i32) -> Fabric {
		let fabric = vec![0; (width * height) as usize];
		Fabric { width, height, fabric }
	}
	fn assert_claim(&self, claim: &Claim) {
		assert!(claim.x >= 0 && claim.y >= 0 &&
			claim.w >= 0 && claim.h >= 0 &&
			claim.x + claim.w <= self.width &&
			claim.y + claim.h <= self.height, "claim: {:?}", claim);
	}
	pub fn claim(&mut self, claim: &Claim, area: &mut i32) {
		self.assert_claim(claim);
		let mut i = (claim.y * self.width + claim.x) as usize;
		let stride = (self.width - claim.w) as usize;
		for _y in 0..claim.h {
			for _x in 0..claim.w {
				if self.fabric[i] == 1 {
					*area += 1;
				}
				self.fabric[i] += 1;
				i += 1;
			}
			i += stride;
		}
	}
	pub fn overlap(&self, claim: &Claim) -> bool {
		self.assert_claim(claim);
		let mut i = (claim.y * self.width + claim.x) as usize;
		let stride = (self.width - claim.w) as usize;
		for _y in 0..claim.h {
			for _x in 0..claim.w {
				if self.fabric[i] != 1 {
					return false;
				}
				i += 1;
			}
			i += stride;
		}
		return true;
	}
}
pub fn overclaimed(claims: &[Claim], width: i32, height: i32) -> i32 {
	let mut fabric = Fabric::new(width, height);
	let mut area = 0;
	for claim in claims {
		fabric.claim(claim, &mut area);
	}
	area
}

#[test]
fn test_overclaimed() {
	let claims = vec![
		"#1 @ 1,3: 4x4".parse().unwrap(),
		"#2 @ 3,1: 4x4".parse().unwrap(),
		"#3 @ 5,5: 2x2".parse().unwrap(),
	];
	assert_eq!(4, overclaimed(&claims, 8, 8));
}

pub fn find_unclaimed(claims: &[Claim], width: i32, height: i32) -> i32 {
	let mut fabric = Fabric::new(width, height);
	let mut area = 0;
	for claim in claims {
		fabric.claim(claim, &mut area);
	}
	for claim in claims {
		if fabric.overlap(claim) {
			return claim.id;
		}
	}
	return 0;
}
