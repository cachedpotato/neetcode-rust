fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    //products -> 
    let forward: Vec<i32> = (0..nums.len())
        .map(|n| {
            nums[n+1..nums.len()]
                .iter()
                .fold(1, |acc, x| {
                    acc * x
                })
        })
        .collect();
    //products <-
    let backward: Vec<i32> = (0..nums.len())
        .map(|n| {
            nums[0..n]
                .iter()
                .fold(1, |acc, x| {
                    acc * x
                })
        })
        .collect();

    forward.iter().zip(backward.iter()).map(|(v1, v2 )| v1 * v2).collect()
}
fn main() {
    let mut nums: Vec<i32> = vec![1, 2, 4, 6];
    println!("{:?}", product_except_self(nums));
    nums = vec![-1, 0, 1, 2, 3];
    println!("{:?}", product_except_self(nums));
}
