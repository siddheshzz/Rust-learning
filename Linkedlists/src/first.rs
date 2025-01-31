struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    root: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { root: None }
    }

    fn insert(&mut self, val: i32) {
        let new_node = Node{value: val, next: None};
        let mut current_node = &mut self.root;
        while let Some(ref mut node) = current_node {
            current_node = &mut node.next;
        }
        *current_node = Some(Box::new(new_node));
    }

    fn print_vals(& self) {
        let mut current_node = & self.root;
        while let Some(node) = current_node {
            println!("node value is {}", node.value);
            current_node = & node.next;
        }
    }

}

fn main() {
    let mut a: LinkedList = LinkedList::new();
    a.insert(1);
    a.insert(2);
    a.insert(3);
    a.insert(4);
    a.insert(5);
    a.print_vals();

    
}