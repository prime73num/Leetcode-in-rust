











// 232. Implement Queue using Stacks
// Stack and HHeap 2
struct MyQueue {
    in_stack: Vec<i32>,
    out_stack: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        Self {
            in_stack: Vec::new(),
            out_stack: Vec::new()
        }
    }
    
    fn push(&mut self, x: i32) {
        self.in_stack.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        if self.out_stack.is_empty() {
            while !self.in_stack.is_empty() {
                let temp = self.in_stack.pop().unwrap();
                self.out_stack.push(temp);
            }
        }
        self.out_stack.pop().unwrap()
    }
    
    fn peek(&mut self) -> i32 {
        if self.out_stack.is_empty() {
            while !self.in_stack.is_empty() {
                let temp = self.in_stack.pop().unwrap();
                self.out_stack.push(temp);
            }
        }
        *self.out_stack.last().unwrap()
    }
    
    fn empty(&self) -> bool {
        return self.in_stack.is_empty() && self.out_stack.is_empty();
    }
}

fn main() {
}
