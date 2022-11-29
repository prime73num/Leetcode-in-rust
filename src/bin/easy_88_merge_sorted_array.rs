







struct Solution {}



impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut end = n + m -1;
        let mut p1 = m -1;
        let mut p2 = n -1;
        while end != -1 {
            let mut target = 0;
            if p1 != -1 {
                target = 1;
            }
            if p2 != -1 {
                if target == 0 || nums1[p1 as usize] < nums2[p2 as usize] {
                    target = 2;
                }
            }
            match target {
                1 => {
                    nums1[end as usize] = nums1[p1 as usize];
                    p1 = p1 - 1;
                },
                2 => {
                    nums1[end as usize] = nums2[p2 as usize];
                    p2 = p2 -1;
                },
                _ => {}
            }
            end -= 1;
        }
    }
}



fn main() {

}
