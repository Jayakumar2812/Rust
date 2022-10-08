// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}


impl From<TreeNode> for  Option<Rc<RefCell<TreeNode>>> {
    fn from(treenode: TreeNode) -> Self {
        Some(Rc::new(RefCell::new(treenode)))
      } 
}


use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    println!("{:?}",sorted_array_to_bst(vec![0,1,2,3,4,5,6,7,8]));
}



pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {

    if nums.len() == 1 {
        let cur_root = TreeNode{val:nums[0],left:None,right:None};
        return cur_root.into()
         
    }
    else if nums.len() == 2 {
        let left_tree = TreeNode{val:nums[0],left:None,right:None};
        let cur_root = TreeNode{val:nums[1],left:left_tree.into(),right:None};
        return cur_root.into()
    }
    else if nums.len() == 3 {
        let left_tree = TreeNode{val:nums[0],left:None,right:None};
        let right_tree = TreeNode{val:nums[2],left:None,right:None};
        let cur_root = TreeNode{val:nums[1],left:left_tree.into(),right:right_tree.into()};
        return cur_root.into()
    }
    else {

    let mid_num = nums[nums.len()/2];
    let mid  = nums.len()/2;
    
    let left_half = &nums[0..mid as usize];
    let right_half = &nums[(mid +1) as usize ..];
    println!("before 1st rec left_half --> {:?} ",left_half.to_vec());
    let left_tree = rec(left_half.to_vec());
    println!("before 2nd rec");
    let right_tree  = rec(right_half.to_vec());
    println!("after rec");
    let root = TreeNode{val:mid_num,left:left_tree,right:right_tree};
    root.into()
    }
}
pub fn rec(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    println!("{}",nums.len());
    if nums.len() == 1 {
        let cur_root = TreeNode{val:nums[0],left:None,right:None};
        return cur_root.into()
         
    }
    else if nums.len() == 2 {
        let left_tree = TreeNode{val:nums[0],left:None,right:None};
        let cur_root = TreeNode{val:nums[1],left:left_tree.into(),right:None};
        return cur_root.into()
    }
    else if nums.len() == 3 {
        let left_tree = TreeNode{val:nums[0],left:None,right:None};
        let right_tree = TreeNode{val:nums[2],left:None,right:None};
        let cur_root = TreeNode{val:nums[1],left:left_tree.into(),right:right_tree.into()};
        return cur_root.into()
    }
    else {
    let mid_num = nums[nums.len()/2];
    let mid  = nums.len()/2;    
    let left_half = &nums[0..mid as usize];
    let right_half = &nums[(mid +1) as usize ..];
    let left_tree = rec(left_half.to_vec());
    let right_tree  = rec(right_half.to_vec());
    let cur_root = TreeNode{val:mid_num,left:left_tree,right:right_tree};
    cur_root.into()
    }

}

        
    // let len = nums.len();
    // let res:Option<Rc<RefCell<TreeNode>>>;
    // if len == 1 {
    //     res =  TreeNode{val:nums[0],left:None,right:None}.into();

    // }
    // else if  len  == 2 {
    //     let  left_res_2 = TreeNode{val:nums[0],left:None,right:None}.into();
    //     res = TreeNode{val:nums[1],left:left_res_2,right:None}.into();

    // }
    // else if  len == 3 {
    //     let left_res_3 = TreeNode{val:nums[0],left:None,right:None}.into();
    //     let right_res_3 = TreeNode{val:nums[2],left:None,right:None}.into();
    //     res = TreeNode{val:nums[1],left:left_res_3,right:right_res_3}.into();
    // }
    // else {
    //     let mut cur_left:Option<Rc<RefCell<TreeNode>>>;
    //     let mut cur_right:Option<Rc<RefCell<TreeNode>>>;
    //     let mut cur_root:Option<Rc<RefCell<TreeNode>>> = None;
    //     for  i  in (0..len).step_by(3){
    //         if i == 0 {
    //                 cur_left = TreeNode{val:nums[i],left:None,right:None}.into();
    //                 cur_right = TreeNode{val:nums[i+2],left:None,right:None}.into();
    //                 cur_root = TreeNode{val:nums[i+1],left:cur_left,right:cur_right}.into();
    //             } 
    //         else if i <= len/2 +2 {
    //                 cur_left = cur_root;
    //                 cur_right = TreeNode{val:nums[i+1],left:None,right:None}.into();
    //                 cur_root = TreeNode{val:nums[i+1],left:cur_left,right:cur_right}.into();
    //             }
    //         }
    //     println!("{:?}",cur_root.unwrap().borrow().val);
    // }
    // let leaf1 = TreeNode{val:4,left:None,right:None};
    // leaf1.into()
