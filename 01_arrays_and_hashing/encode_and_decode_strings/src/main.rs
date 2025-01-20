fn encoder(strs: Vec<String>) -> String {
    //encode both length and separator
    //ex) ["cars", "cat"] > 4#cars3#cat
    let out: String = strs.into_iter()
        .fold(String::from(""), |acc, x| {
            format!("{}{}#{}", acc, x.len(), x.clone())
        });
    out
}
fn decoder(s: String) -> Vec<String> {
    let mut idx = 0 as usize;
    let chars: Vec<char> = s.chars().collect();
    let mut out: Vec<String> = Vec::new();
    //TODO: REPLACE WHILE LOOP TO MAKE IT PURELY FUNCTIONAL
    while idx < s.len() {
        let mut word_len = String::from("");
        while chars[idx] != '#' {
            word_len.push(chars[idx]);
            idx += 1;
        }
        let l = word_len.parse::<usize>().unwrap();
        //currently s[idx] = '#'
        let word: String = s[idx + 1..idx + 1 + l].to_string();
        idx += 1 + l;
        out.push(word);
    }
    out
}
fn main() {
    let input: Vec<String> = vec!["neet".into(), "#code".into(), "love#".into(), "#you#".into()];
    let encoded = encoder(input);
    println!("{}", encoded);
    println!("{:?}", decoder(encoded));
}
