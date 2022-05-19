pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = prices[0];
    let mut max_profit = 0;
    for price in prices {
        if price < min_price {
            min_price = price;
        } else if price - min_price > max_profit {
            max_profit = price - min_price;
        }
    }
    max_profit
}

mod test {
    use super::*;
    
    #[test]
    fn max_profit_test1() {
        assert_eq!(max_profit(vec![7,1,5,3,6,4]), 5)
    }

    #[test]
    fn max_profit_test2() {
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0)
    }
}