fn min_eating_speed(piles: &Vec<usize>, h: usize) -> usize {
    let mut l = 1;
    let mut r = *piles.iter().max().unwrap();

    //lower bound -> minimum
    while l < r {
        let m = l + (r - l) / 2;
        //ceiling
        let t = piles
            .iter()
            .fold(0, |acc, &x| {
                if x % m == 0 {
                    acc + x/m
                } else {
                    acc + x/m + 1
                }
            });
        
        if  t <= h {
            r = m
        } else {
            l += 1; //incrememt by 1 to make sure
        }
    }
    l
}

fn main() {
    let mut piles = vec![1, 4, 3, 2];
    println!("{}", min_eating_speed(&piles, 9));
    piles = vec![25, 10, 23, 4];
    println!("{}", min_eating_speed(&piles, 4));
}