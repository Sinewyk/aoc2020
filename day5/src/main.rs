fn main() {
	let boarding_passes = include_str!("input.txt").split('\n').collect::<Vec<&str>>();
	let mut seat_ids = boarding_passes
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

	part1(&seat_ids);
	part2(&seat_ids);
}

fn part1(v: &Vec<usize>) {
	println!("Highest ticket: {}", v.last().unwrap());
}

fn part2(v: &Vec<usize>) {
	let l = v.chunks(2);

	for p in l {
		if p.len() == 2 && p[0] != p[1] - 1 {
			return println!("My ticket {}", p[0] + 1);
		}
	}
}
