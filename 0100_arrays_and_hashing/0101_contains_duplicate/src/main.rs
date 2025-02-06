use std::collections::HashMap;

fn has_duplicate(nums: Vec<i32>) -> bool {
    let mut map: HashMap<i32, bool> = HashMap::new();
    for i in nums {
        if map.contains_key(&i) {
            return true;
        }
        map.insert(i, true);
    }
    false
}
fn main() {
    let nums: Vec<i32> = vec![1, 2, 3, 4];
    let nums2: Vec<i32> = vec![1, 2, 3, 3];

    println!("{:?} has duplicate: {}", nums.clone(), has_duplicate(nums));
    println!(
        "{:?} Has duplicate: {}",
        nums2.clone(),
        has_duplicate(nums2)
    );
}
