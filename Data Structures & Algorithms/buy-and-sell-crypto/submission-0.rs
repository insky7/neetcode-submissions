impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max = 0;
        for (i, u) in prices.iter().enumerate() {
            for (i2, u2) in prices[i..prices.len()].iter().enumerate() {
                println!("{:?}", max);
                if u2 - u > max {
                    max = u2 - u;
                } else {
                    continue
                }
            }
        }
        max
    }
}
