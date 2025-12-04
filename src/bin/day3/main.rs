use aoc_2025::*;

fn solve_with_length(line: &str, length: usize) -> u64 {
	let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();

	let mut joltage = vec![0; length];

	for (i, digit) in digits.iter().enumerate() {
		// If there are n < length digits left, only touch the last n digits.
		for j in i.saturating_sub(digits.len() - length)..length {
			// Find the first digit in the current value less than this digit. Joltage digits
			// default to zero, so if no digits from the battery bank match, it will match on the
			// first zero instead.
			if *digit > joltage[j] {
				// Overwrite, and reset all digits after to zero. This prevents previous digits from
				// turning up after this one in the output.
				joltage[j] = *digit;
				joltage[j + 1..].fill(0);
				break;
			}
		}
	}

	// Convert it all back into a number
	let mut result: u64 = 0;
	for (i, digit) in joltage.iter().enumerate() {
		result += *digit as u64 * 10u64.pow((length - i - 1) as u32);
	}
	result
}

fn solution(input: &str) -> Output<u64> {
	let mut total_two = 0;
	let mut total_twelve = 0;
	for line in input.lines() {
		let mut first = 0;
		let mut second = 0;
		for c in line.chars() {
			let digit: u64 = c.to_digit(10).unwrap().into();
			if second > first || second == first && digit > second {
				first = second;
				second = digit;
			} else if digit > second {
				second = digit;
			}
		}
		total_two += first * 10 + second;
		total_twelve += solve_with_length(line, 12);
	}

	(Some(total_two), Some(total_twelve))
}

day!(solution;
	"
987654321111111
811111111111119
234234234234278
818181911112111
	" => Some(357), Some(3121910778619);
	"
3434845634454364546334335333448443354324533545443235414334477424442444346844344244444434445333344314
	" => None, Some(888533344314);
);
