use std::cell::RefCell;
use std::fmt::format;
use std::rc::Rc;
use super::node::TreeNode;
use super::tree::BinaryTree;

pub fn pre_order_traversal(root: Option<Rc<RefCell<TreeNode>>>) {
    if root.is_none() {return}
    let node = root.unwrap();
    let mut str = "".to_string();
    if let Some(node) = node.borrow().left.clone() {
        str.push_str(&format!("{} => ", node.borrow().data));
    } else {
        str.push_str("none => ");
    }
    str.push_str(&format!("{}", node.borrow().data));
    if let Some(node) = node.borrow().right.clone() {
        str.push_str(&format!(" <= {} ", node.borrow().data));
    } else {
        str.push_str(" <= none");
    }

    println!("{}", str);
    pre_order_traversal(node.borrow().left.clone());
    pre_order_traversal(node.borrow().right.clone());
}

pub fn run() {
    let data = vec![Some(1), Some(2), Some(3)];
    let tree = BinaryTree::from(data);
    pre_order_traversal(tree.root.clone())
}