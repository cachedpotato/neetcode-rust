fn search(nums: &Vec<i32>, target: i32) -> i32 {
    //STEP 1. FIND PIVOT POINT
    let mut l = 0;
    let mut r = nums.len() - 1;
    
    while l < r {
        let m = l + (r - l) / 2;
        if nums[m] < nums[r] {
            r = m;
        } else {
            l = m + 1;
        }
    }
    let pivot = l;

    //STEP 2. SEARCH
    if target >= nums[0] {
        l = 0;
        r = pivot - 1;
    } else {
        l = pivot;
        r  = nums.len() - 1;
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
    let mut nums = vec![3, 4, 5, 6, 1, 2];
    println!("{}", search(&nums, 4)); // 1
    println!("{}", search(&nums, 3)); // 0
    println!("{}", search(&nums, 2)); // 5
    nums = vec![3, 5, 6, 0, 1, 2];
    println!("{}", search(&nums, 4)); // -1
}