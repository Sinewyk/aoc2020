use grid::Grid;
use itertools::Itertools;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
	Floor,
	Seat,
	Occupied,
}

fn main() {
	let input_as_lines: Vec<&str> = include_str!("input.txt").lines().collect();

	let initial_grid = parse(input_as_lines);

	fn process(
		predicter: impl Fn(&Grid<Cell>) -> (Grid<Cell>, bool),
		grid: &Grid<Cell>,
	) -> Grid<Cell> {
		let (next_grid, res) = predicter(&grid);
		if res == true {
			return next_grid;
		} else {
			return process(predicter, &next_grid);
		}
	}

	let final_grid = process(predict_part1, &initial_grid);

	println!(
		"part1: {}",
		final_grid
			.iter()
			.filter(|o| matches!(o, Cell::Occupied))
			.count()
	);

	let final_grid = process(predict_part2, &initial_grid);

	println!(
		"part2: {}",
		final_grid
			.iter()
			.filter(|o| matches!(o, Cell::Occupied))
			.count()
	);
}

fn parse(text: Vec<&str>) -> Grid<Cell> {
	let row_length = text[0].len();

	Grid::from_vec(
		text.iter()
			.map(|s| {
				s.chars()
					.map(|c| match c {
						'.' => Cell::Floor,
						'L' => Cell::Seat,
						'#' => Cell::Occupied,
						_ => unreachable!(),
					})
					.collect::<Vec<Cell>>()
			})
			.flatten()
			.collect(),
		row_length,
	)
}

fn predict_part1(grid: &Grid<Cell>) -> (Grid<Cell>, bool) {
	let mut next_grid = grid.clone();

	grid_enumerate(grid).for_each(|(pos, c)| match *c {
		Cell::Seat => {
			let neighboors = grid_adjacent_occupied_seats(grid, pos);
			if neighboors == 0 {
				next_grid[pos.0][pos.1] = Cell::Occupied;
			}
		}
		Cell::Occupied => {
			let neighboors = grid_adjacent_occupied_seats(grid, pos);
			if neighboors >= 4 {
				next_grid[pos.0][pos.1] = Cell::Seat;
			}
		}
		Cell::Floor => (),
	});

	let b = &next_grid == grid;

	(next_grid, b)
}

fn predict_part2(grid: &Grid<Cell>) -> (Grid<Cell>, bool) {
	let mut next_grid = grid.clone();

	grid_enumerate(grid).for_each(|(pos, c)| match *c {
		Cell::Seat => {
			let visible_seats = grid_first_visible_seats(grid, pos);
			if visible_seats == 0 {
				next_grid[pos.0][pos.1] = Cell::Occupied;
			}
		}
		Cell::Occupied => {
			let visible_seats = grid_first_visible_seats(grid, pos);
			if visible_seats >= 5 {
				next_grid[pos.0][pos.1] = Cell::Seat;
			}
		}
		Cell::Floor => (),
	});

	let b = &next_grid == grid;

	(next_grid, b)
}

fn grid_adjacent_occupied_seats(grid: &Grid<Cell>, pos: (usize, usize)) -> usize {
	(-1isize..=1)
		.map(|dx| (-1isize..=1).map(move |dy| (dx, dy)))
		.flatten()
		.filter(|&(dx, dy)| !(dx == 0 && dy == 0))
		.map(move |(dx, dy)| (pos.0 as isize + dx, pos.1 as isize + dy))
		.map(|o| {
			grid.get(
				o.0.try_into().unwrap_or(usize::MAX),
				o.1.try_into().unwrap_or(usize::MAX),
			)
		})
		.filter(|o| {
			if let Some(c) = o {
				return matches!(c, Cell::Occupied);
			} else {
				return false;
			}
		})
		.count()
}

