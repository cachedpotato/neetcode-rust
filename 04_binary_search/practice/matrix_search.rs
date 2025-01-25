fn search_matrix(matrix: &Vec<Vec<i32>>, target: i32) -> bool {

    //STEP 1. ROW
    let mut top = 0;
    let mut bot = matrix[0].len();
    let mut col = 0;

    //Binary Search, just with rows
    //Rather than individual elements
    while top <= bot {
        col = top + (bot - top) / 2;
        if matrix[col][0] > target {
            bot = col - 1;
        } else if *matrix[col].last().unwrap() < target {
            top = col + 1;
        } else {
            break;
        }
    }

    if top > bot {return false;}

    //STEP 2. COL
    let mut l = 0;
    let mut r = matrix[col].len() - 1;

    while l <= r {
        let m = l + (r - l) / 2;
        if matrix[col][m] > target {
            r = m - 1;
        } else if matrix[col][m] < target {
            l = m + 1;
        } else {
            return true;
        }
    }

    return false;

}

fn main() {
    let matrix: Vec<Vec<i32>> = vec![
        vec![1,2,4,8],
        vec![10,11,12,13],
        vec![14,20,30,40]
    ];

    println!("{}", search_matrix(&matrix, 10)); //true
    println!("{}", search_matrix(&matrix, 8)); //true
    println!("{}", search_matrix(&matrix, 15)); //false
    println!("{}", search_matrix(&matrix, 20)); //true
}