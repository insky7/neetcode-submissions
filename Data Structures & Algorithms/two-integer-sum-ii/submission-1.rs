use std::collections::HashMap;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, i32> = HashMap::new();
        for (i, u) in numbers.iter().enumerate() {
            let complementary = target - u;
            if seen.contains_key(&complementary) {
                return vec![*seen.get(&complementary).unwrap() + 1, i as i32 + 1]
            } else {
                seen.insert(*u as i32, i as i32);
            }
        }
        vec![]
    }
}
