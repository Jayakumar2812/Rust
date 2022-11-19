impl Solution {

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut sorted_num = nums.clone();
    sorted_num.sort(); 
    let length = nums.len();
    if length == 1 {
        return nums[0];
    }
    let majority = length /2 +1;
    let mut cur_count = 0;

    for  i in 0..length {
        if i == 0 && sorted_num[i] == sorted_num[ i +1] {
            cur_count +=1;
            if cur_count == majority {
                return sorted_num[i];
            }
        }
        else if i!=0 && sorted_num[i] == sorted_num[ i -1] {
            cur_count +=1;
            if cur_count == majority {
                return sorted_num[i];
            }
        }
        else if i!=0 && sorted_num[i] != sorted_num[ i -1] {
            cur_count =1;

        }
        else {
             cur_count = 0
            }
    }

    return -1
}
}
