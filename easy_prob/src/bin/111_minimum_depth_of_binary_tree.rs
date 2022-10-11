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
impl  From<TreeNode> for  Option<Rc<RefCell<TreeNode>>> {
    fn from(treenode: TreeNode) -> Self {
        Some(Rc::new(RefCell::new(treenode)))
      } 
}

use std::rc::Rc;
use std::cell::RefCell;
fn main(){    
    let leaf1 = TreeNode{val:4,left:None,right:None};
    let leaf2 = TreeNode{val:4,left:None,right:None};
    let leaf3 = TreeNode{val:5,left:None,right:None};
    let leaf4 = TreeNode{val:5,left:None,right:None};

    // let i:Option<Rc<RefCell<TreeNode>>> = leaf.into();
    let middle1 = TreeNode{val:2,left:leaf3.into(),right:leaf1.into()};
    let middle2 = TreeNode{val:2,left:leaf2.into(),right:None};
    let root1 =TreeNode{val:1,left:middle1.into(),right:middle2.into()};
    // println!("{}",min_depth(root1.into()));
    println!("{}",min_depth_mock(root1.into()));
}



// pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

    

//     return 1
// }

pub fn helper2(left_root: Option<Rc<RefCell<TreeNode>>>,right_root: Option<Rc<RefCell<TreeNode>>>,mut left_depth:i32, mut right_depth:i32) -> i32{
    let res = 1;
    match (left_root,right_root) {
        (Some(left_root),Some(right_root)) =>{
            
        }
        (Some(left_root),None) =>{

        }
        (None,Some(right_root)) =>{

        }
        (None,None) =>{
            return  1
        }
    }
    return 1
}


pub fn helper3(cur_root: Option<Rc<RefCell<TreeNode>>>,mut cur_depth:i32) -> i32{
    match cur_root {
        Some(cur_root) =>{
            let left_root = cur_root.borrow().left.clone(); 
            let right_root = cur_root.borrow().right.clone(); 
            match (left_root,right_root) {
                (Some(left_root),Some(right_root)) =>{
                    cur_depth +=1;
                    let left_depth = helper3(left_root.into(), cur_depth);
                    let right_depth =   helper3(right_root.into(), cur_depth);
                    println!("cur_depth --> {} , left_depth --> {} , right_depth --> {} ",cur_depth,left_depth,right_depth);
                    if left_depth == cur_depth && right_depth == cur_depth {
                        return cur_depth
                    }
                    else {
                        println!("max_depth --> {}",left_depth.max(right_depth));
                       return left_depth.max(right_depth);
                    }

                }
                (Some(left_root),None) =>{
                    cur_depth +=1;
                    println!("cur_depth from left   --> {}",cur_depth);
                    return  helper3(left_root.into(), cur_depth)
                }
                (None,Some(right_root)) =>{
                    cur_depth +=1;
                    println!("cur_depth from right --> {}",cur_depth);
                    return helper3(right_root.into(), cur_depth)
                }
                (None,None) =>{
                    println!("cur_depth from inner none --> {}",cur_depth);
                    return  cur_depth
                }
            }

        }
        None =>{
            println!("cur_depth from outer none --> {}",cur_depth);
            return cur_depth
        }
    }
}

pub fn min_depth_mock(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        
    match root {
        Some(root) => {
            helper3(root.into(), 0)
        }
        None => {
            return 0
        }
    }
}
pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        
    match root {
        Some(root) => {
            helper(root.into(), 0)
        }
        None => {
            return 0
        }
    }
}

fn helper(cur_root:Option<Rc<RefCell<TreeNode>>>,mut cur_depth:i32) -> i32 {
    match cur_root {
        Some(cur_root) => {
            cur_depth +=1;
            let left = helper(cur_root.borrow().left.clone(),cur_depth); 
            let right = helper(cur_root.borrow().right.clone(),cur_depth); 
            if left == 0 && right == 0 {
                return cur_depth
            }
            else if  left!= 0 && right ==0 {
                           return left;
            }
            else if  left== 0 && right !=0 {
                return right;
            }
            else {
                return left.min(right);
            }

        }
        None =>{
            return 0;
        }
    }
}