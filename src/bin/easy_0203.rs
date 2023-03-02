// 203. Remove Linked List Elements
// Linked List 2

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution {}
impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        todo!()
    }
}

fn main() {}

#[cfg(test)]
mod test {
    #[test]
    fn aaa() {
        let a = "aaah";
        println!("{}", a);
    }
}
