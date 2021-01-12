use std::{collections::HashSet, todo};

#[derive(Debug, Clone, Copy)]
enum InstructionKind {
	Acc,
	Jmp,
	Nop,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
	kind: InstructionKind,
	operand: isize,
}

type Program = Vec<Instruction>;

#[derive(Debug, Clone, Copy, Default)]
struct ProgramState {
	acc: isize,
	index: usize,
}

impl ProgramState {
	fn next(self, prog: &Program) -> Option<Self> {
		if self.index > prog.len() {
			return None;
		}

		let instruction = prog[self.index];
		Some(match instruction.kind {
			InstructionKind::Nop => Self {
				index: self.index + 1,
				..self
			},
			InstructionKind::Acc => Self {
				index: self.index + 1,
				acc: self.acc + instruction.operand,
			},
			InstructionKind::Jmp => Self {
				index: (self.index as isize + instruction.operand) as usize,
				..self
			},
		})
	}
}

fn main() {
	let input = include_str!("input.txt");

	let program = parse_program(input);

	part1(&program);
	part2(program);
}

fn part1(prog: &Program) {
	let mut set: HashSet<usize> = HashSet::new();
	let mut state: ProgramState = Default::default();

	while set.insert(state.index) {
		state = state.next(prog).unwrap();
	}

	println!("part1: {}", state.acc);
}

fn part2(program: Program) {
	todo!("Brute force it, take all of them, flip one, see which one terminates");
}

fn parse_program(input: &str) -> Program {
	input
		.lines()
		.map(|l| {
			let mut tokens = l.split(' ');
			Instruction {
				kind: match tokens.next() {
					Some(tok) => match tok {
						"acc" => InstructionKind::Acc,
						"jmp" => InstructionKind::Jmp,
						"nop" => InstructionKind::Nop,
						_ => panic!("Unknown instruction kind {}", tok),
					},
					None => panic!("for line {}, expected instruction kind", l),
				},
				operand: match tokens.next() {
					Some(tok) => tok.parse().unwrap(),
					None => panic!("for line {}, expected operand", l),
				},
			}
		})
		.collect()
}
