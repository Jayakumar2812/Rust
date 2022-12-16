impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <=0 {
            return false
        }
        format!("{n:b}").matches('1').count() == 1
    }
}
