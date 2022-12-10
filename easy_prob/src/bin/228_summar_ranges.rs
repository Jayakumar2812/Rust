impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.len() ==0 {
            return vec![]
        }
        let mut res = vec![];
        let mut temp = nums[0];
        let mut isSingle = true;
        let mut word:String = "".to_string();
        for i in (0..nums.len()){   
            if i < nums.len()-1 &&  nums[i] + 1 == nums[i+1] {
                if isSingle == true {
                    isSingle = false;
                    temp = nums[i];
                }
            }
            else {
                isSingle = true;
                if temp == nums[i] {
                    word += &temp.to_string();
                    res.push(word);
                }
                else {
                    word += &temp.to_string();
                    word += &"->".to_string();
                    word += &nums[i].to_string();
                    res.push(word);
                }
                
                word = "".to_string();
                if i < nums.len()-1 {
                    temp = nums[i+1];
                }

            }
        }
        return res
    }
}
