use std::ops::RangeInclusive;

use aoc_2025::*;

fn solution(input: &str) -> Output<u64> {
	let (range_lines, ingredient_lines) = input.split_once("\n\n").unwrap();

	let ranges: Vec<RangeInclusive<u64>> = range_lines
		.lines()
		.map(|line| {
			let (left, right) = line.split_once('-').unwrap();
			left.parse().unwrap()..=right.parse().unwrap()
		})
		.collect();

	let mut count = 0;
	for line in ingredient_lines.lines() {
		let id: u64 = line.parse().unwrap();

		if ranges.iter().find(|range| range.contains(&id)).is_some() {
			count += 1;
		}
	}

	let mut deduplicated_ranges: Vec<RangeInclusive<u64>> = vec![];

	let mut stack = vec![];
	for range in ranges {
		stack.push((0, range));

		'idk: while !stack.is_empty() {
			let (start, range) = stack.pop().unwrap();

			for (i, known_range) in deduplicated_ranges.iter().enumerate().skip(start) {
				if range.end() >= known_range.start() && range.start() <= known_range.end() {
					let has_lower_part = range.start() < known_range.start();
					let has_higher_part = range.end() > known_range.end();
					if has_lower_part {
						stack.push((i + 1, *range.start()..=known_range.start() - 1));
					}

					if has_higher_part {
						stack.push((i + 1, known_range.end() + 1..=*range.end()));
					}

					// Skip the rest, and let the individual parts be checked instead.
					continue 'idk;
				}
			}

			deduplicated_ranges.push(range);
		}
	}

	let deduplicated_count: u64 = deduplicated_ranges
		.iter()
		.map(|range| range.end() - range.start() + 1)
		.sum();

	(Some(count), Some(deduplicated_count))
}

day!(solution;
	"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
	" => Some(3), Some(14);
);
