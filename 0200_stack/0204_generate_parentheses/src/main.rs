fn sub(n: i32, open: i32, close: i32, stack: &mut Vec<char>, out: &mut Vec<String>) {
    //base case
    if open == close && open == n {
        out.push(
            stack.iter().collect()
        );
        return;
    }
    
    if open > close {
        stack.push(')');
        sub(n, open, close + 1 , stack, out);
        stack.pop();
    }

    if open < n {
        stack.push('(');
        sub(n, open + 1, close, stack, out);
        stack.pop();
    }
}

fn generate_parentheses(n: i32) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let open = 0;
    let close = 0;
    let mut stack: Vec<char> = Vec::new();

    sub(n, open, close, &mut stack, &mut out);

    out
}

fn main() {
    println!("{:?}", generate_parentheses(1));
    println!("{:?}", generate_parentheses(2));
    println!("{:?}", generate_parentheses(3));
}
