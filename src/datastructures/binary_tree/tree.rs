use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;
use super::node::TreeNode;

pub struct BinaryTree {
    pub root: Option<Rc<RefCell<TreeNode>>>
}

impl BinaryTree {
    pub fn from(data: Vec<Option<i32>>) -> BinaryTree {
        let mut q: LinkedList<Rc<RefCell<TreeNode>>> = LinkedList::new();
        let mut i = 1;
        let mut root: Option<Rc<RefCell<TreeNode>>> = None;
        if let Some(val) = data[0] {
            q.push_back(TreeNode::new(val));
        }
        while q.len() > 0 && i < data.len() {
            let mut node = q.pop_back().unwrap();
            let mut node_ref = node.borrow_mut();
            if root.is_none() {
                root = Some(node.clone());
            }

            // add the left child
            if let Some(val) = data[i] {
                let new_node = TreeNode::new(val);
                node_ref.left = Some(new_node.clone());
                q.push_back(new_node);
            }
            i += 1;

            // add the right child
            if let Some(val) = data[i] {
                let new_node = TreeNode::new(val);
                node_ref.right = Some(new_node.clone());
                q.push_back(new_node);
            }
            i += 1;
        }
        BinaryTree { root }
    }
}