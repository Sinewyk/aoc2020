use std::collections::HashSet;

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
		if self.index >= prog.len() {
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
	part2(&program);
}

fn part1(prog: &Program) {
	let a = exec_prog(prog);

	println!("part1: infinite loop with acc = {}", a.1);
}

fn part2(program: &Program) {
	let val = program
		.iter()
		.enumerate()
		.filter_map(|(i, ins)| match ins.kind {
			InstructionKind::Jmp | InstructionKind::Nop => Some(i),
			_ => None,
		})
		.map(|i| {
			let mut variant = program.clone();
			flip_kind(&mut variant[i].kind);
			variant
		})
		.find_map(|variant| {
			let res = exec_prog(&variant);

			match res.0 {
				true => Some(res.1),
				false => None,
			}
		})
		.unwrap();

	println!("part2: success loop with acc = {}", val);
}

fn exec_prog(prog: &Program) -> (bool, isize) {
	let mut set: HashSet<usize> = HashSet::new();
	let mut state: ProgramState = Default::default();

	while set.insert(state.index) {
		match state.next(prog) {
			Some(next_state) => state = next_state,
			None => return (true, state.acc),
		}
	}

	return (false, state.acc);
}

fn flip_kind(kind: &mut InstructionKind) {
	*kind = match *kind {
		InstructionKind::Jmp => InstructionKind::Nop,
		InstructionKind::Nop => InstructionKind::Jmp,
		x => x,
	}
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
