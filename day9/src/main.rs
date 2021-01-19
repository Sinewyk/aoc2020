use itertools::Itertools;

type Numbers = Vec<usize>;

const WINDOW_SIZE: usize = 25;

fn main() {
	let input = include_str!("input.txt");
	let nums: Numbers = input.lines().map(str::parse).map(Result::unwrap).collect();

	let part1 = part1(&nums);

	part2(&nums, &part1);
}

fn part1(nums: &Numbers) -> usize {
	let found = nums
		.windows(WINDOW_SIZE + 1)
		.find_map(|s| {
			if can_last_be_summed_from_parts(s) {
				return None;
			} else {
				return Some(s[WINDOW_SIZE]);
			}
		})
		.unwrap();

	println!("part 1: {}", found);

	found
}

fn part2(nums: &Numbers, number_to_find: &usize) {
	let loop_up_to = nums.len();

	// Brute force, search in an ever increasing window for a contiguous sum that adds up
	// to what we are looking for
	for x in 2..loop_up_to {
		if let Some(found) = nums.windows(x).find_map(|window| {
			if window.iter().sum::<usize>() == *number_to_find {
				Some(window)
			} else {
				return None;
			}
		}) {
			let min = found.iter().min().unwrap();
			let max = found.iter().max().unwrap();
			println!("part2: {}", min + max);
			break;
		}
	}
}

fn can_last_be_summed_from_parts(parts: &[usize]) -> bool {
	(&parts[..WINDOW_SIZE])
		.iter()
		.tuple_combinations()
		.any(|(a, b)| a + b == parts[WINDOW_SIZE])
}
