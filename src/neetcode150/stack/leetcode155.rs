#![allow(dead_code)]

struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    fn new() -> Self {
        MinStack { stack: vec![] }
    }

    fn push(&mut self, val: i32) {
        if self.stack.is_empty() {
            self.stack.push((val, val));
        } else {
            let min = self.get_min().min(val);
            self.stack.push((val, min));
        }
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        if let Some(top) = self.stack.last() {
            top.0
        } else {
            -1
        }
    }

    fn get_min(&self) -> i32 {
        if let Some(top) = self.stack.last() {
            top.1
        } else {
            -1
        }
    }
}
