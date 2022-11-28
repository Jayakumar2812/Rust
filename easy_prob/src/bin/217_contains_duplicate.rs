use std::collections::HashSet;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut lookup:HashSet<i32> = HashSet::new();
        for i in nums {
        if lookup.insert(i) == false {
            return true
        }
        
        }   
        return false
    }
}
