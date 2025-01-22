fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut l = 0;
    let mut r = nums.len() - 1;
    let mut out: Vec<Vec<i32>> = Vec::new();
    while l < r {
        while l < nums.len() - 1 && nums[l] == nums[l + 1] {
            l += 1;
        }
    }
    out
}
fn main() {
    println!("Hello, world!");
}
