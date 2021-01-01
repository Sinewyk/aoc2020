#[derive(Debug)]
enum Cell {
	Tree,
	Empty,
}

#[derive(Debug)]
struct Position {
	x: usize,
	y: usize,
}

#[derive(Debug)]
struct Itinerary {
	pos: Position,
	length: usize,
	max_depth: usize,
}

type Map = Vec<Vec<Cell>>;

impl Itinerary {
	fn new(map: &Map) -> Self {
		let length = map.get(0).unwrap().len();
		let max_depth = map.len();

		Itinerary {
			pos: Position { x: 1, y: 1 },
			length,
			max_depth,
		}
	}

	fn current_cell<'a>(&self, map: &'a Map) -> &'a Cell {
		return &map[self.pos.y - 1][(self.pos.x - 1) % self.length];
	}

	fn move_<'a>(&mut self, map: &'a Map, (x, y): (usize, usize)) -> Option<&'a Cell> {
		self.pos.x += x;
		self.pos.y += y;

		if self.pos.y <= self.max_depth {
			return Some(self.current_cell(map));
		} else {
			return None;
		}
	}
}

fn main() {
	let map: Map = include_str!("input.txt")
		.split('\n')
		.map(|x| -> Vec<Cell> {
			return x
				.chars()
				.map(|x| match x {
					'.' => Cell::Empty,
					'#' => Cell::Tree,
					_ => panic!("Should not go there"),
				})
				.collect::<Vec<Cell>>();
		})
		.collect::<Map>();

	part1(&map);
	part2(&map);
}

fn part1(map: &Map) {
	let count = generate_itinerary(map, (3, 1));

	println!("Tree count: {}", count);
}

fn part2(map: &Map) {
	let product = vec![(1 as usize, 1 as usize), (3, 1), (5, 1), (7, 1), (1, 2)]
		.into_iter()
		.map(|x| generate_itinerary(map, x))
		.product::<usize>();

	println!("Tree count product: {}", product);
}

fn generate_itinerary(map: &Map, (x, y): (usize, usize)) -> usize {
	let mut iti = Itinerary::new(map);
	let mut count = 0;

	while let Some(x) = iti.move_(map, (x, y)) {
		if let Cell::Tree = x {
			count += 1;
		}
	}

	return count;
}
