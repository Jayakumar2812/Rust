// Definition for a binary tree node.
use std::rc::Rc;
use std::cell::RefCell;
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
fn main () 
{
    let leaf1 = TreeNode{val:4,left:None,right:None};
    let leaf2 = TreeNode{val:4,left:None,right:None};
    let leaf3 = TreeNode{val:5,left:None,right:None};
    let leaf4 = TreeNode{val:5,left:None,right:None};

    // let i:Option<Rc<RefCell<TreeNode>>> = leaf.into();
    let middle1 = TreeNode{val:2,left:None,right:None};
    let middle2 = TreeNode{val:2,left:leaf2.into(),right:None};
    let root1 =TreeNode{val:1,left:middle1.into(),right:middle2.into()};
    println!("{}",max_depth(root1.into()));
}



pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut depth = 0;
    match root {
        Some(root) =>{
            if root.as_ref().borrow().left == None && root.as_ref().borrow().right == None {
                return 1
            }
            else if root.as_ref().borrow().left == None && root.as_ref().borrow().right != None {
                depth = 2;
                return helper(root.borrow().right.clone(),depth);

            }
            else if root.as_ref().borrow().left != None && root.as_ref().borrow().right == None {
                depth = 2;
                return helper(root.borrow().left.clone(),depth);

            }
            else {
                depth = 2;
                let left_depth = helper(root.borrow().left.clone(),depth);
                let right_depth = helper(root.borrow().right.clone(),depth);
                if   left_depth >= right_depth {
                    return left_depth;
                }
                else {
                    return right_depth;
                }
            }
        }
        None =>{
            return 0;
        }
    }
        
}

fn helper(cur_root:Option<Rc<RefCell<TreeNode>>>,mut cur_depth: i32) -> i32{
    match cur_root {
        Some(cur_root) =>{
            if cur_root.as_ref().borrow().left == None && cur_root.as_ref().borrow().right == None {
                return cur_depth
            }
            else if cur_root.as_ref().borrow().left == None && cur_root.as_ref().borrow().right != None {
                cur_depth +=1; 
                return helper(cur_root.borrow().right.clone(),cur_depth);
            }
            else if cur_root.as_ref().borrow().left != None && cur_root.as_ref().borrow().right == None {
                cur_depth +=1; 
                return helper(cur_root.borrow().left.clone(),cur_depth);

            }
            else {
                cur_depth +=1; 
                let left_depth = helper(cur_root.borrow().left.clone(),cur_depth);
                let right_depth = helper(cur_root.borrow().right.clone(),cur_depth);
                if   left_depth >= right_depth {
                    return left_depth;
                }
                else {
                    return right_depth;
                }
            }
        }
        None =>{
            return cur_depth;
        }
    }
}