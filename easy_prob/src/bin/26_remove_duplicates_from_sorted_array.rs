fn main() {
    let mut input = vec![1,1,2,3,3,3,4];
    println!("{:?}",remove_duplicates(&mut input));
}



// pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
//     let mut length = nums.len();
//     let mut i =0;
//     while i <length -1 {

//         let mut j = i +1;
//         println!("i = {} , j = {} , lenght = {}",&i,&j,length);
        
//         while (&nums[i] == &nums[j]){
//             nums.remove(j);
//             if j == length -1{
//                 break;
//             }
//             length -=1; 
//         }
//     i +=1;
//     }
//     println!("res::{:?}",&nums);
//     return nums.len() as i32; 
// }   

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut l: i32 = 1;
    
    for r in 1..nums.len() {
        println!("{:?}", r);
        if nums[r] != nums[r - 1] {
            nums[l as usize] = nums[r];
            l += 1;
        }
    }
    println!("{:?}",nums);
    l
}