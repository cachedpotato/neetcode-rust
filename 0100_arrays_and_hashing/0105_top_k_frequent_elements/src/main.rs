use std::collections::HashMap;

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    //create bins and count hashmap to build said bin upon
    //bin: idx: count (len: len(nums) + 1)
    //<number, count> -> bin[count] = Vec<number>
    let mut bins: Vec<Vec<i32>> = vec![Vec::new(); nums.len() + 1];
    let mut count_map: HashMap<i32, usize> = HashMap::new();

    nums.iter()
        .for_each(|n| {
            count_map.entry(*n)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        });
    
    count_map.iter().for_each(|(k, v)| {
        bins[*v].push(*k);
    }); 

    let mut out: Vec<i32> = Vec::new();
    let mut out_count = 0;
    //TODO: NESTED FOR LOOPS TO SOMETHING PURELY FUNCTIONAL
    for vals in bins.iter().rev() {
        for n in vals {
            out.push(*n);
            out_count += 1;
            if out_count == k {return out;}
        }
    }
    unreachable!();
}

fn main() {
    let mut nums = vec![1, 2, 2, 3, 3, 3];
    let mut k = 2;
    println!("{:?}", top_k_frequent(nums, k));
    nums = vec![7, 7];
    k = 1;
    println!("{:?}", top_k_frequent(nums, k));
}
