




















// 135. Candy
// Greedy 12
struct Solution {}
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let size = ratings.len();
        let mut count = vec![0;size];
        for i in 0..size {
            if i != 0 {
                if ratings[i] > ratings[i-1] {
                    count[i] = count[i-1] + 1;
                    continue;
                }
            }
            if i==size-1 || ratings[i+1] >= ratings[i] {
                count[i] = 1;
            }
        }
        let mut count_two = vec![0;size];

        for i in (0..size).rev() {
            if i != size-1 {
                if ratings[i] > ratings[i+1] {
                    count_two[i] = count_two[i+1] + 1;
                    continue;
                }
            }
            if i==0 || ratings[i-1] >= ratings[i] {
                count_two[i] = 1;
            }
        }
        let mut sum = 0;
        for i in 0..count.len() {
            if count[i] > count_two[i] {
                sum += count[i];
            } else {
                sum += count_two[i];
            }
        }
        println!("{:?}", count);
        println!("{:?}", count_two);
        sum
    }
}
fn main() {
    let test = vec![1,2,2];
    Solution::candy(test);
}
