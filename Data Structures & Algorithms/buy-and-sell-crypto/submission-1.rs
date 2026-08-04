impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = 1;
        let mut maxProfit = 0;
        while r < prices.len() {
            if prices[l] < prices[r] {
                let profit = prices[r] - prices[l];
                maxProfit = max(profit, maxProfit);
            } else {
                l = r
            }
            r += 1
        }
        maxProfit
    }
}
