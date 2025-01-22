fn two_sum(nums: &Vec<i32>, target: i32) -> Vec<usize> {
    let mut l = 0;
    let mut r = nums.len() - 1;
    let mut sum;
    while l < r {
        sum = nums[l] + nums[r];
        if sum == target {
            return vec![l + 1, r + 1]; //1 indexed
        }
        if sum < target {
            l += 1;
        }
        else {
            r -= 1;
        }
    }
    unreachable!()
}

fn main() {
    let mut nums = vec![1, 2, 3, 4];
    println!("{:?}", two_sum(&nums, 3));
    nums = vec![1, 3, 3, 4];
    println!("{:?}", two_sum(&nums, 6));
}
