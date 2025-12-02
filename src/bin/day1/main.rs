use aoc_2025::*;

fn solution(input: &str) -> Output<i32> {
	let mut at_zero_amount = 0;
	let mut passed_zero_amount = 0;
	let mut dial_value = 50;
	for line in input.trim().lines() {
		let direction = line.get(0..1).unwrap();
		let amount: i32 = line.get(1..).unwrap().parse().unwrap();

		let distance = if direction == "L" {
			(100 - dial_value) % 100
		} else {
			dial_value
		};
		passed_zero_amount += (amount + distance) / 100;

		let multiplier = if direction == "R" { 1 } else { -1 };
		dial_value = (dial_value + amount * multiplier).rem_euclid(100);

		if dial_value == 0 {
			at_zero_amount += 1;
		}
	}
	(Some(at_zero_amount), Some(passed_zero_amount))
}

day!(solution;
	"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
	" => Some(3), Some(6);
	"
L68
L230
R48
L405
R60
L55
L1
L199
R14
L82
	" => Some(3), Some(13);
);
