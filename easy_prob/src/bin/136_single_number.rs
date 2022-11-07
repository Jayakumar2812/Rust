
use std::collections::HashMap;

fn main() {
println!("{}",single_number(vec![5]))
}


pub fn single_number(nums: Vec<i32>) -> i32 {
    let half_len =  nums.len()/2;
    let mut map: HashMap<i32, bool> = HashMap::with_capacity(half_len);
    let mut sum= 0;
    for i in 0..nums.len(){
        if map.get(&nums[i]) == Some(&true) {
            sum -= nums[i];
        } 
        else{
            map.insert(nums[i],true);
            sum +=nums[i];
        }
    }    
    return sum;
}
