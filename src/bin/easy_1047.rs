









// 1047. Remove All Adjacent Duplicates In String
// Stack and HHeap 5






struct Solution {}
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let s = s.into_bytes();
        let mut stack = Vec::new();
        for i in s {
            if !stack.is_empty() && *stack.last().unwrap() == i {
                stack.pop();
            } else {
                stack.push(i);
            }
        }
        String::from_utf8(stack).unwrap()
    }
}

fn main() {
}
