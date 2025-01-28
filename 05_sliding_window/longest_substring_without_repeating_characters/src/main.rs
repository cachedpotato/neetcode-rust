use std::collections::HashSet;

fn length_of_longest_substring(s: &str) -> u8 {
    let len = s.len();
    if len == 1 {return 1;}

    let mut l = 0;
    let mut r = 0;
    let mut out = 0;

    let chars: Vec<char> = s.chars().collect();
    let mut char_set:HashSet<char> = HashSet::new();

    while r < len {
        while char_set.contains(&chars[r]) {
            char_set.remove(&chars[l]);
            l += 1;
        }
        char_set.insert(chars[r]);
        out = std::cmp::max(out, (r - l + 1) as u8);
        r += 1;
    }

    out
}

fn main() {
    println!("{}", length_of_longest_substring("zxyzxyz")); // 3
    println!("{}", length_of_longest_substring("xxxx")); // 1
    println!("{}", length_of_longest_substring("xyxxy")); // 2
}
