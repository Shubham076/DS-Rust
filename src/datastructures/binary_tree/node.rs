use std::cell::RefCell;
use std::rc::Rc;

pub struct TreeNode {
    pub data: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(data: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {data, left: None, right: None}))
    }
}