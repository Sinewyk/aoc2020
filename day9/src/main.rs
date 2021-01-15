use itertools::Itertools;

type Numbers = Vec<usize>;

const WINDOW_SIZE: usize = 25;

fn main() {
	let input = include_str!("input.txt");
	let nums: Numbers = input.lines().map(str::parse).map(Result::unwrap).collect();

	part1(&nums);
}

fn part1(nums: &Numbers) {
	let found = nums.windows(WINDOW_SIZE + 1).find_map(|s| {
		if can_last_be_summed_from_parts(s) {
			return None;
		} else {
			return Some(s[WINDOW_SIZE]);
		}
	});

	println!("part 1: {}", found.unwrap());
}

fn can_last_be_summed_from_parts(parts: &[usize]) -> bool {
	(&parts[..WINDOW_SIZE])
		.iter()
		.tuple_combinations()
		.any(|(a, b)| a + b == parts[WINDOW_SIZE])
}
