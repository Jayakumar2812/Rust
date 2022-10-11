// #[derive(Debug, PartialEq, Eq)]
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
use std::rc::Rc;
use std::cell::RefCell;

impl From<TreeNode> for Option<Rc<RefCell<TreeNode>>> {
    fn from(treenode:TreeNode) -> Self {
        Some(Rc::new(RefCell::new(treenode)))
    }
}
fn main() {
    let leaf1 = TreeNode{val:4,left:None,right:None};
    let leaf2 = TreeNode{val:4,left:None,right:None};
    let leaf3 = TreeNode{val:5,left:None,right:None};
    let leaf4 = TreeNode{val:5,left:None,right:None};

    // let i:Option<Rc<RefCell<TreeNode>>> = leaf.into();
    let middle1 = TreeNode{val:2,left:None,right:None};
    let middle2 = TreeNode{val:2,left:leaf2.into(),right:None};
    let root1 =TreeNode{val:1,left:middle1.into(),right:middle2.into()};
    println!("{}",has_path_sum(root1.into(),8));
}

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        
    match root {
        Some(root) =>{
            helper(root.into(), target_sum, 0)
        }
        None =>{
            return false
        }
    }

}


fn helper(cur_root: Option<Rc<RefCell<TreeNode>>>,target_sum: i32,mut cur_sum:i32) -> bool {
    match cur_root {
        Some(cur_root) =>{
            cur_sum += cur_root.borrow().val;
            let left = cur_root.borrow().left.clone();
            let right = cur_root.borrow().right.clone();
            match (left,right) {
                (Some(left),Some(right)) =>{
                    let left = helper(left.into(), target_sum, cur_sum);
                    let right = helper(right.into(), target_sum, cur_sum);
                    if left || right {
                        return true
                    }
                    else {
                        return false
                    }
                }
                (Some(left),None)=>{
                    helper(left.into(), target_sum, cur_sum)
                }
                (None,Some(right))=>{
                    helper(right.into(), target_sum, cur_sum)
                }
                (None,None)=>{
                    if cur_sum == target_sum {
                        return true
                    }
                    else {
                        return false
                    }
                }
            }
            

            
        }

        None =>{
            return false 
        }
    }
}