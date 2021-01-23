use std::collections::HashMap;

#[derive(Default, Debug)]
struct State {
	current: usize,
	diff1: usize,
	diff2: usize,
	diff3: usize,
}

type Joltages = Vec<usize>;

fn main() {
	let mut jolts: Joltages = std::iter::once(0)
		.chain(
			include_str!("input.txt")
				.lines()
				.map(str::parse)
				.map(Result::unwrap),
		)
		.collect();

	jolts.sort_unstable();

	jolts.push(jolts.iter().max().unwrap() + 3);

	part1(&jolts);
	part2(&jolts);
}

fn part2(jolts: &Joltages) {
	let mut num_to_path: HashMap<usize, usize> = HashMap::new();

	let n = jolts.len();

	num_to_path.insert(*jolts.last().unwrap(), 1);

	for i in (0..(n - 1)).into_iter().rev() {
		let val = jolts[i];

		let range = (i + 1)..=std::cmp::min(i + 3, n - 1);

		let neighbours_paths: usize = range
			.into_iter()
			.filter_map(|j| {
				let j_val = jolts[j];
				let gap = j_val - val;

				if (1..=3).contains(&gap) {
					num_to_path.get(&j_val)
				} else {
					None
				}
			})
			.sum();

		num_to_path.insert(val, neighbours_paths);
	}

	println!("part2: {}", num_to_path.get(&0).unwrap());
}

fn part1(jolts: &Joltages) {
	let s = run(jolts);
	println!("part1: {}", s.diff1 * s.diff3);
}

fn run(jolts: &Joltages) -> State {
	let mut s: State = Default::default();

	s = jolts.iter().fold(s, |acc, x| match *x - acc.current {
		0 => acc,
		1 => State {
			current: *x,
			diff1: acc.diff1 + 1,
			..acc
		},
		3 => State {
			current: *x,
			diff3: acc.diff3 + 1,
			..acc
		},
		_ => unreachable!(),
	});

	State {
		diff3: s.diff3 + 1,
		..s
	}
}
