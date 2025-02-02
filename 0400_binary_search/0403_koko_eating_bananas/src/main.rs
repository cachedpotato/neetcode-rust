use std::cmp::{max, min};
fn min_eating_speed(pile: &Vec<u32>, h: u32) -> u32 {
	let mut l = 1;

	let r = pile
		.iter()
		.fold(pile[0], |acc, x| max(acc, *x));

	let mut min_speed: u32 = u32::MAX;

	while l < r {
		let m = l + (r - l) / 2;
		let time = pile
			.iter()
			.fold(0, |acc, x| {
				if x % m == 0 {acc + x / m}
				else {acc + x / m + 1}
			});

		if time <= h {
			min_speed = min(min_speed, m);
			break;
		} else {
			//GOTTA EAT FASTER YOUNG MAN
			l = m + 1;
			min_speed = l;
		}
	}
	min_speed
}

fn main() {
	let mut pile: Vec<u32> = vec![25, 10, 23, 4];
    println!("{}", min_eating_speed(&pile, 4));
	pile = vec![1, 4, 3, 2];
    println!("{}", min_eating_speed(&pile, 9));
}
