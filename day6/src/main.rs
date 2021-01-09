use std::collections::HashMap;

fn main() {
	let input = include_str!("input.txt")
		.split("\n\n")
		.collect::<Vec<&str>>();

	let sum: usize = input
		.iter()
		.map(|x| {
			let mut h: HashMap<char, bool> = HashMap::new();

			x.split('\n').for_each(|s| {
				s.chars().for_each(|c| {
					h.insert(c, true);
				});
			});

			return h.len();
		})
		.sum();

	println!("part1: {}", sum);
}
