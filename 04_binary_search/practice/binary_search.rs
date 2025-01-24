fn binary_search_iter(nums: &Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = 0;

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
    let mut r = 0;

    while l <= r {
        let m = l + (r - l) / 2;
    }
}


fn binary_search_upper_bound(nums: &Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = 0;

    while l <= r {
        let m = l + (r - l) / 2;
    }
}

fn main() {
    let nums = vec![1, 2, 5, 8, 10];
    println!("{}", binary_search_iter(&nums, 4))

}