use std::collections::HashMap;

type BagSpec<'a> = (&'a str, &'a str);
type Rules<'a> = HashMap<BagSpec<'a>, Option<Vec<(usize, BagSpec<'a>)>>>;

fn main() {
	let input = include_str!("input.txt");
	let rules = parse_rules(&input);

	let needle = &("shiny", "gold");

	part1(&rules, needle);
	part2(&rules, needle);
}

fn part1(rules: &Rules, needle: &(&str, &str)) {
	let len: Vec<_> = rules
		.keys()
		.filter(|&k| k != needle)
		.filter(|&k| subgraph_contains(&rules, k, needle))
		.collect();

	println!("part1: {}", len.len());
}

fn part2(rules: &Rules, needle: &(&str, &str)) {
	let sum = bag_quantities(rules, needle);

	println!("part2: {}", sum);
}

fn bag_quantities(graph: &Rules<'_>, root: &(&str, &str)) -> usize {
	if let Some(Some(neighboors)) = graph.get(root) {
		return neighboors
			.iter()
			.map(|&(qt, n)| qt + qt * bag_quantities(graph, &n))
			.sum();
	}
	0
}

fn subgraph_contains(graph: &Rules<'_>, root: &(&str, &str), needle: &(&str, &str)) -> bool {
	if let Some(Some(neighbors)) = graph.get(root) {
		for (_, neighbor) in neighbors {
			if needle == neighbor || subgraph_contains(graph, neighbor, needle) {
				return true;
			}
		}
	}
	false
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
