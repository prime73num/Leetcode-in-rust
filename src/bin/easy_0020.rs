



// 20. Valid Parentheses
// Stack and HHeap 4




struct Solution {}
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let s = s.into_bytes();
        let mut stack = Vec::new();
        for i in s {
            match i {
                b'{' | b'[' | b'(' => {
                    stack.push(i);
                },
                b'}' => { 
                    if stack.is_empty() || stack.pop().unwrap() != b'{' {
                        return false;
                    }
                },
                b']' => {
                    if stack.is_empty() || stack.pop().unwrap() != b'[' {
                        return false;
                    }
                },
                b')' => {
                    if stack.is_empty() || stack.pop().unwrap() != b'(' {
                        return false;
                    }
                },
                _ => {}
            }
        }
        return stack.is_empty();
    }
}
fn main() {
}
