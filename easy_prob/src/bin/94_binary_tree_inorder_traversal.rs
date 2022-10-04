// Definition for a binary tree node.
use std::rc::Rc;
use std::cell::RefCell;

// Input: root = [1,null,2,3]
// Output: [1,3,2]

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
// impl From<Option<Rc<RefCell<TreeNode>>>> for TreeNode{
//   fn from(treenode: Option<Rc<RefCell<TreeNode>>>) -> Self {
//     unsafe {
//       * treenode.unwrap().borrow_mut()
//     }
        
//   }
// }

fn main() {
  let leaf = TreeNode{val:3,left:None,right:None};
  // let i:Option<Rc<RefCell<TreeNode>>> = leaf.into();
  let middle = TreeNode{val:2,left:leaf.into(),right:None};
  let root =TreeNode{val:1,left:None,right:middle.into()};

  println!("{:?}",inorder_traversal(root.into()));

}
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
  helper(root)
}

fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
  let mut output = Vec::new();

  match root {
      None => output,
      Some(root) => {
          output.append(&mut helper(root.borrow().left.clone()));
          output.push(root.borrow().val);
          output.append(&mut helper(root.borrow().right.clone()));
          println!("{:?}",output);
          output
      }
  }
}
// pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {




  
//   // // println!("{:?}", root.unwrap().as_ref().borrow_mut().right);
//   // // let mut vec:Vec<Option<Rc<RefCell<TreeNode>>>> = vec![root.clone()];
//   // let root_clone_tree:TreeNode = root.into().clone();
//   // let mut vec:Vec<i32> = vec![root_clone_tree.val]; 
//   // let  cloned = root.clone();
//   // while cloned.as_ref().unwrap().borrow_mut().left != None && cloned.as_ref().unwrap().borrow_mut().right != None {
//   //   if cloned.as_ref().unwrap().borrow_mut().left != None {
//   //      let left_tree:TreeNode = cloned.clone().into(); 
//   //      vec.push(left_tree.val);
//   //   }
//   //   else{
//   //     let i:TreeNode = cloned.clone().into();
//   //     vec.push(i.val);
//   //     let right_tree:TreeNode = cloned.clone().into(); 
//   //     vec.push(right_tree.val);
//   //   }
//   // }
//   // println!("{:?}",vec);
//   return vec![1]; 
// }

