use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    data: i32,
    next: Option<NodeRef>
}
type NodeRef = Rc<RefCell<Node>>;

impl Node {
    pub fn new(data: i32, next: Option<NodeRef>) -> NodeRef {
        Rc::new(RefCell::new(Node { data, next }))
    }
}

pub struct LinkedList {
    head: Option<NodeRef>,
    tail: Option<NodeRef>,
    size: i32
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None, tail: None, size: 0 }
    }

    pub fn size(&self) -> i32 {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn print_addresses(&self) {
        // Print the head address
        if let Some(head) = &self.head {
            println!("Head address: {:p}", Rc::as_ptr(head));
        } else {
            println!("Head: None");
        }

        // Print the tail address
        if let Some(tail) = &self.tail {
            println!("Tail address: {:p}", Rc::as_ptr(tail));
        } else {
            println!("Tail: None");
        }
    }
    pub fn print(&self) {
        if self.is_empty() {
            println!("Empty list");
            return;
        }
        let mut temp = self.head.clone();
        while let Some(node) = temp {
            let node_ref = node.borrow();
            print!("{} -> ", node_ref.data);
            temp = node_ref.next.clone();
        }

        println!("None");
        self.print_addresses();
    }

    pub fn add_first(&mut self, data: i32) {
        let node = Node::new(data, self.head.clone());
        if self.is_empty() {
            self.tail = Some(node.clone());
        }
        self.head = Some(node);
        self.size += 1;
    }

    pub fn add_last(&mut self, data: i32) {
        let node = Node::new(data, None);
        if self.is_empty() {
            self.head = Some(node.clone());
        } else if let Some(val) = self.tail.as_ref() {
            val.borrow_mut().next = Some(node.clone());
        }
        self.tail = Some(node);
        self.size += 1;
    }

    pub fn get_node_at(&self, index: i32) ->  Option<NodeRef>{
        if index == 0 || index == self.size {
            return None;
        }
        let mut temp = self.head.clone();
        for i in 0.. index {
            if let Some(node) = temp {
                temp = node.borrow().next.clone();
            }
        }
        temp
    }
}