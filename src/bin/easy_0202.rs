// 202. Happy Number
// Hash Map 5

struct Solution {}
impl Solution {
    fn cal(mut num: i32) -> i32 {
        let mut res = 0;
        while num != 0 {
            let temp = num % 10;
            res += temp * temp;
            num /= 10;
        }
        res
    }
    pub fn is_happy(n: i32) -> bool {
        let mut exist_map = [0; 100];
        let mut num = n;
        while !(num < 100 && exist_map[num as usize] == 1) {
            if num < 100 {
                exist_map[num as usize] = 1;
            }
            num = Self::cal(num);
        }
        return num == 1;
    }
}
fn main() {
    Solution::is_happy(19);
}
