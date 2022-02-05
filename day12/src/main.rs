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

#[derive(Debug, PartialEq, Clone, Copy)]
struct Position {
	north: isize,
	west: isize,
}

#[derive(Debug, PartialEq)]
struct State {
	waypoint_vector: Position,
	ship_position: Position,
}

impl State {
	fn exec(self, instruction: &Instruction) -> Self {
		match instruction.kind {
			InstructionKind::MoveNorth => State {
				waypoint_vector: Position {
					north: self.waypoint_vector.north + instruction.value,
					..self.waypoint_vector
				},
				..self
			},
			InstructionKind::MoveSouth => State {
				waypoint_vector: Position {
					north: self.waypoint_vector.north - instruction.value,
					..self.waypoint_vector
				},
				..self
			},
			InstructionKind::MoveEast => State {
				waypoint_vector: Position {
					west: self.waypoint_vector.west - instruction.value,
					..self.waypoint_vector
				},
				..self
			},
			InstructionKind::MoveWest => State {
				waypoint_vector: Position {
					west: self.waypoint_vector.west + instruction.value,
					..self.waypoint_vector
				},
				..self
			},
			InstructionKind::TurnLeft => State {
				waypoint_vector: match instruction.value {
					90 => Position {
						north: -self.waypoint_vector.west,
						west: self.waypoint_vector.north,
					},
					180 => Position {
						north: -self.waypoint_vector.north,
						west: -self.waypoint_vector.west,
					},
					270 => Position {
						north: self.waypoint_vector.west,
						west: -self.waypoint_vector.north,
					},
					_ => unreachable!(),
				},
				..self
			},
			InstructionKind::TurnRight => State {
				waypoint_vector: match instruction.value {
					90 => Position {
						north: self.waypoint_vector.west,
						west: -self.waypoint_vector.north,
					},
					180 => Position {
						north: -self.waypoint_vector.north,
						west: -self.waypoint_vector.west,
					},
					270 => Position {
						north: -self.waypoint_vector.west,
						west: self.waypoint_vector.north,
					},
					_ => unreachable!(),
				},
				..self
			},
			InstructionKind::GoForward => State {
				ship_position: Position {
					north: self.ship_position.north
						+ self.waypoint_vector.north * instruction.value,
					west: self.ship_position.west + self.waypoint_vector.west * instruction.value,
				},
				..self
			},
		}
	}
}

type Program = Vec<Instruction>;

fn main() {
	let input_as_lines: Vec<&str> = include_str!("input.txt").lines().collect();

	let program: Program = parse(input_as_lines);

	let final_state = program.iter().fold(
		State {
			waypoint_vector: Position {
				north: 1,
				west: -10,
			},
			ship_position: Position { north: 0, west: 0 },
		},
		|prev_state, instruction| prev_state.exec(instruction),
	);

	println!(
		"N: {}, W: {} and so manhattan distance is {}",
		final_state.ship_position.north,
		final_state.ship_position.west,
		final_state.ship_position.north.abs() + final_state.ship_position.west.abs()
	);
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
