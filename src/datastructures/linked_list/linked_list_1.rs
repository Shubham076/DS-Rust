use super::node::Node;

 pub(crate) struct LinkedList {
    head: Option<Box<Node>>,
    tail: Option<*mut Node>,
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

    pub fn print(&self) {
        if self.is_empty() {
            println!("Empty list");
        } else {
            let mut current = &self.head;
            while let Some(node) = current {
                print!("{} -> ", node.data);
                current = &node.next;
            }
            println!("None");
        }
    }

    pub fn add_first(&mut self, data: i32) {
        let node = Box::new(Node::new(data, self.head.take()));
        unsafe {
            let raw_node = Box::into_raw(node);
            if self.is_empty() {
                self.tail = Some(raw_node);
            }
            self.head = Some(Box::from_raw(raw_node));
            self.size += 1;
        }
    }

    pub fn add_last(&mut self, data: i32) {
        let node = Box::new(Node::new(data, None));
        unsafe {
            let raw_node = Box::into_raw(node);
            if self.is_empty() {
                self.head = Some(Box::from_raw(raw_node));
            } else if let Some(tail) = self.tail {
                (*tail).next = Some(Box::from_raw(raw_node));
            }
            self.tail = Some(raw_node);
        }
        self.size += 1;
    }

    pub fn get_node_at(&mut self, index: i32) ->  Option<&mut Box<Node>>{
        if index == 0 || index == self.size {
            return None;
        }
        let mut temp = self.head.as_mut();
        for i in 0.. index {
            if let Some(node) = temp {
                temp = node.next.as_mut();
            }
        }
        temp
    }

    pub fn remove_last(&mut self) {
        if let Some(node) = self.get_node_at(self.size - 2) {
            node.next = None;
            unsafe {
                let raw_pointer: *mut Node = &mut **node;
                self.tail = Some(raw_pointer);
            }
        }
    }

    pub fn remove_first(&mut self) {
        if self.size == 1 {
            self.head = None;
            self.tail = None;
        } else if self.size > 1 {
             self.head = self.head.take().and_then(| node| {node.next})
        }
    }

    pub fn remove(&mut self, index: i32) {
        if index == 0 || index == self.size {
            return;
        }
        if index == 0 {
            self.remove_first();
        } else if index == self.size - 1 {
            self.remove_last();
        } else {
            let mut temp = self.head.as_mut();
            for i in 0.. index - 1 {
                if let Some(node) = temp {
                    temp = node.next.as_mut();
                }
            }
            // Now `temp` is pointing to the node just before the one to remove
            if let Some(node) = temp {
                // Set the `next` of the previous node to skip the node to be removed
                if let Some(next_node) = node.next.as_mut() {
                    node.next = next_node.next.take();
                }
            }
        }
    }

    pub fn reverse(&mut self) {
        let mut prev: Option<Box<Node>> = None;
        let mut cur = self.head.take();
        while let Some(mut node) = cur {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            cur = next;
        }
        self.head = prev;
    }
}