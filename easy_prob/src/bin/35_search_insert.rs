use core::num;

fn main() {
    println!("{:?}",search_insert(vec![1,3], 3));
}


pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let l = nums.len() -1;
    if nums[(nums.len() -1) as usize] < target{
        return nums.len() as i32;
    }
    return middle_out(&nums,target,0,l as usize) ;
}

pub fn middle_out(nums: &Vec<i32>, target: i32,start: usize,end: usize) -> i32 {

    let mut mid  = (start + end)/2;

    println!("start -> {} , mid -> {} , end -> {}",start,mid,end);

    if nums[mid] == target  {
        return mid as i32; 
    }
    else if nums[mid] < target && nums[mid +1] > target{
        return mid as i32 +1; 
    }
    else if nums[mid] < target {
        middle_out(&nums, target, mid +1, end)
    }
    else {
        println!("hi");
        middle_out(&nums, target, start, mid -1)
    }
    
}
