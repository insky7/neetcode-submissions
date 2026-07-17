use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut freq: Vec<Vec<i32>> = vec![vec![]; nums.len() + 1];
        for n in nums.iter() {
            *map.entry(*n).or_insert(0) += 1;
        }
        for (num, count) in map.iter() {
            freq[*count as usize].push(*num);
        }
        let mut res = vec![];
        for i in (0..freq.len()).rev() {
            for n in &freq[i] {
                res.push(*n);
                if res.len() == k as usize {
                    return res;
                }
            }
        }
        res
    }
}