use std::collections::HashSet;

fn main() {
	let input = include_str!("input.txt")
		.split("\n\n")
		.collect::<Vec<&str>>();

	println!("part1: {}", part1(&input));
	println!("part2: {}", part2(&input));
}

fn part1(split: &Vec<&str>) -> usize {
	return split
		.iter()
		.map(|group| {
			let mut h: HashSet<u8> = HashSet::new();

			group.lines().for_each(|line| {
				line.as_bytes().iter().for_each(|c| {
					h.insert(*c);
				});
			});

			return h.len();
		})
		.sum();
}

fn part2(split: &Vec<&str>) -> usize {
	let init: HashSet<u8> = (b'a'..=b'z').collect();

	return split
		.iter()
		.map(|group| {
			group
				.split('\n')
				.map(|line| line.as_bytes().iter().copied().collect::<HashSet<u8>>())
				.fold(init.clone(), |acc, x| {
					acc.intersection(&x).copied().collect()
				})
				.len()
		})
		.sum();
}
