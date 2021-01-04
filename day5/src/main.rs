fn main() {
	let boarding_passes = include_str!("input.txt").split('\n').collect::<Vec<&str>>();

	part1(&boarding_passes);
}

fn part1(v: &Vec<&str>) {
	let mut seat_ids = v
		.iter()
		.map(|s| {
			let temp = s
				.chars()
				.map(|x| match x {
					'F' => '0',
					'B' => '1',
					'L' => '0',
					'R' => '1',
					x => unreachable!("Boarding pass contained {}, which is invalid", x),
				})
				.collect::<String>();

			let (r, c) = temp.split_at(7);

			let row = usize::from_str_radix(r, 2).unwrap();
			let col = usize::from_str_radix(c, 2).unwrap();

			return row * 8 + col;
		})
		.collect::<Vec<usize>>();

	seat_ids.sort();

	print!("Highest ticket: {}", seat_ids.last().unwrap());
}
