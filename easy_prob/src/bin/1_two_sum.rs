use std::collections::HashMap;
use::std::cmp::Ordering;
fn main() {
    println!("two_sum_iterative_approach --> {:?}",two_sum(vec![1,2,3,4,5,9,10,25,67],7));
    println!("two_sum_recursive_approach--> {:?}",rec(vec![1,2,3,4,5,9,10,25,67],7));
}


// it is the iterative approach 
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    
    let mut hm = HashMap::with_capacity(nums.len());
    for (i,&val) in nums.iter().enumerate() {
        let look = target - val;
        if let Some(&j) = hm.get(&look){
            return vec![i as i32,j];
        }
        hm.insert(val, i as i32);
    }
    vec![]
}
// it  is  recursive approach 
pub fn rec(sortarr: Vec<i32>, target: i32) -> Vec<i32> {
    if sortarr.is_empty() {
        return vec![];
    }
    // println!("array is {:?}",sortarr);
    // println!("mid index : {:?}",sortarr.len()/2);
    let cmp = sortarr[sortarr.len()/2]+sortarr[sortarr.len()/2 +1];
    // println!("comparison string is {:?} :===> target is {:?} ",cmp,target);

    match (sortarr[sortarr.len()/2]+sortarr[sortarr.len()/2 +1]).cmp(&target) {
        Ordering::Less =>{
            let mid = sortarr.len()/2;
            let retarr =  sortarr[mid..].to_vec();
            return rec(retarr,target);
        }
        Ordering::Equal =>{
                // println!("{:?} {:?}",&sortarr[0],sortarr[sortarr.len()/2]);
                let res = vec![sortarr[0],sortarr[sortarr.len()/2]];
                res
        }
        Ordering::Greater =>{
            // println!("inside gt");
                let mid = sortarr.len() /2;
                let retarr =  sortarr[..=mid].to_vec();
                return rec(retarr,target);
           
        }
    }


}
