#[derive(Debug, Clone, Copy)]
enum InstructionKInd {
	Acc,
	Jmp,
	Nop,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
	kind: InstructionKInd,
	operand: isize,
}

type Program = Vec<Instruction>;

fn main() {
	let input = include_str!("input.txt");

	let program = parse_program(input);

	dbg!(&program[0..2]);
}

fn parse_program(input: &str) -> Program {
	input
		.lines()
		.map(|l| {
			let mut tokens = l.split(' ');
			Instruction {
				kind: match tokens.next() {
					Some(tok) => match tok {
						"acc" => InstructionKInd::Acc,
						"jmp" => InstructionKInd::Jmp,
						"nop" => InstructionKInd::Nop,
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
