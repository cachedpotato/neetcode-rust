fn search_iter(nums: &Vec<i32>, target: i32) -> i32 {
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

// m]
fn search_lower_bound(nums: &Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len() - 1;

    while l < r {
        let m = l + (r - l) / 2;
        if nums[m] >= target {
            r = m;
        } else {
            l = m + 1;
        }
    }

    if l < nums.len() && nums[l] == target {return l as i32;} else {return -1;}
}

// m)
fn search_upper_bound(nums: &Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len() - 1;

    while l < r {
        let m = l + (r - l) / 2;
        if nums[m] > target {
            r = m;
        } else {
            l = m + 1;
        }
    }

    if l > 0 && nums[l - 1] == target {return (l - 1) as i32;} else {return -1;}
}

fn main() {
	let nums = vec![-1, 0, 2, 4, 6, 8];
	let target = 4;
    println!("{}", search_iter(&nums, target));
    println!("{}", search_upper_bound(&nums, target));
    println!("{}", search_lower_bound(&nums, target));
}
