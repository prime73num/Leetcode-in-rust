// 72. Edit Distance
// Dynamic Programming 50
struct Solution {}
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let bytes1 = word1.as_bytes();
        let bytes2 = word2.as_bytes();
        let mut dp = vec![0;bytes1.len()+1];
        for i in 1..dp.len() {
            dp[i] = i;
        }
        for i in 0..bytes2.len() {
            let mut old = dp[0];
            dp[0] += 1;
            let val = bytes2[i];
            for j in 1..dp.len() {
                let temp = dp[j];
                let mut min = dp[j-1] + 1;
                if dp[j] + 1 < min { min = dp[j] + 1; }
                let mut replace = 1;
                if bytes1[j-1] == val {
                    replace = 0;
                }
                if old + replace < min { min = old + replace; }
                dp[j] = min;
                old = temp;
            }
        }
        return dp[bytes1.len()] as i32;
    }
    pub fn min_distance_v2(word1: String, word2: String) -> i32 {
        let b1 = word1.as_bytes();
        let b2 = word2.as_bytes();
        let size1 = b1.len();
        let size2 = b2.len();
        let mut dp:Vec<usize> = (0..=size1).collect();
        for i in 0..size2 {
            let mut pre = dp[0];
            dp[0] += 1;
            let temp = b2[i];
            for j in 1..=size1 {
                let mut min = pre;
                if b1[j-1] != temp {
                    min += 1;
                }
                pre = dp[j];
                if min > dp[j] + 1 {
                    min = dp[j] + 1;
                }
                if min > dp[j-1] + 1 {
                    min = dp[j-1] + 1;
                }
                dp[j] = min;
            }
        }
        return dp[size1] as i32;
    }
}
fn main() {
}
