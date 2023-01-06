// 70. Climbing Stairs
// Dynamic Programming 3
struct Solution {}
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut first = 0;
        let mut second = 1;
        for _ in 0..n {
            let temp = first + second;
            first = second;
            second = temp;
        }
        return first;
    }
}
fn main() {
}
