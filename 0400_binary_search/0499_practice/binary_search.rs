fn binary_search_iter(nums: &Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len() - 1;

    while l <= r {
        let m = l + (r - l) / 2;
        if nums[m] > target {
            r = m - 1;
        } else if nums[m] < target {
            l = m + 1;
        } else {
            return m as i32;
        }
    }
    -1
}

fn binary_search_lower_bound(nums: &Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len();

    while l < r { //if using bound approach: l < r / if iterative: l <= r
        let m = l + (r - l) / 2;
        if nums[m] >= target {
            r = m;
        } else {
            l = m + 1;
        }
    }

    if l < nums.len() && nums[l] == target {return l as i32;} else {return -1;}
}


fn binary_search_upper_bound(nums: &Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len();

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
    let nums = vec![1, 2, 5, 8, 10];
    println!("{}", binary_search_iter(&nums, 8));
    println!("{}", binary_search_upper_bound(&nums, 8));
    println!("{}", binary_search_lower_bound(&nums, 10));
}