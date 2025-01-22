fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    //n l r
    let mut n = 0;
    let mut l = 1;
    let mut r = nums.len() - 1;
    let mut out: Vec<Vec<i32>> = Vec::new();

    while n < nums.len() {
        let vn = nums[n];
        //fixed n
        while l < r {
            if nums[n] + nums[l] + nums[r] == 0 {
                out.push(vec![nums[n], nums[l], nums[r]]);
                l += 1;
                r -= 1;
                while l < r && nums[l - 1] == nums[l] {
                    l += 1;
                }

            } else if nums[n] + nums[l] + nums[r] > 0 {
                r -= 1;
            } else {
                l += 1;
            }
        }

        //move anchor point (n)
        while n < nums.len() && nums[n] == vn {
            n += 1;
        }
        //reset l r
        l = n + 1;
        r = nums.len() - 1;
    }

    out
}
fn main() {
    let nums = vec![-1, 0, 1, 2, -1, 4];
    println!("{:?}", three_sum(nums));
}
