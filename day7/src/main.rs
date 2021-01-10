use std::collections::HashMap;

type BagSpec<'a> = (&'a str, &'a str);
type Rules<'a> = HashMap<BagSpec<'a>, Option<Vec<(usize, BagSpec<'a>)>>>;

fn main() {
	let input = include_str!("input.txt");

	let rules = parse_rules(&input);

	dbg!(rules);
}

fn parse_rules(input: &str) -> Rules<'_> {
	let mut rules = Default::default();

	peg::parser! {
		grammar parser() for str {
			pub(crate) rule root(r: &mut Rules<'input>) = (line(r) "." whitespace()*)* ![_]

			rule line(r: &mut Rules<'input>)
				= spec:bag_spec() " contain " rules:rules() {
						r.insert(spec, rules);
				}

			rule bag_spec() -> BagSpec<'input>
			= adjective:name() " " color:name() " bag" "s"? { (adjective, color) }

			rule rules() -> Option<Vec<(usize, BagSpec<'input>)>>
					= rules:rule1()+ { Some(rules) }
					/ "no other bags" { None }

			/// Rule followed by an optional comma and space
			rule rule1() -> (usize, BagSpec<'input>)
					= r:rule0() ", "? { r }

			/// A single rule
			rule rule0() -> (usize, BagSpec<'input>)
					= quantity:number() " " spec:bag_spec() { (quantity, spec) }

			rule number() -> usize
					= e:$(['0'..='9']+) { e.parse().unwrap() }

			/// A sequence of non-whitespace characters
			rule name() -> &'input str
					= $((!whitespace()[_])*)

			/// Spaces, tabs, CR and LF
			rule whitespace()
					= [' ' | '\t' | '\r' | '\n']
		}
	}

	parser::root(input, &mut rules).unwrap();
	rules
}
