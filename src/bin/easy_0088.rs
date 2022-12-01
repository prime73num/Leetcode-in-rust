


// merge two sorted array




struct Solution {}



impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut end = n + m -1;
        let mut p1 = m -1;
        let mut p2 = n -1;
        while end != -1 {
            let mut temp1 = std::i32::MIN;
            let mut temp2 = std::i32::MIN;
            if p1 != -1 { temp1 = nums1[p1 as usize]; }
            if p2 != -1 { temp2 = nums2[p2 as usize]; }
            if temp1 > temp2 {
                p1 -= 1;
                nums1[end as usize] = temp1;
            } else {
                p2 -= 1;
                nums1[end as usize] = temp2;
            }
            end -= 1;
        }
    }
}



fn main() {

}