fn grid_first_visible_seats(grid: &Grid<Cell>, pos: (usize, usize)) -> usize {
	(-1isize..=1)
		.map(|dx| (-1isize..=1).map(move |dy| (dx, dy)))
		.flatten()
		.filter(|&(dx, dy)| !(dx == 0 && dy == 0))
		.map(move |(dx, dy)| {
			itertools::iterate((pos.0 as isize, pos.1 as isize), move |new_pos| {
				(new_pos.0 + dx, new_pos.1 + dy)
			})
			.skip(1)
			.map(move |pos| {
				return grid.get(
					pos.0.try_into().unwrap_or(usize::MAX),
					pos.1.try_into().unwrap_or(usize::MAX),
				);
			})
			.while_some()
			.filter_map(|c| match c {
				Cell::Floor => None,
				seat => Some(seat),
			})
			.take(1)
		})
		.flatten()
		.filter(|c| matches!(c, Cell::Occupied))
		.count()
}

#[test]
fn test_parse() {
	assert_eq!(
		parse(
			indoc::indoc!(
				"
				L.LL
				LLLL
				L.L.
				"
			)
			.lines()
			.collect::<Vec<&str>>(),
		),
		Grid::from_vec(
			vec![
				//first rows
				Cell::Seat,
				Cell::Floor,
				Cell::Seat,
				Cell::Seat,
				// second row
				Cell::Seat,
				Cell::Seat,
				Cell::Seat,
				Cell::Seat,
				//third row
				Cell::Seat,
				Cell::Floor,
				Cell::Seat,
				Cell::Floor
			],
			4
		)
	);
}

#[test]
fn test_parse_with_occupied() {
	assert_eq!(
		parse(
			indoc::indoc!(
				"
				L.#L
				L#LL
				L#L.
				"
			)
			.lines()
			.collect::<Vec<&str>>(),
		),
		Grid::from_vec(
			vec![
				//first rows
				Cell::Seat,
				Cell::Floor,
				Cell::Occupied,
				Cell::Seat,
				// second row
				Cell::Seat,
				Cell::Occupied,
				Cell::Seat,
				Cell::Seat,
				//third row
				Cell::Seat,
				Cell::Occupied,
				Cell::Seat,
				Cell::Floor
			],
			4
		)
	);
}

#[test]
fn test_adjacent() {
	let grid = parse(
		indoc::indoc!(
			"
			L.#L
			L#LL
			L#L.
			"
		)
		.lines()
		.collect::<Vec<&str>>(),
	);

	assert_eq!(grid_adjacent_occupied_seats(&grid, (0, 0)), 1);
	assert_eq!(grid_adjacent_occupied_seats(&grid, (0, 1)), 2);
	assert_eq!(grid_adjacent_occupied_seats(&grid, (1, 2)), 3);
}

#[test]
fn test_visible() {
	let grid = parse(
		indoc::indoc!(
			"
			L.#LL
			L#L.#
			L##.#
			.....
			..L..
			"
		)
		.lines()
		.collect::<Vec<&str>>(),
	);

	assert_eq!(grid_first_visible_seats(&grid, (0, 0)), 2);
	assert_eq!(grid_first_visible_seats(&grid, (0, 1)), 2);
	assert_eq!(grid_first_visible_seats(&grid, (1, 2)), 5);
	assert_eq!(grid_first_visible_seats(&grid, (4, 2)), 2);
}

fn grid_enumerate<'a, T>(grid: &'a Grid<T>) -> impl Iterator<Item = ((usize, usize), &T)> + 'a
where
	T: Clone,
{
	let (_, col) = grid.size();

	grid.iter()
		.enumerate()
		.map(move |(i, c)| ((i / col, i % col), c))
}

#[test]
fn test_iterate() {
	let grid = parse(
		indoc::indoc!(
			"
			L.#L
			L#LL
			L#L.
			"
		)
		.lines()
		.collect::<Vec<&str>>(),
	);

	assert_eq!(
		grid_enumerate(&grid).collect::<Vec<((usize, usize), &Cell)>>(),
		vec![
			((0, 0), &Cell::Seat),
			((0, 1), &Cell::Floor),
			((0, 2), &Cell::Occupied),
			((0, 3), &Cell::Seat),
			((1, 0), &Cell::Seat),
			((1, 1), &Cell::Occupied),
			((1, 2), &Cell::Seat),
			((1, 3), &Cell::Seat),
			((2, 0), &Cell::Seat),
			((2, 1), &Cell::Occupied),
			((2, 2), &Cell::Seat),
			((2, 3), &Cell::Floor)
		]
	);
}
