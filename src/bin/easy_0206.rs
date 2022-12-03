






// 206. Reverse Linked List
// Linked List 4




#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
struct Solution {}
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head : Option<Box<ListNode>> = None;
        let mut point = head;
        while point.is_some() {
            let mut node = point.unwrap();
            point = node.next.take();
            node.next = new_head;
            new_head = Some(node);
        }
        new_head
    }
}

fn main() {
}
