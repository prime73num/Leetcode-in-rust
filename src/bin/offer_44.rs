struct Solution {}
impl Solution {
    pub fn find_nth_digit(mut n: i32) -> i32 {
        if n <= 9 { return n; }
        n += 1;
        let mut total = 10;
        let mut size = 1;
        let mut amount = 9;
        let mut first = 10;
        loop {
            amount *= 10;
            size += 1;
            if (n - total)/size <= amount {
                let mut target = first + (n-total-1) / size;
                let offset = size - (n-1-total) % size;
                let mut res = 0;
                for _ in 0..offset {
                    res = target % 10;
                    target /= 10;
                }
                return res;
            }
            total += amount * size;
            first *= 10;
        }
    }
}
fn main() {
    let res = Solution::find_nth_digit(189);
    println!("{}", res);
}
