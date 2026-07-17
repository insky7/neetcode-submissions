use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let mut count: HashMap<i32, usize> = HashMap::new();
        for num in nums {
            *count.entry(num).or_insert(0) += 1;
        }
        let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); n + 1];
        for (num, freq) in count {
            buckets[freq].push(num);
        }
        let mut answer = Vec::with_capacity(k as usize);
        for bucket in buckets.iter().rev() {
            for &num in bucket {
                answer.push(num);
                if answer.len() == k as usize {
                    return answer;
                }
            }
        }
        answer
    }
}