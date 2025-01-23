fn eval_rpn(tokens: &Vec<String>) -> i32 {
    let mut stack: Vec<String> = Vec::new();
    let mut out: i32 = 0;

    for t in tokens {
        if let Ok(_) = t.parse::<i32>() {
            stack.push(t.clone());
        } else {
            let n1 = stack.pop().unwrap().parse::<i32>().unwrap();
            let n2 = stack.pop().unwrap().parse::<i32>().unwrap();
            match t.as_str() {
                "+" => {out = n2 + n1; stack.push(format!("{}", n1 + n2));},
                "-" => {out = n2 - n1; stack.push(format!("{}", n1 - n2));},
                "*" => {out = n2 * n1; stack.push(format!("{}", n1 * n2));},
                "/" => {out = n2 / n1; stack.push(format!("{}", n1 / n2));},
                _ => {unreachable!()}
            };
        }
    }
    out
}
fn main() {
    let tokens: Vec<String> = ["1","2","+","3","*","4","-"].iter().map(|t| t.to_string()).collect();
    println!("{}", eval_rpn(&tokens));
}
