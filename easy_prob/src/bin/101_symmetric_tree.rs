use std::rc::Rc;
use std::cell::RefCell;
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
impl From<TreeNode> for Option<Rc<RefCell<TreeNode>>> {
    fn from(treenode: TreeNode) -> Self {
      Some(Rc::new(RefCell::new(treenode)))
    }
  }
  

fn main(){
    let leaf1 = TreeNode{val:4,left:None,right:None};
    let leaf2 = TreeNode{val:4,left:None,right:None};
    let leaf3 = TreeNode{val:5,left:None,right:None};
    let leaf4 = TreeNode{val:5,left:None,right:None};

    // let i:Option<Rc<RefCell<TreeNode>>> = leaf.into();
    let middle1 = TreeNode{val:2,left:leaf1.into(),right:leaf3.into()};
    let middle2 = TreeNode{val:2,left:leaf2.into(),right:leaf4.into()};
    let root1 =TreeNode{val:1,left:middle1.into(),right:middle2.into()};
    
    println!("{}",is_symmetric(root1.into()));
}

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let  left:Option<Rc<RefCell<TreeNode>>>;
    let  right:Option<Rc<RefCell<TreeNode>>>;
    match root {
        None => {
            left = None;
            right = None;
        },
        Some(root) => {
            left =root.borrow().left.clone();
            right = root.borrow().right.clone();

        }
    }
    match (left,right) {
        (Some(left),Some(right)) =>{
            // println!("left --> {:?} , right --> {:?}",left,right);
            if left.borrow().val == right.borrow().val {
                helper(Some(left), Some(right))
            }
            else {
                println!("Primary1 false");
                return false
            }
        },
        (None,None) =>{
            println!("prim 2");
            return true
        },
        _ => {
            println!("Primary3 false");
            return false
        }
    }
    

    // helper(root)
}
fn helper(left:Option<Rc<RefCell<TreeNode>>>,right:Option<Rc<RefCell<TreeNode>>>) -> bool{ 
    let  left_of_left:Option<Rc<RefCell<TreeNode>>>;
    let  right_of_left:Option<Rc<RefCell<TreeNode>>>;
    let  left_of_right:Option<Rc<RefCell<TreeNode>>>;
    let  right_of_right:Option<Rc<RefCell<TreeNode>>>;
    match (left,right) {
        (Some(left),Some(right)) =>{
        left_of_left = left.borrow().left.clone();
        right_of_left = left.borrow().right.clone();
        left_of_right = right.borrow().left.clone();
        right_of_right = right.borrow().right.clone();
            
        let mut res = true;
        match (left_of_left,right_of_left,left_of_right,right_of_right) {
            (Some(left_of_left),Some(right_of_left),Some(left_of_right),Some(right_of_right)) =>{
                if left_of_left.borrow().val == right_of_right.borrow().val && right_of_left.borrow().val == left_of_right.borrow().val {
                    println!("sec1");
                    let res1 = helper(Some(left_of_left), Some(right_of_right));
                    let res2 =helper(Some(right_of_left), Some(left_of_right));
                    if res1 && res2 {
                        res = true
                    }
                    else {
                        res = false
                    }
                }
                else {
                    res = false
                }
            }
            (Some(left_of_left),None,None,Some(right_of_right))=>{
                println!("sec2");
                if left_of_left.borrow().val == right_of_right.borrow().val {
                     res = helper(Some(left_of_left), Some(right_of_right));
                }
                else {
                    res = false
                }
            }
            (None,Some(right_of_left),Some(left_of_right),None) =>{
                println!("sec3");
                if right_of_left.borrow().val == left_of_right.borrow().val {
                     res = helper(Some(right_of_left), Some(left_of_right));
                }
                else {
                    res = false
                }
            }
            (None,None,None,None) =>{
                println!("sec4");
                res=  true
            }
            _ =>{
                res= false
            }
        }
        
        res
        }

        (None,None) =>{
            println!("sec5");
                true
            },
        _ => {
            false
        },
    }

}


// if left.borrow().left.as_ref().unwrap().borrow().val == right.borrow().right.as_ref().unwrap().borrow().val && left.borrow().right.as_ref().unwrap().borrow().val == right.borrow().left.as_ref().unwrap().borrow().val {
//     let res1 = helper(left.borrow().left.as_ref(), right.borrow().right.as_ref());
//     let res2 = helper(left.borrow().right.as_ref(), right.borrow().left.as_ref());
//     if res1 && res2{
//          true
//     }      
//     else{
//         false
//     }          
// }
// else {
//     return false
// }

// },