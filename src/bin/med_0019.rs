








// 19. Remove Nth Node From End of List
// Linked List 6



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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut point = &mut head;
        let mut sum = 0;
        while point.is_some() {
            sum += 1;
            point = &mut point.as_mut().unwrap().next;
        }
        point = &mut head;
        for _ in 0..sum-n {
            point = &mut point.as_mut().unwrap().next;
        }
        let to_del = point.take();
        *point = to_del.unwrap().next;
        head
    }
}

fn main() {
}
