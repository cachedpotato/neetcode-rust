use std::collections::HashSet;

fn longest_consecutive(nums: Vec<i32>) -> u16 {
    let mut set = HashSet::new();
    //fimut nd starters
    let mut starter: Vec<i32> = vec![];

    nums.iter()
        .for_each(|i| {
            if !set.contains(i) {set.insert(*i);}
        });
    
    nums.iter()
        .for_each(|&i| {
            if !set.contains(&(i - 1)) {starter.push(i)}
        });
    
    let mut max = 0;
    starter.iter().for_each(|s| {
        let mut l = 0;
        let mut num = *s;
        while set.contains(&num) {
            l += 1;
            num += 1;
        }
        max = if max > l {max} else {l};
    });
    max
}
fn main() {
    let mut nums = vec![2, 20, 4, 10, 3, 4, 5];
    println!("{}", longest_consecutive(nums));
    nums = vec![0,3,2,5,4,6,1,1];
    println!("{}", longest_consecutive(nums));
}
