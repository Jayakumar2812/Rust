use std::collections::HashSet;
impl Solution {
pub fn is_happy(n: i32) -> bool {
        let mut num = n;
        let mut lookup = HashSet::new();
        let mut sum = 0;
        lookup.insert(num);
        while sum !=1 {
            sum = 0;
            while num >= 1 {
                    let cur_digit = num %10;
                    sum += cur_digit* cur_digit;
                    num = num/10;
                }
            num = sum;
            if num == 1 {
                return true
            }
            if lookup.insert(num) == false {
                return false
            }
            
        }
        return true
    }
}
