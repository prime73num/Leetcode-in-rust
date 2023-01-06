// 63. Unique Paths II
// Dynamic Programming 7
struct Solution {}
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut dp = vec![0;n];
        dp[0] = 1;
        for i in 0..m {
            if obstacle_grid[i][0] == 1 { dp[0] = 0; }
            for j in 1..n {
                if obstacle_grid[i][j] == 1 { dp[j] = 0; }
                else {
                    dp[j] = dp[j] + dp[j-1];
                }
            }
        }
        return dp[n-1];
    }
}
fn main() {
}
