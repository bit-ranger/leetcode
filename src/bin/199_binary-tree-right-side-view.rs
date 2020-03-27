//给定一棵二叉树，想象自己站在它的右侧，按照从顶部到底部的顺序，返回从右侧所能看到的节点值。
//
// 示例:
//
// 输入: [1,2,3,null,5,null,4]
//输出: [1, 3, 4]
//解释:
//
//   1            <---
// /   \
//2     3         <---
// \     \
//  5     4       <---
//
// Related Topics 树 深度优先搜索 广度优先搜索


//leetcode submit region begin(Prohibit modification and deletion)
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
use std::rc::Rc;
use std::cell::{RefCell};
use std::collections::HashMap;
use std::cmp::max;

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if let None = root {
        return Vec::with_capacity(0);
    }
    let mut rightmost_value_at_depth:HashMap<u32, i32> = HashMap::new();
    let mut max_depth = 0;
    let mut stack:Vec<(u32,Rc<RefCell<TreeNode>>)> = vec![(0, Rc::clone(&root.unwrap()))];
    while stack.len() > 0 {
        //last in first out
        let (depth, node) = stack.pop().unwrap();
        max_depth = max(max_depth, depth);
        if !rightmost_value_at_depth.contains_key(&depth){
            rightmost_value_at_depth.insert(depth, Rc::clone(&node).borrow().val);
        }

        //left first in
        if let Some(left) = &Rc::clone(&node).borrow().left{
            stack.push((depth+1, Rc::clone(left)));
        }
        //right last in
        if let Some(right) = &Rc::clone(&node).borrow().right{
            stack.push((depth+1, Rc::clone(right)));
        }

    }

    return (0..max_depth+1)
        .map(|x| rightmost_value_at_depth.get(&x))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap().clone())
        .collect::<Vec<i32>>();
}
//leetcode submit region end(Prohibit modification and deletion)


#[test]
fn right_side_view_test() {
    let root:Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode{
        val:1,
        left: Some(Rc::new(RefCell::new(TreeNode{
            val:2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode{
                val:5,
                left: None,
                right: None
            })))
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode{
            val:3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode{
                val:4,
                left: None,
                right: None
            })))
        })))
    })));

    let rst = right_side_view(root);
    assert_eq!(rst, [1,3,4]);
}