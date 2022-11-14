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
    fn from(treenode:TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(treenode)))
    }
}
pub struct  Solution {

}
use std::rc::Rc;
use std::cell::RefCell;
// impl Solution {
//     pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
//         match root {            
//             Some (root)=> {
//                 let result = vec![root.borrow().val];
//                 helper(root.into(),result,true)
//             }
//             None =>{
//                 return vec![]
//             }
//         }
//     }
// }

// fn helper ( root:Option<Rc<RefCell<TreeNode>>>,mut res:Vec<i32>,mut init:bool) -> Vec<i32> {

//     match root {
//         Some(root) => {

//             let left_root = root.borrow().left.clone();
//             let right_root = root.borrow().right.clone();
//             match (left_root,right_root) {
//                 (Some(left_root),Some(right_root)) =>{

//                     let lefts_left_root = left_root.borrow().left.clone();
//                     let lefts_right_root = left_root.borrow().right.clone();
//                     let rights_left_root = right_root.borrow().left.clone();
//                     let rights_right_root = right_root.borrow().right.clone();
//                     match (lefts_left_root,lefts_right_root,rights_left_root,rights_right_root){
                     
//                         (None,None,Some(rights_left_root),Some(rights_right_root)) => {
//                             res = helper(right_root.into(), res, init);
//                             println!(" res1 --> {:?}",&res);
//                             res.push(root.borrow().val);
//                             println!(" res2 --> {:?}",&res);
//                             res
//                         }
//                         (None,None,None,Some(rights_right_root)) => {
//                             res = helper(right_root.into(), res, init);
//                             println!(" res3 --> {:?}",&res);
//                             res.push(root.borrow().val);
//                             println!(" res4 --> {:?}",&res);
//                             res                        }
//                         (SNone,None,Some(rights_left_root),None) => {
//                             res = helper(right_root.into(), res, init);
//                             println!(" res5 --> {:?}",&res);
//                             res.push(root.borrow().val);
//                             println!(" res6 --> {:?}",&res);
//                             res                        }
//                         (Some(lefts_left_root),Some(lefts_right_root),None,None) => {
//                             res = helper(left_root.into(), res, init);
//                             println!(" res7 --> {:?}",&res);
//                             res.push(root.borrow().val);
//                             println!(" res8 --> {:?}",&res);
//                             res
//                         }
//                         (None,Some(lefts_right_root),None,None) => {
//                             res = helper(left_root.into(), res, init);
//                             println!(" res9 --> {:?}",&res);
//                             res.push(root.borrow().val);
//                             println!(" res10 --> {:?}",&res);
//                             res
//                         }
//                         (Some(lefts_left_root),None,None,None) => {
//                             res = helper(left_root.into(), res, init);
//                             println!(" res11 --> {:?}",&res);
//                             res.push(root.borrow().val);
//                             println!(" res12 --> {:?}",&res);
//                             res                        
//                         }

//                         (None,None,None,None) =>{
//                             if init {
//                                 init = false;
//                                 res.push(root.borrow().val);
//                                 println!(" res13 --> {:?}",&res);
//                                 res.push(left_root.borrow().val);
//                                 println!(" res14 --> {:?}",&res);
//                                 res.push(right_root.borrow().val);
//                                 println!(" res15 --> {:?}",&res);
//                                 return res
//                             }
//                             else {
//                                 res.push(root.borrow().val);
//                                 println!(" res16 --> {:?}",&res);
//                                 res.push(left_root.borrow().val);
//                                 println!(" res17 --> {:?}",&res);
//                                 res.push(right_root.borrow().val);
//                                 println!(" res18 --> {:?}",&res);
//                                 return res
//                             }
                            
//                         }
//                         _=>{
//                             res = helper(left_root.into(), res, init);
//                             println!(" res19 --> {:?}",&res);
//                             helper(right_root.into(), res, init)
//                         }
//                     }
//                 }
//                 (Some(left_root),None) =>{
//                     let lefts_left_root = left_root.borrow().left.clone();
//                     let lefts_right_root = left_root.borrow().right.clone();
//                     match(lefts_left_root,lefts_right_root) {
//                         (None,None) =>{
//                             if init {
//                                 init = false;
//                                 res.push(root.borrow().val);
//                                 println!(" res20 --> {:?}",&res);
//                                 res.push(left_root.borrow().val);
//                                 println!(" res21 --> {:?}",&res);
//                                 return res
//                             }
//                             else {
//                                 res.push(root.borrow().val);
//                                 println!(" res22 --> {:?}",&res);
//                                 res.push(left_root.borrow().val);
//                                 println!(" res23 --> {:?}",&res);
//                                 return res
//                             }
//                         }
//                         _=>{
//                             res = helper(left_root.into(), res, init);
//                             println!(" res24 --> {:?}",&res);
//                             res.push(root.borrow().val);
//                             println!(" res25 --> {:?}",&res);
//                             res                        
//                         }
//                     }
//                 }
//                 (None,Some(right_root)) =>{
//                     let rights_left_root = right_root.borrow().left.clone();
//                     let rights_right_root = right_root.borrow().right.clone();
//                     match(rights_left_root,rights_right_root) {
//                         (None,None) =>{
//                             if init {
//                                 init = false;
//                                 res.push(root.borrow().val);
//                                 println!(" res26 --> {:?}",&res);
//                                 res.push(right_root.borrow().val);
//                                 println!(" res27 --> {:?}",&res);
//                                 return res
//                             }
//                             else {
//                                 res.push(root.borrow().val);
//                                 println!(" res28 --> {:?}",&res);
//                                 res.push(right_root.borrow().val);
//                                 println!(" res29 --> {:?}",&res);
//                                 return res
//                             }
//                         }
//                         _=>{
//                             res = helper(right_root.into(), res, init);
//                             println!(" res30 --> {:?}",&res);
//                             res.push(root.borrow().val);
//                             println!(" res31 --> {:?}",&res);
//                             res                        
//                         }
//                     }

//                 }
//                 (None,None) =>{
//                     res.push(root.borrow().val);
//                     println!(" res32 --> {:?}",&res);
//                     return res
//                 }

//         }
//     }
//     None =>{
//         return res
//     }
// }
// }

impl Solution {
fn _preorder_traversal(root: Option<&Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
    if let Some(node) = root {
        let nodeb = node.borrow();
        ret.push(nodeb.val);
        Self::_preorder_traversal(nodeb.left.as_ref(), ret);
        Self::_preorder_traversal(nodeb.right.as_ref(), ret);
    }
}

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ret = Vec::new();
    Self::_preorder_traversal(root.as_ref(), &mut ret);
    ret
}
}

fn main(){
    let leaf3 = TreeNode{val:4,left:None,right:None};
    let leaf4 = TreeNode{val:5,left:None,right:None};
    
    let leaf1 = TreeNode{val:6,left:leaf3.into(),right:None};    
    let leaf2 = TreeNode{val:3,left:None,right:None};

    let middle1 = TreeNode{val:2,left:leaf1.into(),right:None};
    let middle2 = TreeNode{val:8,left:leaf2.into(),right:None};

    let root1 =TreeNode{val:1,left:None,right:middle2.into()};
    
    println!(" res from main --> {:?}",Solution::preorder_traversal(root1.into()));
}