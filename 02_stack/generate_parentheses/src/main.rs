fn sub(n: i32, open: i32, close: i32, curr: String, out: &mut Vec<String>) {
    //println!("{}", curr);
    //TODO: CLEAN THIS SHIT UP WOW THIS IS HORRIBLE
    if close == n {
        out.push(curr.clone());
    }
    else {
        //close < n
        if open > close {

            if open < n {
                sub(n, open + 1, close, (curr.clone() + "(").to_owned(), out);
            }

            //closing possible
            sub(n, open, close + 1, (curr.clone() + ")").to_owned(), out);

        }
        else {
            //open == close && close < n
            sub(n, open + 1, close, (curr.clone() + "(").to_owned(), out);
        }
    }
}

fn generate_parentheses(n: i32) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let open = 0;
    let close = 0;
    let curr = String::from("");

    sub(n, open, close, curr, &mut out);

    out
}
fn main() {
    println!("{:?}", generate_parentheses(1));
    println!("{:?}", generate_parentheses(2));
    println!("{:?}", generate_parentheses(3));
}
