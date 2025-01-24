fn search(nums: &Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len() - 1;

    //search pivot point
    // 3 4 5 (0) 1 2
    while l < r {
        let m = l + (r - l) / 2;
        if nums[m] > nums[r] {
            l = m + 1;
        } else {
            r = m;
        }
    }

    let pivot = l;

    //see which side the target may be on
    if target >= nums[0] {
        l = 0;
        r = pivot - 1;
    } else {
        l = pivot;
        r = nums.len() - 1;
    }

    while l <= r {
        let m = l + (r - l) / 2;
        if nums[m] == target {
            return m as i32;
        } else if nums[m] > target {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }
    -1

}

fn main() {
    let nums = vec![3, 4, 5, 6, 1, 2];
    println!("{}", search(&nums, 4)); // 1
    println!("{}", search(&nums, 3)); // 0
    println!("{}", search(&nums, 2)); // 5
    println!("{}", search(&nums, 7)); // -1
    println!("{}", search(&nums, 0)); // -1
}
