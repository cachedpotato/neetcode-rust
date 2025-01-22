//Need Review
fn daily_temperatures(temperatures: &Vec<u8>) -> Vec<u8> {
    //using stacks
    let mut out: Vec<u8> = vec![0;temperatures.len()];
    let mut stack: Vec<(u8, usize)> = Vec::new(); //(temp, idx) pair

    temperatures.iter().enumerate()
        .for_each(|(i, &t)| {
            while !stack.is_empty() && t > stack.last().unwrap().0 {
                let (_top_temp, top_idx ) = stack.pop().unwrap();
                out[top_idx] = i as u8 - top_idx as u8;
            }
            stack.push((t, i));
        });

    out
}
fn main() {
    let mut temperatures: Vec<u8> = vec![30, 38, 30, 36, 35, 40, 28];
    println!("{:?}", daily_temperatures(&temperatures));
    temperatures = vec![22, 21, 20];
    println!("{:?}", daily_temperatures(&temperatures));
}
