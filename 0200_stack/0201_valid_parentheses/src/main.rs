use std::collections::HashMap;

fn is_valid_parentheses(s: &String) -> bool {
    //stack
    let pairs: HashMap<char, char> = HashMap::from([
        (')', '('),
        ('}', '{'),
        (']', '[')
    ]);
    let mut stack: Vec<char> = Vec::new();

    //TODO: MAKE THIS PURELY FUNCTIONAL WITHOUT USING for_each()
    for c in s.chars() {
        if c == '(' || c == '[' || c == '{' {
            stack.push(c);
        } else {
            //see if top of stack is paired
            if *pairs.get(&c).unwrap() != stack[stack.len() - 1] {return false;}
            stack.pop();
        }
    }
    true
}
fn main() {
    let mut s = "([{}{}])".to_string();
    println!("{} valid: {}", &s, is_valid_parentheses(&s));
    s = "(([})".to_string();
    println!("{} valid: {}", &s, is_valid_parentheses(&s));
}
