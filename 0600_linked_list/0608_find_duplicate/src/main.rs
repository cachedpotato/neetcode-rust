fn find_duplicate(nums: Vec<usize>) -> usize {
    let mut slow = 0;
    let mut fast = 0;
    loop {
        slow = nums[slow];
        fast = nums[nums[fast]];

        if slow == fast {
            return slow
        }
    }
}
fn main() {
    let nums = vec![1, 2, 3, 2, 2];
    println!("{}", find_duplicate(nums));
    let nums = vec![1, 2, 3, 4, 4];
    println!("{}", find_duplicate(nums));
    let nums = vec![1, 2, 5, 3, 4, 5];
    println!("{}", find_duplicate(nums));
}
