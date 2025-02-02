use std::cmp::max;
fn trap(height: &Vec<u32>) -> u32 {
    let mut maxl = 0;
    let mut l = 0;
    let mut maxr = height.len() - 1;
    let mut r = height.len() - 1;
    let mut out = 0;

    while l < r {
        //move and update l and r
        if height[maxl] < height[l] {
            maxl = l;
        }
        if height[maxr] < height[r] {
            maxr = r;
        }

        //add ends
        out += max(0, height[maxl] - height[l]) + max(0, height[maxr] - height[r]);

        //update l and r
        if height[l] < height[r] {
            l += 1;
        } else {
            r -= 1;
        }
    }

    out
}

fn main() {
    let height: Vec<u32> = vec![0, 2, 0, 3, 1, 0, 1, 3, 2, 1];
    println!("{}", trap(&height));
}
