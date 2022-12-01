














// Square a sorted array





struct Solution {}
impl Solution {
    pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
        let mid = nums.iter().enumerate().find(|(_, y)| {
            return **y >= 0;
        }).map_or(nums.len(), |(x,_)| {x});
        let left = nums[..mid].to_vec();
        let mut p1 = (mid -1) as i32;
        let mut p2 = mid as i32;
        for i in 0..nums.len() {
            let mut temp1 = std::i32::MAX;
            let mut temp2 = std::i32::MAX;
            if p1 >= 0 {
                temp1 = -left[p1 as usize];
            }
            if p2 < nums.len() as i32 {
                temp2 = nums[p2 as usize];
            }
            if temp2 < temp1 {
                nums[i] = temp2*temp2;
                p2 += 1;
            } else {
                nums[i] = temp1*temp1;
                p1 -= 1;
            }
        }
        nums
    }
}
fn main() {
}
