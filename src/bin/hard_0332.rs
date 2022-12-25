// 332. Reconstruct Itinerary
// BackTrace 19
struct Solution {}
impl Solution {
    fn backtrace(pos: &mut Vec<String>, res: &mut Vec<String>, tickets: &[Vec<String>], valid: &mut [bool]) -> bool {
        if pos.len() == tickets.len() + 1 {
            *res = pos.clone();
            return true;
        }
        let head = pos.last().unwrap();
        let mut ways = Vec::new();
        for i in 0..tickets.len() {
            if tickets[i][0] == *head {
                ways.push(i);
            }
        }
        for i in ways {
            if valid[i] {
                let ticket = &tickets[i];
                pos.push(ticket[1].clone());
                valid[i] = false;
                if Self::backtrace(pos, res, tickets, valid) { return true;}
                valid[i] = true;
                pos.pop();
            }
        }
        return false;
    }
    pub fn find_itinerary(mut tickets: Vec<Vec<String>>) -> Vec<String> {
        tickets.sort();
        let mut pos = Vec::new();
        pos.push("JFK".to_string());
        let mut res = Vec::new();
        let mut valid = vec![true;tickets.len()];
        Self::backtrace(&mut pos, &mut res, &tickets, &mut valid);
        return res;
    }
}
fn main() {
}
