use super::linked_list_1::LinkedList;
use super::node::Node;


fn print(head: Option<Box<Node>>) {
    let mut current = head.as_ref();
    while let Some(node) = current {
        print!("{} ", node.data);
        current = node.next.as_ref();
    }
    println!();
}

pub fn run() {
    let mut head = Node::new(1, None);
    let mut temp = &mut head;
    let arr: [i32; 4] = [1, 2, 3, 4];
    for val in arr.iter() {
        // create new node
        temp.next = Some(Box::new(Node::new(*val, None)));
        temp = temp.next.as_mut().unwrap();
    }
    if let Some(next_node) = head.next {
        print(Some(next_node));
    } else {
        println!("Empty list");
    }

    let mut ll = LinkedList::new();
    let data: [i32; 4] = [1, 2, 3, 4];
    for val in data.iter() {
        ll.add_last(*val)
    }
    ll.print();
    let val = ll.get_node_at(2);
    println!("{}", val.unwrap().data);
    ll.remove_last();
    ll.print();
    ll.remove_first();
    ll.print();
    ll.add_last(4);
    ll.print();
    ll.remove(1);
    ll.print();
    ll.reverse();
    ll.print();
}