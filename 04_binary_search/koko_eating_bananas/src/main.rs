use std::cmp::{max, min};
fn min_eating_speed(pile: &Vec<u32>, h: u32) -> u32 {
	let mut l = pile
		.iter()
		.fold(pile[0], |acc, x| min(acc, *x));

	let mut r = pile
		.iter()
		.fold(pile[0], |acc, x| max(acc, *x));

	let mut out: u32 = u32::MAX;

	while l < r {
		let m = l + (r - l) / 2;
		let time = pile
			.iter()
			.fold(0, |acc, x| {
				if x % m == 0 {acc + x / m}
				else {acc + x / m + 1}
			});

		println!("{}", time);

		if time > h {
			r -= 1;
		} else {
			l = m;
			out = min(m, out);
		}
	}
	out
}

fn main() {
	let pile: Vec<u32> = vec![25, 10, 23, 4];
	let h = 4;
    println!("{}", min_eating_speed(&pile, h));
}
