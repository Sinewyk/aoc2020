use anyhow::Result;

#[derive(Debug)]
struct Pair<'a> {
	policy: Policy,
	password: &'a str,
}

impl<'a> Pair<'a> {
	fn from_str(s: &'a str) -> Self {
		let parts = s.split(' ').collect::<Vec<&str>>();
		let least_and_most = parts[0]
			.split('-')
			.map(str::parse::<usize>)
			.map(Result::unwrap)
			.collect::<Vec<usize>>();

		let least = least_and_most[0];
		let most = least_and_most[1];

		let char = parts[1].chars().next().unwrap();

		Self {
			policy: Policy { least, most, char },
			password: parts[2],
		}
	}

	fn is_valid_part1(&self) -> bool {
		let num = self
			.password
			.chars()
			.filter(|c| c == &self.policy.char)
			.count();

		if num >= self.policy.least && num <= self.policy.most {
			true
		} else {
			false
		}
	}

	fn is_valid_part2(&self) -> bool {
		let first_char = &self.password[(self.policy.least - 1)..self.policy.least];
		let second_char = &self.password[(self.policy.most - 1)..self.policy.most];

		if first_char == self.policy.char.to_string() && second_char == self.policy.char.to_string()
		{
			// Both the char is false
			false
		} else if first_char == self.policy.char.to_string()
			|| second_char == self.policy.char.to_string()
		{
			// One of them is ok
			true
		} else {
			/* None of them is not ok*/
			false
		}
	}
}

#[derive(Debug)]
struct Policy {
	least: usize,
	most: usize,
	char: char,
}

fn main() -> Result<()> {
	let data = include_str!("input.txt")
		.split('\n')
		.filter(|&x| !x.is_empty())
		.map(Pair::from_str)
		.collect::<Vec<Pair>>();

	let first_count = &data.iter().filter(|p| p.is_valid_part1()).count();
	let second_count = &data.iter().filter(|p| p.is_valid_part2()).count();

	println!("Valid: {}", first_count);
	println!("Valid: {}", second_count);

	Ok(())
}
