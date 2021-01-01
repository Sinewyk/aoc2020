fn main() -> anyhow::Result<()> {
	let num = include_str!("input.txt")
		.split('\n')
		.filter(|&x| !x.is_empty())
		.map(str::parse::<usize>)
		.collect::<Result<Vec<usize>, _>>()?;

	if let Some(x) = part1(&num) {
		println!("part1: {}", x);
	}

	if let Some(x) = part2(&num) {
		println!("part2: {}", x);
	}

	Ok(())
}

fn part1(s: &Vec<usize>) -> Option<usize> {
	for (i, x) in s.iter().enumerate() {
		for (j, y) in s.iter().enumerate() {
			if i == j {
				continue;
			};
			if x + y == 2020 {
				let res = x * y;
				return Some(res);
			}
		}
	}
	None
}

fn part2(s: &Vec<usize>) -> Option<usize> {
	for (i, x) in s.iter().enumerate() {
		for (j, y) in s.iter().enumerate() {
			for (k, z) in s.iter().enumerate() {
				if i == j || i == k || j == k {
					continue;
				};
				if x + y + z == 2020 {
					let res = x * y * z;
					return Some(res);
				}
			}
		}
	}
	None
}
