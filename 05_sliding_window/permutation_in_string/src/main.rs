fn check_inclusion(s1: &str, s2: &str) -> bool {
    if s1.len() > s2.len() {return false;}

    let mut count_arr: [usize; 27] = [0; 27];
    let perm_arr: [usize; 27] = 
        s1.chars()
            .fold([0; 27], |mut acc, x| {
                acc[x as usize - b'a' as usize] += 1;
                acc
        });

    let mut l = 0;
    let mut r = 0;
    let len = s2.len();
    let s2_chars = s2.chars().collect::<Vec<char>>();

    while r < len {
        let r_idx = s2_chars[r] as usize - 97; //b'a' = 97
        let l_idx = s2_chars[l] as usize - 97;
        if (r - l + 1) < s1.len() {
            count_arr[r_idx] += 1;
            r += 1;
            continue;
        }
        count_arr[r_idx] += 1;
        if count_arr == perm_arr {return true;}
        r += 1;
        count_arr[l_idx] -= 1;
        l += 1;
    }
    false
}
fn main() {
    println!("{}", check_inclusion("abc", "lecaabee"));
}
