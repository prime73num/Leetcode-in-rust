// 746. Min Cost Climbing Stairs
// Dynamic Programming 4
struct Solution {}
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut first = 0;
        let mut second = 0;
        let n = cost.len();
        for i in 2..=n {
            let mut temp = first + cost[i-2];
            if first + cost[i-2] > second + cost[i-1] {
                temp = second + cost[i-1];
            }
            first = second;
            second = temp;
        }
        return second;
    }
}
fn main() {
}
