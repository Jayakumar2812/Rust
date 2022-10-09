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
//[1,2,2,3,null,null,3,4,null,null,4]

fn main () {
    let leaf3 = TreeNode{val:4,left:None,right:None};
    let leaf4 = TreeNode{val:4,left:None,right:None};
    
    let leaf2 = TreeNode{val:3,left:None,right:leaf4.into()};
    let leaf1 = TreeNode{val:3,left:leaf3.into(),right:None};
    // let i:Option<Rc<RefCell<TreeNode>>> = leaf.into();
    
    let middle1 = TreeNode{val:2,left:leaf1.into(),right:None};
    let middle2 = TreeNode{val:2,left:None,right:leaf2.into()};
    let root1 =TreeNode{val:1,left:middle1.into(),right:middle2.into()};
    println!("{}",is_balanced(root1.into()));
}

struct IntermediateResult {
    is_balanced: bool,
    height: i32,
}

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        is_balanced_recursive_impl(&root).is_balanced
}

fn is_balanced_recursive_impl(node: &Option<Rc<RefCell<TreeNode>>>) -> IntermediateResult {
        if node.is_none() {
            return IntermediateResult {
                is_balanced: true,
                height: 0
            }
        }

        let nd: &TreeNode = &(node.as_ref().unwrap().borrow());

        let l_res = is_balanced_recursive_impl(&nd.left);
        let r_res = is_balanced_recursive_impl(&nd.right);

        let height_diff: i32 = l_res.height - r_res.height;

        IntermediateResult {
            is_balanced: l_res.is_balanced && r_res.is_balanced && (height_diff.abs() <= 1),
            height: std::cmp::max(l_res.height, r_res.height) + 1,
        }
    }


// pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {

//     match root {

//         Some(root)=>{
//             let left = root.borrow().left.clone();
//             let right = root.borrow().right.clone();

//             let left = self::is_balanced(left);
//             let right = self::is_balanced(right);
//             if left && right {
//                 return true
//             }
//             else{
//                 return false
//             }
//             // self::helper(cur_root, left_val, right_val)
//         }
//         None =>{
//             return true;
//         }

//     }
    
// }
// pub fn helper(cur_root: &RefCell<TreeNode>,left_val:i32,right_val:i32) ->  bool {

//     let left  = cur_root.borrow().left.clone();
//     let right  = cur_root.borrow().right.clone();
//     let mut  left_val = 0;
//     let mut right_val = 0;

//     match (left,right) {
//         (Some(left),Some(right)) =>{
//             left_val = longest_chain(&left,1,1 ,1);
//             right_val = longest_chain(&right,1,1,1 );
//             println!("left_val --> {} , right_val --> {} ",left_val,right_val);
//         }
//         (Some(left),None) =>{
//             left_val = longest_chain(&left,1,0 ,1);
//             println!("left_val bye --> {} ",left_val);
//         }
//         (None,Some(right)) =>{
//             right_val = longest_chain(&right,0,1 ,1);
//             println!("right_val --> {} ",right_val);
//         }
//         (None,None) => {
//             println!("hi");
//             return true
//         }
//     }

    
//     println!(" final left_val --> {} , right_val --> {} ",left_val,right_val);
//     if (left_val -right_val).abs() <2 {
//         return true
//     }
//     else{
//         println!("hi1");
//         return false
//     }
//     }

// pub fn longest_chain(cur_root: &RefCell<TreeNode>, mut cur_depth_left: i32,mut cur_depth_right: i32,mut actual_depth:i32) -> i32 {

//     let left  = cur_root.borrow().left.clone();
//     let right  = cur_root.borrow().right.clone();

//     match (left,right) {
//         (Some(left),Some(right)) => {

//             let temp_left  =  longest_chain(left.as_ref(), cur_depth_left,cur_depth_right,actual_depth);
//             let temp_right =  longest_chain(right.as_ref(), cur_depth_left,cur_depth_right,actual_depth);

//             if temp_left > temp_right {
//                 cur_depth_left += temp_left ;
//                 actual_depth += temp_left;
//             }
//             else {
//                 cur_depth_right += temp_right;
//                 actual_depth += temp_right;
//             }
//             println!("cur_depth_left --> {} , cur_depth_right --> {} ",cur_depth_left,cur_depth_right);
//         }
//         (Some(left),None) =>{
//             println!("cur_depth_left --> {} ",cur_depth_left);
//             let temp_left  =  longest_chain(left.as_ref(), cur_depth_left,cur_depth_right,actual_depth);
//             cur_depth_left += temp_left ;
//             actual_depth += temp_left;
            
//         }
//         (None,Some(right)) =>{
//             println!("cur_depth_right --> {} ",cur_depth_right);
//             let  temp_right =  longest_chain(right.as_ref(), cur_depth_left,cur_depth_right,actual_depth);
//             cur_depth_right += temp_right;
//             actual_depth += temp_right;
//         }

//         (None,None) =>{
//               cur_depth_left; 
//               cur_depth_right;
//         } 
//     }
//     actual_depth
// }







// match cur_root {

//     Some(cur_root) =>{
//      let left =  cur_root.borrow().left.clone();
//      match left {
//         Some(left) =>{
//             left_val +=1;
//             helper(left, left_val, 0);
//         }
//      }   
//     }
//     // Some(cur_root) => {
//     //     let left_sum = self::helper(cur_root.borrow().left.as_ref(),1,0);
//     //     let right_sum  = self::helper(cur_root.borrow().right.as_ref(),0,1);
//     // }
//     // None => {
//     //     return true
//     // }