fn search(nums: &Vec<i32>, target: i32) -> i32 {
	let mut l = 0 as usize;
	let mut r = nums.len() - 1;

	while l <= r {
		let m = l + (r - l) / 2;
		if nums[m] == target {
			return m as i32;
		} else if nums[m] < target {
			l = m + 1;
		} else {
			r = m - 1;
		}
	}
	return -1;
}

fn main() {
	let nums = vec![-1, 0, 2, 4, 6, 8];
	let target = 4;
    println!("{}", search(&nums, target));
}
