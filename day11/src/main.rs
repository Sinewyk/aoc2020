use grid::Grid;

#[derive(Debug, Clone)]
enum Cell {
	Floor,
	Seat,
	Occupied,
}

fn main() {
	let input_as_lines: Vec<&str> = include_str!("input.txt").lines().collect();

	let row_length = input_as_lines[0].len();

	let initial_grid = Grid::from_vec(
		input_as_lines
			.iter()
			.map(|s| {
				s.chars()
					.map(|c| match c {
						'.' => Cell::Floor,
						'L' => Cell::Seat,
						_ => unreachable!(),
					})
					.collect::<Vec<Cell>>()
			})
			.flatten()
			.collect(),
		row_length,
	);
}
