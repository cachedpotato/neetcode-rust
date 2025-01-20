use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<usize> {
    let mut out: Vec<usize> = vec![];
    let mut map: HashMap<i32, usize> = HashMap::new();
    let len = nums.len();

    //map: {idx, num}
    for i in 0..len {
        let complement = target - nums[i];
        if map.contains_key(&complement) {
            match i > map[&complement] {
                true => {out.push(map[&complement]); out.push(i);}
                false => {out.push(i); out.push(map[&complement]);}
            }
            return out;
        }
        map.insert(nums[i], i);
    }
    unreachable!();
}
fn main() {
    let nums = vec![4, 5, 6];
    let target = 10;

    println!("{:?}", two_sum(nums, target));
}
