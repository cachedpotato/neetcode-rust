use std::collections::HashMap;


fn is_valid_sudoku(board: Vec<Vec<String>>) -> bool {
    let mut rows: HashMap<usize, [bool;9]> = HashMap::new();
    let mut cols: HashMap<usize, [bool;9]> = HashMap::new();
    let mut squares: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    for r in 0..9 {
        for c in 0..9 {
            if &board[r][c] == "." {continue;}
            let num = (&board[r][c]).parse::<usize>().unwrap();

            //update
            if let Some(&arr) = rows.get(&r) {
                if arr[c] {return false;}
                else {rows.get_mut(&r).unwrap()[c] = true;}
            } else {
                rows.insert(r, [false;9]);
            }

            if let Some(&arr) = cols.get(&c) {
                if arr[r] {return false;}
                else {cols.get_mut(&c).unwrap()[r] = true;}
            } else {
                rows.insert(c, [false;9]);
            }

            if let Some(set) = squares.get(&(r, c)) {
                if set.contains(&num) {return false;}
                else {squares.get_mut(&(r, c)).unwrap().push(num);}
            } else {
                squares.insert((r, c), Vec::new());
            }
        }
    }

    true
}
fn main() {
   let board: Vec<Vec<String>> = 
   [["1","2",".",".","3",".",".",".","."].iter().map(|&s| String::from(s)).collect(),
    ["4",".",".","5",".",".",".",".","."].iter().map(|&s| String::from(s)).collect(),
    [".","9","8",".",".",".",".",".","3"].iter().map(|&s| String::from(s)).collect(),
    ["5",".",".",".","6",".",".",".","4"].iter().map(|&s| String::from(s)).collect(),
    [".",".",".","8",".","3",".",".","5"].iter().map(|&s| String::from(s)).collect(),
    ["7",".",".",".","2",".",".",".","6"].iter().map(|&s| String::from(s)).collect(),
    [".",".",".",".",".",".","2",".","."].iter().map(|&s| String::from(s)).collect(),
    [".",".",".","4","1","9",".",".","8"].iter().map(|&s| String::from(s)).collect(),
    [".",".",".",".","8",".",".","7","9"].iter().map(|&s| String::from(s)).collect()].into_iter().collect();

    println!("{}", is_valid_sudoku(board));
}