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

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
	N,
	S,
	W,
	E,
}

impl Direction {
	fn turn(&self, angle: isize) -> Self {
		match (self, angle) {
			(Direction::N, 90) => Direction::E,
			(Direction::N, 180) => Direction::S,
			(Direction::N, 270) => Direction::W,
			(Direction::N, -90) => Direction::W,
			(Direction::N, -180) => Direction::S,
			(Direction::N, -270) => Direction::E,
			(Direction::W, 90) => Direction::N,
			(Direction::W, 180) => Direction::E,
			(Direction::W, 270) => Direction::S,
			(Direction::W, -90) => Direction::S,
			(Direction::W, -180) => Direction::E,
			(Direction::W, -270) => Direction::N,
			(Direction::E, 90) => Direction::S,
			(Direction::E, 180) => Direction::W,
			(Direction::E, 270) => Direction::N,
			(Direction::E, -90) => Direction::N,
			(Direction::E, -180) => Direction::W,
			(Direction::E, -270) => Direction::S,
			(Direction::S, 90) => Direction::W,
			(Direction::S, 180) => Direction::N,
			(Direction::S, 270) => Direction::E,
			(Direction::S, -90) => Direction::E,
			(Direction::S, -180) => Direction::N,
			(Direction::S, -270) => Direction::W,
			_ => unreachable!(),
		}
	}
}

#[derive(Debug, PartialEq)]
struct State {
	position: Position,
	direction: Direction,
}

impl State {
	fn exec(&self, instruction: &Instruction) -> Self {
		match instruction.kind {
			InstructionKind::MoveNorth => State {
				direction: self.direction,
				position: Position {
					north: self.position.north + instruction.value,
					west: 0,
				},
			},
			InstructionKind::MoveSouth => State {
				direction: self.direction,
				position: Position {
					north: self.position.north - instruction.value,
					west: 0,
				},
			},
			InstructionKind::MoveEast => State {
				direction: self.direction,
				position: Position {
					north: 0,
					west: self.position.west - instruction.value,
				},
			},
			InstructionKind::MoveWest => State {
				direction: self.direction,
				position: Position {
					north: 0,
					west: self.position.west + instruction.value,
				},
			},
			InstructionKind::TurnLeft => State {
				direction: self.direction.turn(-instruction.value),
				position: Position { north: 0, west: 0 },
			},
			InstructionKind::TurnRight => State {
				direction: self.direction.turn(instruction.value),
				position: self.position,
			},
			InstructionKind::GoForward => State {
				direction: self.direction,
				position: self.position,
			},
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_exec() {
		let initial_state = State {
			position: Position { north: 0, west: 0 },
			direction: Direction::N,
		};

		let data = vec![
			// Move
			(
				Instruction {
					kind: InstructionKind::MoveNorth,
					value: 1,
				},
				State {
					position: Position { north: 1, west: 0 },
					direction: Direction::N,
				},
			),
			(
				Instruction {
					kind: InstructionKind::MoveSouth,
					value: 1,
				},
				State {
					position: Position { north: -1, west: 0 },
					direction: Direction::N,
				},
			),
			(
				Instruction {
					kind: InstructionKind::MoveWest,
					value: 1,
				},
				State {
					position: Position { north: 0, west: 1 },
					direction: Direction::N,
				},
			),
			(
				Instruction {
					kind: InstructionKind::MoveEast,
					value: 1,
				},
				State {
					position: Position { north: 0, west: -1 },
					direction: Direction::N,
				},
			),
			// value
			(
				Instruction {
					kind: InstructionKind::MoveNorth,
					value: 2,
				},
				State {
					position: Position { north: 2, west: 0 },
					direction: Direction::N,
				},
			),
			(
				Instruction {
					kind: InstructionKind::MoveSouth,
					value: 2,
				},
				State {
					position: Position { north: -2, west: 0 },
					direction: Direction::N,
				},
			),
			(
				Instruction {
					kind: InstructionKind::MoveWest,
					value: 2,
				},
				State {
					position: Position { north: 0, west: 2 },
					direction: Direction::N,
				},
			),
			(
				Instruction {
					kind: InstructionKind::MoveEast,
					value: 2,
				},
				State {
					position: Position { north: 0, west: -2 },
					direction: Direction::N,
				},
			),
			// Turn
			(
				Instruction {
					kind: InstructionKind::TurnLeft,
					value: 90,
				},
				State {
					position: Position { north: 0, west: 0 },
					direction: Direction::W,
				},
			),
			(
				Instruction {
					kind: InstructionKind::TurnRight,
					value: 90,
				},
				State {
					position: Position { north: 0, west: 0 },
					direction: Direction::E,
				},
			),
			(
				Instruction {
					kind: InstructionKind::TurnLeft,
					value: 180,
				},
				State {
					position: Position { north: 0, west: 0 },
					direction: Direction::S,
				},
			),
			(
				Instruction {
					kind: InstructionKind::TurnRight,
					value: 180,
				},
				State {
					position: Position { north: 0, west: 0 },
					direction: Direction::S,
				},
			),
			(
				Instruction {
					kind: InstructionKind::TurnLeft,
					value: 270,
				},
				State {
					position: Position { north: 0, west: 0 },
					direction: Direction::E,
				},
			),
			(
				Instruction {
					kind: InstructionKind::TurnRight,
					value: 270,
				},
				State {
					position: Position { north: 0, west: 0 },
					direction: Direction::W,
				},
			),
		];

		data.into_iter().for_each(|x| {
			let ins = x.0;

			assert_eq!(initial_state.exec(&ins), x.1);
		})
	}
}
