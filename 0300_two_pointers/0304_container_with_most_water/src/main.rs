use std::cmp::{min, max};
fn max_area(heights: &Vec<u32>) -> u32 {
    let mut l = 0;
    let mut r = heights.len() - 1;
    let mut out: u32 = 0;

    while l < r {
        let hl = heights[l];
        let hr = heights[r];
        out = max(out, (r - l) as u32 * min(hl, hr));

        if hl < hr {
            l += 1;
        } else {
            r -= 1;
        }
    }
    out
}
fn main() {
    let heights: Vec<u32> = vec![1, 7, 2, 5, 4, 7, 3, 6];
    println!("{}", max_area(&heights));
}
