#[derive(Default, Debug)]
struct State {
	current: usize,
	diff1: usize,
	diff2: usize,
	diff3: usize,
}

type Joltages = Vec<usize>;

fn main() {
	let mut jolts: Joltages = include_str!("input.txt")
		.lines()
		.map(str::parse)
		.map(Result::unwrap)
		.collect();

	jolts.sort();

	part1(&jolts);
}

fn part1(jolts: &Joltages) {
	let s = run(jolts);
	println!("part1: {}", s.diff1 * s.diff3);
}

fn run(jolts: &Joltages) -> State {
	let mut s: State = Default::default();

	s = jolts.iter().fold(s, |acc, x| match *x - acc.current {
		1 => State {
			current: *x,
			diff1: acc.diff1 + 1,
			..acc
		},
		2 => State {
			current: *x,
			diff2: acc.diff2 + 1,
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
