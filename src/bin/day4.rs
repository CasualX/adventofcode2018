use std::{io, str, time};
use std::io::Read;
use std::collections::HashMap;

fn main() {
	let stdin = io::stdin();
	let mut input = String::new();
	stdin.lock().read_to_string(&mut input).unwrap();
	let mut records = Vec::new();
	for line in input.lines() {
		records.push(line.parse::<Record>().expect(line));
	}

	// Ensure the records are sorted before analysis
	records.sort_by_key(|rec| rec.time());
	let snoozes = analyse(&records);

	let instant1 = time::Instant::now();
	let result1 = strategy1(&snoozes);
	let duration1 = instant1.elapsed();
	println!("Strategy 1: The GuardID ({}) and minute ({} min) produce {}. Took {:?}.",
		result1.guard_id, result1.minute, result1.checksum(), duration1);

	let instant2 = time::Instant::now();
	let result2 = strategy2(&snoozes);
	let duration2 = instant2.elapsed();
	println!("Strategy 2: The GuardID ({}) and minute ({} min) produce {}. Took {:?}.",
		result2.guard_id, result2.minute, result2.checksum(), duration2);
}

#[derive(Copy, Clone, Debug)]
enum Record {
	BeginShift(i32, i32),
	FallsAsleep(i32, i32),
	WakesUp(i32, i32),
}
impl Record {
	fn time(&self) -> i32 {
		match *self {
			Record::BeginShift(time, _) => time,
			Record::FallsAsleep(time, _) => time,
			Record::WakesUp(time, _) => time,
		}
	}
}
impl str::FromStr for Record {
	type Err = ();
	fn from_str(s: &str) -> Result<Record, ()> {
		let month: i32 = s[6..8].parse().map_err(|_| ())?;
		let day: i32 = s[9..11].parse().map_err(|_| ())?;
		let hour: i32 = s[12..14].parse().map_err(|_| ())?;
		let min = s[15..17].parse().map_err(|_| ())?;
		let time = min + hour * 60 + day * 60 * 24 + month * 60 * 24 * 32;
		match &s[19..24] {
			"Guard" => {
				let gid = &s[26..];
				let mut n = 0;
				while gid.as_bytes()[n] >= b'0' && gid.as_bytes()[n] <= b'9' {
					n += 1;
				}
				let guard_id = gid[..n].parse().map_err(|_| ())?;
				Ok(Record::BeginShift(time, guard_id))
			},
			"falls" => Ok(Record::FallsAsleep(time, min)),
			"wakes" => Ok(Record::WakesUp(time, min)),
			_ => panic!("Invalid record: {}", s),
		}
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Snooze {
	guard_id: i32,
	start: i32,
	end: i32,
}
impl Snooze {
	fn new(guard_id: i32, start: i32, end: i32) -> Snooze {
		Snooze { guard_id, start, end }
	}
}

fn analyse(records: &[Record]) -> Vec<Snooze> {
	// Keep track of the order of records
	// [BeginShift [-> Falls Asleep -> Wakes Up]*]*
	let mut snoozes = Vec::new();
	let mut guard_id = None;
	let mut falls_asleep = None;
	for (line, &rec) in records.iter().enumerate() {
		match rec {
			Record::BeginShift(_, id) => {
				guard_id = Some(id);
				falls_asleep = None;
			},
			Record::FallsAsleep(_, min) => {
				if guard_id.is_none() {
					panic!("Missing BeginShift (line: {})", line);
				}
				if let Some(_) = falls_asleep {
					panic!("Unexpected FallsAsleep (line: {})", line);
				}
				falls_asleep = Some(min);
			},
			Record::WakesUp(_, wakes_up) => {
				if guard_id.is_none() {
					panic!("Missing BeginShift (line: {})", line);
				}
				if falls_asleep.is_none() {
					panic!("Missing FallsAsleep (line: {})", line);
				}
				snoozes.push(Snooze::new(guard_id.unwrap(), falls_asleep.unwrap(), wakes_up));
				falls_asleep = None;
			},
		}
	}
	snoozes
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct GuardMin {
	guard_id: i32,
	minute: i32,
}
impl GuardMin {
	fn checksum(&self) -> i32 {
		self.guard_id * self.minute
	}
}
fn strategy1(snoozes: &[Snooze]) -> GuardMin {
	let ids_len = snoozes.iter().map(|snooze| snooze.guard_id).max().unwrap() + 1;
	let mut asleep = vec![0; ids_len as usize];
	let mut total_sleep = 0;
	let mut total_id = -1;
	for snooze in snoozes {
		asleep[snooze.guard_id as usize] += snooze.end - snooze.start;
		if asleep[snooze.guard_id as usize] > total_sleep {
			total_sleep = asleep[snooze.guard_id as usize];
			total_id = snooze.guard_id;
		}
	}
	let mut asleep_mins = [0; 60];
	let mut most_min = -1;
	let mut most_total = 0;
	for snooze in snoozes.iter().filter(|snooze| snooze.guard_id == total_id) {
		for min in snooze.start..snooze.end {
			asleep_mins[min as usize] += 1;
			if asleep_mins[min as usize] > most_total {
				most_total = asleep_mins[min as usize];
				most_min = min;
			}
		}
	}
	GuardMin {
		guard_id: total_id,
		minute: most_min,
	}
}
fn strategy2(snoozes: &[Snooze]) -> GuardMin {
	let mut asleep_mins = HashMap::<i32, [u8; 60]>::new();
	let mut guard_id = -1;
	let mut guard_min = 0;
	let mut guard_total = 0;
	for snooze in snoozes {
		for min in snooze.start..snooze.end {
			let e = asleep_mins.entry(snooze.guard_id).or_insert([0; 60]);
			(*e)[min as usize] += 1;
			if (*e)[min as usize] > guard_total {
				guard_id = snooze.guard_id;
				guard_min = min;
				guard_total = (*e)[min as usize];
			}
		}
	}
	GuardMin {
		guard_id: guard_id,
		minute: guard_min,
	}
}

#[cfg(test)]
static TEST_RECORDS: [Record; 17] = [
	Record::BeginShift(0, 10),
	Record::FallsAsleep(1, 5),
	Record::WakesUp(2, 25),
	Record::FallsAsleep(3, 30),
	Record::WakesUp(4, 55),
	Record::BeginShift(5, 99),
	Record::FallsAsleep(6, 40),
	Record::WakesUp(7, 50),
	Record::BeginShift(8, 10),
	Record::FallsAsleep(9, 24),
	Record::WakesUp(10, 29),
	Record::BeginShift(11, 99),
	Record::FallsAsleep(12, 36),
	Record::WakesUp(13, 46),
	Record::BeginShift(14, 99),
	Record::FallsAsleep(15, 45),
	Record::WakesUp(16, 55),
];
#[cfg(test)]
static TEST_SNOOZES: [Snooze; 6] = [
	Snooze { guard_id: 10, start: 5, end: 25 },
	Snooze { guard_id: 10, start: 30, end: 55 },
	Snooze { guard_id: 99, start: 40, end: 50 },
	Snooze { guard_id: 10, start: 24, end: 29 },
	Snooze { guard_id: 99, start: 36, end: 46 },
	Snooze { guard_id: 99, start: 45, end: 55 },
];

#[test]
fn test_parse() {
	assert_eq!(&TEST_SNOOZES, &analyse(&TEST_RECORDS)[..]);
}
#[test]
fn test_strategy1() {
	let expected = GuardMin {
		guard_id: 10,
		minute: 24,
	};
	assert_eq!(expected, strategy1(&TEST_SNOOZES));
}
#[test]
fn test_strategy2() {
	let expected = GuardMin {
		guard_id: 99,
		minute: 45,
	};
	assert_eq!(expected, strategy2(&TEST_SNOOZES));
}
