fn find_median_sorted_arrays(nums1: &Vec<i32>, nums2: &Vec<i32>) -> f32 {
    let arr1;
    let arr2;

    //arr1 < arr2
    if nums1.len() < nums2.len() {
        arr1 = nums1.clone();
        arr2 = nums2.clone();
    } else {
        arr1 = nums2.clone();
        arr2 = nums1.clone();
    }

    let total: i32 = nums1.len() as i32 + nums2.len() as i32;
    let half: i32 = total / 2;

    let mut l: i32 = 0;
    let mut r: i32 = arr1.len() as i32 - 1;

    loop {
        let i: i32 = l + (r - l) / 2;
        let j: i32 = half - i - 2;

        let arr1_left = if i >= 0 {arr1[i as usize] as f32} else {f32::NEG_INFINITY};
        let arr1_right = if (i + 1) < arr1.len() as i32 {arr1[i as usize+ 1] as f32} else {f32::INFINITY};
        let arr2_left = if j >= 0 {arr2[j as usize] as f32} else {f32::NEG_INFINITY};
        let arr2_right = if (j + 1) < arr2.len() as i32{arr2[j as usize+ 1] as f32} else {f32::INFINITY};

        if arr1_left <= arr2_right && arr2_left <= arr1_right {
            if total % 2 != 0 {
                return arr1_right.max(arr2_right);
            }
            return (arr1_left.max(arr2_left) + arr1_right.min(arr2_right)) / 2 as f32;
        } else if arr1_left > arr2_right {
            r = i - 1;
        } else {
            l = i + 1;
        }
    }
}
fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2, 4];
    println!("{}", find_median_sorted_arrays(&nums1, &nums2));
}
