use grid::Grid;

#[derive(Debug, Clone, PartialEq, Eq)]
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

	fn process(grid: Grid<Cell>) -> Grid<Cell> {
		let (next_grid, res) = predict(&grid);
		if res == true {
			return next_grid;
		} else {
			return process(next_grid);
		}
	}

	let final_grid = process(initial_grid);

	println!(
		"part1: {}",
		final_grid
			.iter()
			.filter(|o| matches!(o, Cell::Occupied))
			.count()
	)
}

fn predict(grid: &Grid<Cell>) -> (Grid<Cell>, bool) {
	let mut next_grid = grid.clone();

	let size = &grid.size();

	for i in 0..(size.0) {
		for j in 0..(size.1) {
			match grid[i][j] {
				Cell::Seat => {
					let neighboors = grid_adjacent_occupied_seats(grid, &(i, j), size);
					if neighboors == 0 {
						next_grid[i][j] = Cell::Occupied;
					}
				}
				Cell::Occupied => {
					let neighboors = grid_adjacent_occupied_seats(grid, &(i, j), size);
					if neighboors >= 4 {
						next_grid[i][j] = Cell::Seat;
					}
				}
				Cell::Floor => (),
			}
		}
	}

	let b = &next_grid == grid;

	(next_grid, b)
}

fn grid_adjacent_occupied_seats(
	grid: &Grid<Cell>,
	pos: &(usize, usize),
	size: &(usize, usize),
) -> usize {
	// j_range is moved into first i_range loop if we keep range, because automatic into_iter by default for "for" loops ...
	// and range doesn't implement iter apparently ?
	// so collect into a vector to iter
	let i_range = ((if let Some(x) = pos.0.checked_sub(1) {
		x
	} else {
		0
	})..=std::cmp::min(pos.0 + 1, size.0 - 1))
		.collect::<Vec<usize>>();
	let j_range = ((if let Some(x) = pos.1.checked_sub(1) {
		x
	} else {
		0
	})..=std::cmp::min(pos.1 + 1, size.1 - 1))
		.collect::<Vec<usize>>();

	let mut count = 0;
	for i in i_range.iter() {
		for j in j_range.iter() {
			if i == &pos.0 && j == &pos.1 {
				continue;
			}
			if let Cell::Occupied = grid[*i][*j] {
				count += 1;
			}
		}
	}

	count
}
