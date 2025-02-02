fn is_palindrome(s: &String) -> bool {
    let v: Vec<char> = s.to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();

    let l = v.len();
    for i in 0..(l/2 as usize) {
        if v[i] != v[l - 1 - i] {
            return false;
        }
    }
    true
}
fn main() {
    let mut s = "Was it a car or a cat I saw?".to_string();
    println!("valid palindrome: {}", is_palindrome(&s));
    s = "a".to_string();
    println!("valid palindrome: {}", is_palindrome(&s));

}
