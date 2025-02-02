use std::collections::HashMap;

fn is_anagram(s: String, t: String) -> bool {
    let mut map1: HashMap<char, u32> = HashMap::new();
    let mut map2: HashMap<char, u32> = HashMap::new();

    s.chars()
        .for_each(|c| {
            map1.entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
    }); 

    t.chars()
        .for_each(|c| {
            map2.entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
    }); 

    s==t
}
fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("hello");
    let s3 = String::from("world");

    println!("{}", is_anagram(s1.clone(), s2));
    println!("{}", is_anagram(s1, s3));
}
