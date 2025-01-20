use std::collections::HashMap;

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    //each string turns into an array of char counts
    //change that array into string
    //create HashMap<String, Vec<String>>: count_array as key, the string as value
    let mut count_map: HashMap<String, Vec<String>> = HashMap::new();
    strs.iter()
        .for_each(|s| {
            let mut count: [usize; 24] = [0;24];
            s.chars()
                .for_each(|c| {
                    count[(c as usize) - b'a' as usize] += 1;
                });
            let key: String = count
                                .iter()
                                .fold(String::from(""), |acc, c| format!("{}{}", acc, c.to_string()));
            count_map.entry(key)
                .and_modify(|v| v.push(s.clone()))
                .or_insert(vec![s.clone()]);
        });
    
    //output: [[], []]...

    let out: Vec<Vec<String>> = count_map.into_values().collect();
    out
}
fn main() {
    let strs: Vec<String> = vec!["act".into(), "pots".into(), "tops".into(), "cat".into(), "stop".into(), "hat".into()];
    let out = group_anagrams(strs);
    println!("{:?}", out);
}
