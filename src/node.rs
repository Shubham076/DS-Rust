
pub struct Node {
    pub(crate) data: i32,
    pub next: Option<Box<Node>>
}

impl Node {
    pub fn new(data: i32, next: Option<Box<Node>>) -> Box<Node> {
        Box::new(Node { data, next })
    }
}