// mod merge_sorted_array;
fn main() {
    let mut vec1 = vec![-12,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    let mut vec2 = vec![-49,-45,-42,-41,-40,-39,-39,-39,-38,-36,-34,-34,-33,-33,-32,-31,-29,-28,-26,-26,-24,-21,-20,-20,-18,-16,-16,-14,-11,-7,-6,-5,-4,-4,-3,-3,-2,-2,-1,0,0,0,2,2,6,7,7,8,10,10,13,13,15,15,16,17,17,19,19,20,20,20,21,21,22,22,24,24,25,26,27,29,30,30,30,35,36,36,36,37,39,40,41,42,45,46,46,46,47,48];
    // merge_sorted_array::merge(&mut vec1,3,&mut vec2,3);
    merge(&mut vec1,1,&mut vec2,90);
}
// [-12,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
// 1
// [-49,-45,-42,-41,-40,-39,-39,-39,-38,-36,-34,-34,-33,-33,-32,-31,-29,-28,-26,-26,-24,-21,-20,-20,-18,-16,-16,-14,-11,-7,-6,-5,-4,-4,-3,-3,-2,-2,-1,0,0,0,2,2,6,7,7,8,10,10,13,13,15,15,16,17,17,19,19,20,20,20,21,21,22,22,24,24,25,26,27,29,30,30,30,35,36,36,36,37,39,40,41,42,45,46,46,46,47,48]
// 90

// [1,2,3,0,0,0]
// 3
// [2,5,6]
// 3
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let len_of_nums1 = nums1.len();
    let len_of_nums2 = nums2.len();
    if m == 0 {
        for  i in  0.. len_of_nums2 {
            nums1[i] = nums2[i];
        }
        return;
    }
    if n== 0 {
        return ;
    }
    let mut index_of_nums1:usize = 0;
    let mut index_of_nums2:usize = 0;
    let mut actual_inc_index_nums1:usize =0;
    while index_of_nums1 < len_of_nums1 && index_of_nums2 < n as usize {
        println!("hi");
        println!("index_of_nums1 --> {:?} , index_of_nums2 -->{:?} ",index_of_nums1,index_of_nums2);
        if nums1[index_of_nums1] >= nums2[index_of_nums2] {
            println!("hi 1");
            nums1.insert(index_of_nums1,nums2[index_of_nums2]);
            nums1.pop();
            index_of_nums2 +=1;
            index_of_nums1 +=1;
        }
        else if actual_inc_index_nums1 >= m as usize {
            nums1[index_of_nums1] = nums2[index_of_nums2];
            index_of_nums1 +=1;
            index_of_nums2 +=1;
        }
        else {
            actual_inc_index_nums1 +=1;
            index_of_nums1 +=1;
        }
    }
    // let cur_len_of_nums1 = nums1.len();

    println!(" after {:?}",nums1);
}





// if m == 0{
//     let len = nums1.len();        
//      for i in 0..nums2.len(){
//         nums1.push(nums2[i]);
//      }
//      for i in 0..len{
//         nums1.remove(0);
//     }

//     return ;
// }
// if n == 0 {
//     return;
// }
// let mut i:usize = 0;
// let mut j :usize= 0; 
// while  i <m as usize|| j< n  as usize {
//     println!(" inside i-->{} , j--> {} , num1-->{:?}",i,j,nums1);
//     if j == n as usize{
//         break;
//     }
//     else if i > m as usize {
//         println!("hi 222");           
//        nums1.insert(i,nums2[j]);
//        i +=1;
//        j +=1;
//        println!("hi 333");           
//     }
//     else if nums1[i] == nums2[j]{

//         nums1.insert(1,nums2[j]);
//         i+=1;
//         j +=1;
//     }
//     else if nums1[i] > nums2[j] {
//         nums1.insert(i,nums2[j]);
//         println!("{:?}",nums1);
//         j+=1;
//     }
//     else if nums1[i] == 0 && nums1[i-1] > nums1[i] {
//         println!("lol");
//         nums1.insert(i,nums2[j]);
//         j+=1;
//     }
//     else if i > m as usize {
//         println!("hi 222");           
//        nums1.insert(i,nums2[j]);
//        i +=1;
//        j +=1;
//        println!("hi 333");           
//     }
//     else{
//         println!("what's happening");
//         i+=1;
//         // j +=1;
//     }
// }
// while m+n < nums1.len() as i32 {
//     println!("bye");
//     nums1.pop();
// }
// println!("{:?}",nums1);

