

fn main() {
    println!("{:?}",max_profit(vec![7,6,4,3,1]))
}

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut cur_max = 0;
        let mut cur_min = prices[0];
        let mut max_profit = 0;
        for i in  0..prices.len() {
            if prices[i as usize] <= cur_min {
                cur_min = prices[i as usize];
                cur_max = cur_min;
            }
            else if prices[i] > cur_max {
                cur_max =  prices[i as usize]  
            }
            let temp_max_profit = cur_max - cur_min;
            if temp_max_profit > max_profit {
                max_profit = temp_max_profit;
            }
        }
        return max_profit; 
    }