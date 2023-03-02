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
use std::collections::BinaryHeap;
struct Solution {}
impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        let mut res: Option<Box<ListNode>> = None;
        let mut point = &mut res;
        for i in 0..lists.len() {
            if lists[i].is_none() { continue; }
            let node = lists[i].as_mut().unwrap();
            heap.push([std::cmp::Reverse(node.val), std::cmp::Reverse(i as i32)]);
            lists[i] = std::mem::replace(&mut node.next, None);
        }
        while !heap.is_empty() {
            let min = heap.pop().unwrap();
            *point = Some(Box::new(ListNode::new(min[0].0)));
            point = &mut point.as_mut().unwrap().next;
            let idx = min[1].0 as usize;
            if lists[idx].is_none() { continue; }
            let node = lists[idx].as_mut().unwrap();
            heap.push([std::cmp::Reverse(node.val), std::cmp::Reverse(idx as i32)]);
            lists[idx] = std::mem::replace(&mut node.next, None);
        }
        res
    }
}
fn main() {
}

