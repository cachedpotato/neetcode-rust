fn find_min(nums: &Vec<i32>) -> i32 {
    if nums[0] < *nums.last().unwrap() {return nums[0]} //early return

    let mut l = 0;
    let mut r = nums.len() - 1;

    while l < r {
        let m = l + (r - l) / 2;
        if nums[m] > nums[r] {
            l = m + l;
        } else {
            r = m;
        } 
    }
    return nums[l];
}

fn main() {
    let mut nums = vec![3, 4, 5, 6, 1, 2];
    println!("{}", find_min(&nums));
    nums = vec![4, 5, 0, 1, 2, 3];
    println!("{}", find_min(&nums));
    nums = vec![4, 5, 6, 7];
    println!("{}", find_min(&nums));
}