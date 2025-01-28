use std::collections::HashMap;
fn min_window<'a>(s: &'a str, t: &'a str) -> &'a str {
    let mut out = "";
    if s.len() < t.len() {return "";}
    let s_chars: Vec<char> = s.chars().collect();

    //count hashmap
    let need: HashMap<char, usize> = t.chars()
        .fold(HashMap::new(), |mut map, x| {
            map.entry(x)
                .and_modify(|i| *i += 1)
                .or_insert(1);

            map
        });

    let mut l = 0;
    for r in 0..s.len() {
        if let None = need.get(&s_chars[r]) {
            l += 1;
            continue;
        }
        let mut have:HashMap<char, usize> = need.keys()
            .fold(HashMap::new(), |mut map, x| {
                map.insert(*x, 0);
                map
            });
        let mut j = r;
        l = j;
        while j < s.len() {
            if let None = need.get(&s_chars[j]) {j += 1; continue;}
            *have.get_mut(&s_chars[j]).unwrap() += 1;
            if need == have {
                if out.len() == 0 || out.len() > s[l..r].len() {
                    out= &s[l..j + 1];
                }
                println!("{}", out);
            }
            j += 1;
        }
    }
    out
}
fn main() {
    println!("{}", min_window("OUZODYXAZV", "XYZ"));
    println!("{}", min_window("xyz", "xyz"));
    println!("{}", min_window("x", "xy"));

}
