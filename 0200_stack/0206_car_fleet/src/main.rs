fn car_fleet(target: i32, position: &Vec<i32>, speed: &Vec<i32>) -> i32 {
    //target = speed *x + pos

    //[(arrival time, car idx), (), ...]
    let arrival_time: Vec<(i32, usize)> = (0..position.len())
        .map(|i| (target - position[i]) / speed[i])
        .zip((0..position.len()).collect::<Vec<usize>>())
        .collect();


    

}

fn main() {
    println!("Hello, world!");
}
