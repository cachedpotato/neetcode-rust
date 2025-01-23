fn search_matrix(matrix: &Vec<Vec<i32>>, target: i32) -> bool {
	//row
	let mut top = 0;
	let mut bot = matrix.len() - 1;
	let mut row = top + (bot - top) / 2;

	//row
	while top <= bot {
		row = top + (bot - top) / 2;
		if &target > matrix[row].last().unwrap() {
			top = row + 1
		} else if target < matrix[row][0] {
			bot = row - 1;
		} else {
			break;
		}
	}

	if top > bot {return false;}

	//col
	let mut l = 0;
	let mut r = matrix[row].len() - 1;
	let mut m = 0;
	while l < r {
		m = l + (r - l) / 2;
		if matrix[row][m] == target {
			return true;
		} else if matrix[row][m] < target {
			l = m + 1;
		} else {
			r = m;
		}
	}
	matrix[row][m] == target
	
}

fn main() {
	let matrix = vec![
		vec![1,2,4,8],
		vec![10,11,12,13],
		vec![14,20,30,40],
	];
    println!("{}", search_matrix(&matrix, 11));
    println!("{}", search_matrix(&matrix, 13));
    println!("{}", search_matrix(&matrix, 15));
}
