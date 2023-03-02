struct Solution {}
impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let mut nines = 0;
        let mut times = 1;
        let mut left = n;
        let mut right = 0;
        let mut res = 0;
        let target = 1;
        while left != 0 {
            let ten = left/10;
            let unit = left % 10;
            res += (ten+1) * (nines+1);
            if unit <= target {
                res -= nines + 1;
            }
            if unit == target {
                res += right + 1;
            }
            left = ten;
            right += unit * times;
            nines += times * 9;
            times *= 10;
        }
        return res;
    }
}
fn main() {
}
