// 474. Ones and Zeroes
// Dynamic Programming 17
struct Solution {}
impl Solution {
    fn count(s: &str) -> (i32, i32) {
        let bytes = s.as_bytes();
        let mut zeros = 0;
        for i in bytes {
            if *i == b'0' { zeros += 1; }
        }
        return (zeros, bytes.len() as i32 -zeros);
    }
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0;n as usize + 1];m as usize + 1];
        for idx in 0..strs.len() {
            let (zeros, ones) = Self::count(&strs[idx]);
            for i in (zeros..=m).rev() {
                for j in (ones..=n).rev() {
                    let temp = dp[(i-zeros) as usize][(j-ones) as usize];
                    if temp + 1 > dp[i as usize][j as usize] {
                        dp[i as usize][j as usize] = temp + 1;
                    }
                }
            }
        }
        return dp[m as usize][n as usize];
    }
}
fn main() {
}
