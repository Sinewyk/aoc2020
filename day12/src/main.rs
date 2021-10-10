#[derive(Debug)]
enum InstructionKind {
	MoveNorth,
	MoveSouth,
	MoveEast,
	MoveWest,
	TurnLeft,
	TurnRight,
	GoForward,
}

#[derive(Debug)]
struct Instruction {
	kind: InstructionKind,
	value: isize,
}

struct Position {
	north: isize,
	west: isize,
}

enum Direction {
	N,
	S,
	W,
	E,
}

struct State {
	position: Position,
	direction: Direction,
}

impl State {
	fn exec(self, instruction: &Instruction) -> Self {
		State {
			position: Position { north: 0, west: 0 },
			direction: Direction::E,
		}
	}
}

type Program = Vec<Instruction>;

fn main() {
	let input_as_lines: Vec<&str> = include_str!("input.txt").lines().collect();

	let program: Program = parse(input_as_lines);

	unimplemented!("Execute the program on the starting state");
}

fn parse(input: Vec<&str>) -> Program {
	input
		.iter()
		.map(|line| line_parse(line))
		.collect::<Program>()
}

fn line_parse(line: &str) -> Instruction {
	let first_char = line.get(0..1).unwrap();
	let the_rest = line.get(1..).unwrap();
	Instruction {
		kind: match first_char {
			"N" => InstructionKind::MoveNorth,
			"S" => InstructionKind::MoveSouth,
			"E" => InstructionKind::MoveEast,
			"W" => InstructionKind::MoveWest,
			"L" => InstructionKind::TurnLeft,
			"R" => InstructionKind::TurnRight,
			"F" => InstructionKind::GoForward,
			_ => unreachable!(),
		},
		value: isize::from_str_radix(the_rest, 10).unwrap(),
	}
}
