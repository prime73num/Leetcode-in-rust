// 24. Swap Nodes in Pairs
// Linked List 5

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
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut point = &mut head;
        while point.is_some() && point.as_ref().unwrap().next.is_some() {
            let mut node = point.take().unwrap();
            let mut next = node.next.take().unwrap();
            node.next = next.next.take();
            next.next = Some(node);
            *point = Some(next);
            point = &mut point.as_mut().unwrap().next.as_mut().unwrap().next;
        }
        head
    }
}

fn main() {
    println!("hello world");
}
#[cfg(test)]
mod tt {
    use super::*;
    #[test]
    fn hello() {
        println!("hello world");
    }
}
