use aoc_2025::*;

fn solution(input: &str) -> Output<u64> {
	let mut grid: Vec<Vec<_>> = input
		.lines()
		.map(|line| line.split(' ').filter(|s| !s.is_empty()).collect())
		.collect();

	let operations = grid.pop().unwrap();

	let mut part1_total = 0;
	for (i, operation) in operations.iter().enumerate() {
		let iter = grid.iter().map(|row| row[i].parse::<u64>().unwrap());
		part1_total += if *operation == "+" {
			iter.sum::<u64>()
		} else {
			iter.product()
		};
	}

	let mut lines: Vec<_> = input.lines().collect();
	let mut part2_total = 0;

	for (start_index, operation) in lines.pop().unwrap().chars().enumerate().peekable() {
		if operation == ' ' {
			continue;
		}

		let mut values: Vec<u64> = vec![];

		for j in start_index.. {
			let column: String = lines
				.iter()
				.map(|line| line.chars().nth(j).unwrap_or(' '))
				.collect();

			if column.trim().is_empty() {
				break;
			}

			values.push(column.trim().parse().unwrap());
		}

		part2_total += if operation == '+' {
			values.iter().sum::<u64>()
		} else {
			values.iter().product()
		};
	}

	(Some(part1_total), Some(part2_total))
}

day!(solution;
	"
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
	" => Some(4277556), Some(3263827);
);
