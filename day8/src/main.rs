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

fn main() {
	let input = include_str!("input.txt");

	let program = parse_program(input);

	part1(&program);
}

fn part1(prog: &Program) {
	let mut set: HashSet<usize> = HashSet::new();
	let mut program_index: usize = 0;
	let mut acc: isize = 0;

	while let None = set.get(&program_index) {
		// print!("{}, {}", program_index, acc);
		set.insert(program_index);

		let instruction = prog.get(program_index).unwrap();
		match instruction {
			Instruction {
				kind: InstructionKind::Nop,
				..
			} => program_index += 1,
			Instruction {
				kind: InstructionKind::Acc,
				..
			} => {
				acc = acc + instruction.operand;
				program_index += 1;
			}
			Instruction {
				kind: InstructionKind::Jmp,
				..
			} => program_index = (program_index as isize + instruction.operand) as usize,
		}
		// println!(", {:?}", instruction);
	}

	println!(
		"part1: before executing {} a second time, the accumulator was {}",
		program_index, acc
	);
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
