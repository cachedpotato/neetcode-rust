use std::collections::HashMap;
use std::cmp::max;

fn character_replacement(s: &str, k: usize) -> usize {
    let mut map: HashMap<char, usize> = HashMap::new();
    let mut l = 0;
    let mut max_count = 0;
    let mut len = 0;
    let chars = s.chars().collect::<Vec<char>>();

    for r in 0..s.len() {
        map.entry(chars[r])
            .and_modify(|i| {*i += 1;})
            .or_insert(1);

        max_count = max(max_count, map[&chars[r]]);
        while l < r && r - l + 1 - max_count > k {
            map.entry(chars[l])
                .and_modify(|i| {*i -= 1;});
            l += 1;
        }
        len = max(len, r - l + 1);
    }

    len
}
fn main() {
    println!("{}", character_replacement("XYYX", 2));     // 4
    println!("{}", character_replacement("AAABABB", 1));  // 5
}
