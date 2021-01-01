enum PassportField<'a> {
	PID(&'a str),
	CID(&'a str),
	BYR(&'a str),
	IYR(&'a str),
	EYR(&'a str),
	HGT(&'a str),
	HCL(&'a str),
	ECL(&'a str),
}

impl<'a> PassportField<'a> {
	fn validate(&self) -> bool {
		true
	}
}

struct Passport<'a> {
	pid: PassportField<'a>,
	cid: &'a str,
	byr: &'a str,
	iyr: &'a str,
	eyr: &'a str,
	hgt: &'a str,
	hcl: &'a str,
	ecl: &'a str,
}

impl<'a> Passport<'a> {
	fn from_str(s: &'a str) -> anyhow::Result<Self> {
		Ok(Passport {
			pid: s,
			cid: s,
			byr: s,
			iyr: s,
			eyr: s,
			hgt: s,
			hcl: s,
			ecl: s,
		})
	}
}

fn main() -> anyhow::Result<()> {
	let data = include_str!("input.txt")
		.split("\n\n")
		.collect::<Vec<&str>>();

	let valid_passports = data.into_iter().map(valid_passport).filter(|x| *x).count();

	println!("Part1: {}", valid_passports);

	Ok(())
}

fn valid_passport(s: &str) -> bool {
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
