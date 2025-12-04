use std::fmt::{Debug, Display};

#[macro_export]
macro_rules! day {
	($solution:expr; $($input:literal => $part1:expr, $part2:expr);* ;) => {
		fn main() {
			__run_solution($solution, &[$( ($input, $part1, $part2) ),*], include_str!("input.txt"));
		}
	};
}

pub fn __run_solution<T: Debug + Display + PartialEq, F: Fn(&str) -> (Option<T>, Option<T>)>(
	solution: F,
	tests: &[(&str, Option<T>, Option<T>)],
	input: &str,
) {
	let mut part1_failures = 0;
	let mut part2_failures = 0;
	for (i, (test_input, part1_expected, part2_expected)) in tests.iter().enumerate() {
		let (part1_actual, part2_actual) = solution(test_input.trim());

		if let Some(expected) = part1_expected
			&& let Some(actual) = part1_actual
			&& *expected != actual
		{
			part1_failures += 1;
			println!(
				"(part1) Test {}: expected {expected:?}, got {actual:?}",
				i + 1
			);
		}
		if let Some(expected) = part2_expected
			&& let Some(actual) = part2_actual
			&& *expected != actual
		{
			part2_failures += 1;
			println!(
				"(part2) Test {}: expected {expected:?}, got {actual:?}",
				i + 1
			);
		}
	}

	let (part1, part2) = solution(input.trim());
	if let Some(part1) = part1
		&& part1_failures == 0
	{
		println!("part1 => {part1}");
	}
	if let Some(part2) = part2
		&& part2_failures == 0
	{
		println!("part2 => {part2}");
	}
}

pub type Output<T> = (Option<T>, Option<T>);
