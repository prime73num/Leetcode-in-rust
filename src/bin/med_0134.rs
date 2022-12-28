




















// 134. Gas Station
// Greedy 11
struct Solution {}
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut cost_gas = Vec::new();
        let size = gas.len();
        for i in 0..size {
            cost_gas.push(gas[i]-cost[i]);
        }
        let mut start = 0;
        let mut sum = 0;
        while start < size {
            sum = 0;
            let mut found = true;
            for i in 0..size {
                sum += cost_gas[(start+i)%size];
                if sum < 0 {
                    start = start+i+1;
                    found = false;
                    break;
                }
            }
            if found {
                return start as i32;
            }
        }
        return -1;
    }
}
fn main() {
}
