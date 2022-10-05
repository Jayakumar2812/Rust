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
impl From<TreeNode> for Option<Rc<RefCell<TreeNode>>> {
  fn from(treenode: TreeNode) -> Self {
    Some(Rc::new(RefCell::new(treenode)))
  }
}

fn main() {
    let leaf1 = TreeNode{val:3,left:None,right:None};
    // let i:Option<Rc<RefCell<TreeNode>>> = leaf.into();
    let middle1 = TreeNode{val:2,left:leaf1.into(),right:None};
    let root1 =TreeNode{val:1,left:None,right:middle1.into()};
    
    let leaf2 = TreeNode{val:3,left:None,right:None};
    // let i:Option<Rc<RefCell<TreeNode>>> = leaf.into();
    let middle2 = TreeNode{val:2,left:None,right:leaf2.into()};
    let root2 =TreeNode{val:1,left:None,right:middle2.into()};


    println!("{:?}",is_same_tree(root1.into(),root2.into()));
  
  }
  pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        helper(p,q)
}
  
  fn helper(root1: Option<Rc<RefCell<TreeNode>>>,root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut result = true;
    if root1 == None && root2 == None {
        return true;
    }
    let  i:Option<i32> ;
    let  j:Option<i32> ;
    match root1.as_ref() {
        None => i = None,
        Some(root1) => {
            i = Some(root1.borrow().val);
        }
    }
    match root2.as_ref() {
        None => j = None,
        Some(root2) => {
             j = Some(root2.borrow().val);
        }
    }
    println!("i ---> {:?} , j ---> {:?}",i,j);

    match (i, j) {
        (Some(i), Some(j)) =>{
            if i != j {
                result = false;
                return result;
            }
        }, 
        (Some(_i),None) => {
            result = false;
            return result;
        },
        (None,Some(_j)) =>{
            result = false;
            return result;
        }
        (None, None) => (),
    }
    let  left1:Option<Rc<RefCell<TreeNode>>>;
    let  left2:Option<Rc<RefCell<TreeNode>>> ;
    match root1.as_ref() {
        None => left1 = None,
        Some(root1) => {
            left1 = root1.borrow().left.clone()
        }
    }
    match root2.as_ref() {
        None => left2 = None,
        Some(root2) => {
            left2 = root2.borrow().left.clone()
        }
    }
    let  left_i:Option<i32> ;
    let  left_j:Option<i32> ;
    
    match left1.as_ref() {
        None => left_i = None,
        Some(left1) => {
            left_i = Some(left1.borrow().val);
        }
    }
    match left2.as_ref() {
        None => left_j = None,
        Some(left2) => {
            left_j = Some(left2.borrow().val);
        }
    }
    println!("left_i ---> {:?} , left_j ---> {:?}",left_i,left_j);

    match (left_i, left_j) {
        (Some(left_i), Some(left_j)) =>{
            if left_i != left_j {
                result = false;
                return result;            }
        }, 
        (Some(_left_i),None) => {
            result = false;
            return result;        },
        (None,Some(_left_j)) =>{
            result = false;
            return result;        },
        (None, None) => (   ),
    }

    let  right1:Option<Rc<RefCell<TreeNode>>>;
    let  right2:Option<Rc<RefCell<TreeNode>>> ;
    match root1.as_ref() {
        None => right1 = None,
        Some(root1) => {
            right1 = root1.borrow().right.clone()
        }
    }
    match root2.as_ref() {
        None => right2 = None,
        Some(root2) => {
            right2 = root2.borrow().right.clone()
        }
    }
    let  right_i:Option<i32> ;
    let  right_j:Option<i32> ;
    
    match right1.as_ref() {
        None => right_i = None,
        Some(right1) => {
            right_i = Some(right1.borrow().val);
        }
    }
    match right2.as_ref() {
        None => right_j = None,
        Some(right2) => {
            right_j = Some(right2.borrow().val);
        }
    }
    println!("right_i ---> {:?} , right_j ---> {:?}",right_i,right_j);
    match (right_i, right_j) {
        (Some(right_i), Some(right_j)) =>{
            if right_i != right_j {
                result = false;
                return result;            }
        }, 
        (Some(_right_i),None) => {
            result = false;
            return result;        },
        (None,Some(_right_j)) =>{
            result = false;
            return result;        }
        (None, None) => (),
    }
    let res1 = helper(left1, left2);
    let res2 = helper(right1, right2);
    if result && res1 && res2 {
        return true
    }  
    else {
        return false
    }

  }

      // let mut output = Vec::new();
  
    // match root {
    //     None => output,
    //     Some(root) => {
    //         output.append(&mut helper(root.borrow().left.clone()));
    //         output.push(root.borrow().val);
    //         output.append(&mut helper(root.borrow().right.clone()));
    //         println!("{:?}",output);
    //         output
    //     }
    // }