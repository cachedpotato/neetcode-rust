struct MinStack {
	stack: Vec<i32>,
	min: Vec<i32>
}

impl MinStack {
	fn new() -> Self {
		Self {
			stack: Vec::new(),
			min: Vec::new(),
		}
	}

	fn push(&mut self, num: i32) {
		self.stack.push(num);
		if self.min.len() == 0 {self.min.push(num); return;}
		let top = self.min[self.min.len() - 1];
		match top > num {
			true => {self.min.push(num);}
			false => {self.min.push(top);}
		}
	}

	fn pop(&mut self) {
		self.stack.pop();
		self.min.pop();
	}

	fn top(&self) -> i32 {
		self.stack[self.stack.len() - 1]
	}

	fn get_min(&self) -> i32{
		self.min[self.min.len() - 1]
	}
}

fn main() {
	let mut min_stack = MinStack::new();
	min_stack.push(1);
	min_stack.push(2);
	min_stack.push(0);
	println!("{}", min_stack.get_min()); // return 0
	min_stack.pop();
	println!("{}", min_stack.top());    // return 2
	println!("{}", min_stack.get_min()); // return 1
}
