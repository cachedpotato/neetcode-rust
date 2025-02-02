//REVIEW
fn find_min(nums: &Vec<i32>) -> i32 {
    if nums[0] < *nums.last().unwrap() {return nums[0];} //0 rotation

    let mut l = 0;
    let mut r = nums.len() - 1;

    //4 5 6 (1) 2 3
    while l < r {
        let m = l + (r - l) / 2;
        if nums[m] < nums[r] {
            r = m;
        } else {
            l = m + 1;
        }
    }
    nums[l]

}
fn main() {
    let nums = vec![3, 4, 5, 6, 1, 2];
    println!("{}", find_min(&nums));
}
