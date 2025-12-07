use std::collections::HashSet;

use aoc_2025::*;

fn solution(input: &str) -> Output<u64> {
	let mut lines = input.lines().step_by(2);

	let start_pos = lines.next().unwrap().find('S').unwrap();

	let mut beams = HashSet::new();
	beams.insert(start_pos);
	let mut splits = 0;

	let mut timelines = vec![0; input.find('\n').unwrap()];
	timelines[start_pos] += 1;

	for line in lines {
		for (splitter_pos, _) in line.chars().enumerate().filter(|(_, c)| *c == '^') {
			if beams.contains(&splitter_pos) {
				beams.remove(&splitter_pos);
				beams.insert(splitter_pos - 1);
				beams.insert(splitter_pos + 1);
				splits += 1;
			}

			let timeline_count = timelines[splitter_pos];
			timelines[splitter_pos] = 0;
			timelines[splitter_pos - 1] += timeline_count;
			timelines[splitter_pos + 1] += timeline_count;
		}
	}

	(Some(splits), Some(timelines.iter().sum()))
}

day!(solution;
	"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
	" => Some(21), Some(40);
);
