use aoc_2025::*;

static OFFSETS: &[(i8, i8)] = &[
	(-1, -1),
	(0, -1),
	(1, -1),
	(-1, 0),
	(1, 0),
	(-1, 1),
	(0, 1),
	(1, 1),
];

fn find_accessible_cells(grid: &[Vec<bool>]) -> Vec<(usize, usize)> {
	let mut cells = vec![];

	let width = grid[0].len();
	let height = grid.len();

	for x in 0..width {
		for y in 0..height {
			if !grid[y][x] {
				continue;
			}

			let mut neighbor_count = 0;

			for (offset_x, offset_y) in OFFSETS {
				// Ignore out of bounds neighbors.
				if let Some(neighbor_x) = x.checked_add_signed(*offset_x as isize)
					&& neighbor_x < width
					&& let Some(neighbor_y) = y.checked_add_signed(*offset_y as isize)
					&& neighbor_y < height
					&& grid[neighbor_y][neighbor_x]
				{
					neighbor_count += 1;
				};
			}

			if neighbor_count < 4 {
				cells.push((x, y));
			}
		}
	}

	cells
}

fn solution(input: &str) -> Output<usize> {
	#[rustfmt::skip]

	let mut grid: Vec<Vec<bool>> = input
		.lines()
		.map(|line| line.chars().map(|c| c == '@').collect())
		.collect();

	let initial_cells = find_accessible_cells(&grid);

	let mut total = 0;
	loop {
		let cells = find_accessible_cells(&grid);
		if cells.is_empty() {
			break;
		}

		total += cells.len();

		for (x, y) in cells {
			grid[y][x] = false;
		}
	}

	(Some(initial_cells.len()), Some(total))
}

day!(solution;
	"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
	" => Some(13), Some(43);
);
