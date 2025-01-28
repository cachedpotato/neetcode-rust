use std::collections::BinaryHeap; //Max Heap
                                  //For Min Heap wrap with Reverse(n)

fn max_sliding_window(nums: &Vec<i32>, k: i32) -> Vec<i32> {
    let mut out = Vec::new();
    let mut max_heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();

    for (i, &n) in nums.iter().enumerate() {
        max_heap.push((n, i));
        if max_heap.len() < k as usize {continue;}
        while max_heap.peek().unwrap().1 as i32 <= i as i32 - k {
            max_heap.pop();
        }
        out.push(max_heap.peek().unwrap().0);
    }
    out
}
fn main() {
    let nums= vec![1, 2, 1, 0, 1, 2, 6];
    println!("{:?}", max_sliding_window(&nums, 3));
}
