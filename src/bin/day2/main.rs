use aoc_2025::*;

fn solution(input: &str) -> Output<u64> {
	let mut double_ids_sum = 0;
	let mut repeat_ids_sum = 0;

	for range in input.split(',') {
		let (lower_s, upper_s) = range.split_once('-').unwrap();

		let lower: u64 = lower_s.parse().unwrap();
		let upper: u64 = upper_s.parse().unwrap();

		for i in lower..=upper {
			let s = i.to_string();

			'repeat_id: for j in 1..s.len() {
				if s.len() % j != 0 {
					continue;
				}

				let sequence = &s[0..j];
				for k in (j..s.len()).step_by(j) {
					if &s[k..k + j] != sequence {
						continue 'repeat_id;
					}
				}

				repeat_ids_sum += i;
				break 'repeat_id;
			}

			if s.len() % 2 != 0 {
				continue;
			}
			let (left, right) = s.split_at(s.len() / 2);

			if left == right {
				double_ids_sum += i;
			}
		}
	}

	(Some(double_ids_sum), Some(repeat_ids_sum))
}

day!(solution;
	"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
		=> Some(1227775554), Some(4174379265);
);
