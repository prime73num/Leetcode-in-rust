














// 455. Assign Cookies
// Greedy 2






struct Solution {}
impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();
        let mut sum = 0;
        while !s.is_empty() && !g.is_empty() {
            let greedy = g.pop().unwrap();
            let size = s.last().unwrap();
            if *size >= greedy {
                s.pop();
                sum += 1;
            }
        }
        sum
    }
}
fn main() {
}
