fn main() {
	let input_as_lines: Vec<&str> = include_str!("input.txt").lines().collect();
	let starting_minutes = input_as_lines.get(0).unwrap();
	println!("{}", starting_minutes);
	let bus_lines = input_as_lines
		.get(1)
		.unwrap()
		.split(',')
		.filter(|x| *x != "x")
		.map(str::parse::<usize>)
		.map(Result::unwrap)
		.collect::<Vec<usize>>();

	bus_lines.iter().for_each(|x| println!("{}", x));
}

#[cfg(test)]
mod tests {
	use super::*;
}
