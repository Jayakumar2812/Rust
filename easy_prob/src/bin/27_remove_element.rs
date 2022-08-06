fn main() {
    let mut input = vec![3,2,2,3,2];
    println!("{:?}",remove_element(&mut input,3));
}



pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut l = nums.len();
    let mut i = 0;
    while i < l {
        println!("i==> {} , l==> {} ",&i,&l);
        if nums[i] == val {
            nums.remove(i);
            l-=1;
        }
        else {
            i+=1;
        }
    }
    println!("{:?}",&nums);
    
    return nums.len() as i32;
}