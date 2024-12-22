pub struct Node {
    pub data: i32,
    pub next: Option<Box<Node>>
}

impl Node {
    pub fn new(data: i32, next: Option<Box<Node>>) -> Self {
        Node {
            data,
            next
        }
    }
}