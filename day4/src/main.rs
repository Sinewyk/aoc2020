use anyhow::{anyhow, Result};

// I guess the goal for now is to link the validation to the enums ... no idea what i'm doing for now
enum PassportField<'a> {
	// CID(&'a str),
	PID(&'a str),
	BYR(&'a str),
	IYR(&'a str),
	EYR(&'a str),
	HGT(&'a str),
	HCL(&'a str),
	ECL(&'a str),
}

impl<'a> PassportField<'a> {
	fn validate(&self) -> Result<()> {
		match self {
			Self::PID(x) => {
				if x.len() != 9 {
					return Err(anyhow!("PID invalide"));
				}
				x.parse::<usize>()?; // ensure it's a number
				Ok(())
			}
			Self::BYR(x) => match x.parse::<usize>() {
				Ok(i) => {
					if i >= 1920 && i <= 2002 {
						Ok(())
					} else {
						Err(anyhow!("BYR invalide"))
					}
				}
				Err(x) => Err(anyhow!(x)),
			},
			Self::IYR(x) => match x.parse::<usize>() {
				Ok(i) => {
					if i >= 2010 && i <= 2020 {
						Ok(())
					} else {
						Err(anyhow!("IYR invalide"))
					}
				}
				Err(x) => Err(anyhow!(x)),
			},
			Self::EYR(x) => match x.parse::<usize>() {
				Ok(i) => {
					if i >= 2020 && i <= 2030 {
						Ok(())
					} else {
						Err(anyhow!("IYR invalide"))
					}
				}
				Err(x) => Err(anyhow!(x)),
			},
			Self::HGT(x) => match &x[x.len() - 2..] {
				"cm" => {
					let len = x[0..x.len() - 2].parse::<usize>()?;
					if len > 193 || len < 150 {
						return Err(anyhow!("HGT invalide"));
					}
					Ok(())
				}
				"in" => {
					let len = x[0..x.len() - 2].parse::<usize>()?;
					if len > 76 || len < 59 {
						return Err(anyhow!("HGT invalide"));
					}
					Ok(())
				}
				_ => Err(anyhow!("HGT invalide")),
			},
			Self::HCL(x) => {
				if x.len() != 7 || &x[0..1] != "#" || !x[1..7].chars().all(|x| x.is_digit(16)) {
					return Err(anyhow!("HCL invalide"));
				}
				Ok(())
			}
			Self::ECL(x) => match *x {
				"amb" => Ok(()),
				"blu" => Ok(()),
				"brn" => Ok(()),
				"gry" => Ok(()),
				"grn" => Ok(()),
				"hzl" => Ok(()),
				"oth" => Ok(()),
				_ => Err(anyhow!("ECL invalide")),
			},
		}
	}
}

struct Passport<'a> {
	// cid: PassportField<'a>,
	pid: PassportField<'a>,
	byr: PassportField<'a>,
	iyr: PassportField<'a>,
	eyr: PassportField<'a>,
	hgt: PassportField<'a>,
	hcl: PassportField<'a>,
	ecl: PassportField<'a>,
}

impl<'a> Passport<'a> {
	fn from_str(s: &'a str) -> anyhow::Result<Self> {
		let parts = s
			.split(|x| x == ' ' || x == '\n')
			.map(|s| {
				let mut inner_parts = s.split(':');
				let u = (inner_parts.next().unwrap(), inner_parts.next().unwrap());
				u
			})
			.collect::<Vec<(&str, &str)>>();

		// I need to find a way to make that shit better, i don't know if I can, it's not exactly JSON
		//
		let pid = PassportField::PID(
			parts
				.iter()
				.find(|x| x.0 == "pid")
				.ok_or(anyhow!("Error parsing the passport"))?
				.1,
		);
		pid.validate()?;

		let byr = PassportField::BYR(
			parts
				.iter()
				.find(|x| x.0 == "byr")
				.ok_or(anyhow!("Error parsing the passport"))?
				.1,
		);
		byr.validate()?;

		let iyr = PassportField::IYR(
			parts
				.iter()
				.find(|x| x.0 == "iyr")
				.ok_or(anyhow!("Error parsing the passport"))?
				.1,
		);
		iyr.validate()?;

		let eyr = PassportField::EYR(
			parts
				.iter()
				.find(|x| x.0 == "eyr")
				.ok_or(anyhow!("Error parsing the passport"))?
				.1,
		);
		eyr.validate()?;

		let hgt = PassportField::HGT(
			parts
				.iter()
				.find(|x| x.0 == "hgt")
				.ok_or(anyhow!("Error parsing the passport"))?
				.1,
		);
		hgt.validate()?;

		let hcl = PassportField::HCL(
			parts
				.iter()
				.find(|x| x.0 == "hcl")
				.ok_or(anyhow!("Error parsing the passport"))?
				.1,
		);
		hcl.validate()?;

		let ecl = PassportField::ECL(
			parts
				.iter()
				.find(|x| x.0 == "ecl")
				.ok_or(anyhow!("Error parsing the passport"))?
				.1,
		);
		ecl.validate()?;

		Ok(Passport {
			// cid: PassportField::CID(s),
			pid,
			byr,
			iyr,
			eyr,
			hgt,
			hcl,
			ecl,
		})
	}
}

fn main() -> anyhow::Result<()> {
	let data = include_str!("input.txt")
		.split("\n\n")
		.collect::<Vec<&str>>();

	part1(&data);
	part2(&data);

	Ok(())
}

fn raw_valid_passport(s: &&str) -> bool {
	let needed_keys = vec![
		"pid", /*"cid",*/ "byr", "iyr", "eyr", "hgt", "hcl", "ecl",
	];

	let parts = s
		.split(|x| x == ' ' || x == '\n')
		.map(|s| {
			let mut inner_parts = s.split(':');
			let u = (inner_parts.next().unwrap(), inner_parts.next().unwrap());
			u
		})
		.collect::<Vec<(&str, &str)>>();

	for key in needed_keys {
		if !parts.iter().any(|x| x.0 == key) {
			return false;
		};
	}

	true
}

fn part1(s: &Vec<&str>) {
	let valid_passports = s.iter().map(raw_valid_passport).filter(|x| *x).count();

	println!("Part1: {}", valid_passports);
}

fn part2(s: &Vec<&str>) {
	let valid_passports = s
		.iter()
		.map(|x| Passport::from_str(*x))
		.filter(Result::is_ok)
		.count();

	println!("Part2: {}", valid_passports);
}
