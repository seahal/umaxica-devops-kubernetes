// 225. Implement Stack using Queues

fn main() {
    let mut obj = MyStack::new();
    obj.push(1);
    obj.push(2);
    obj.pop();
    obj.empty();
}

struct MyStack {
    x: Vec<i32>,
}

impl MyStack {
    fn new() -> Self {
        Self { x: vec![] }
    }

    fn push(&mut self, x: i32) {
        self.x.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.x.pop().unwrap()
    }

    fn top(&self) -> i32 {
        let y = &self.x;
        *y.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.x.is_empty()
    }
}
