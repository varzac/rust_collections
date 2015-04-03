use std::rc::Rc;


enum OptionalNode {
    Value(Rc<Node>),
    Missing,
}


struct Node {
    data: i32,
    next: OptionalNode,
}


fn pretty_print_list(node: &OptionalNode) {
    match *node {
        OptionalNode::Value(ref n) => {
            print!("{{ {} }} -> ", n.data);
            pretty_print_list(&n.next);
        },
        OptionalNode::Missing => println!("{{}}")
    };

}


fn copy_list(node: &OptionalNode) -> OptionalNode {
    match *node {
        OptionalNode::Value(ref n) => {
            let copy_val = n.data;
            let new_node = Rc::new (
                Node { data: copy_val, next: copy_list(&n.next) }
                );
            OptionalNode::Value(new_node)
        },
        OptionalNode::Missing => OptionalNode::Missing
    }
}


fn delete_from_list(node: &OptionalNode, val: i32) -> OptionalNode {
    match *node {
        OptionalNode::Value(ref n) => {
            if n.data == val {
                copy_list(&n.next)
            } else {
                let copy_val = n.data;
                let new_node = Rc::new (
                    Node { data: copy_val, next: delete_from_list(&n.next, val) }
                    );
                OptionalNode::Value(new_node)
            }
        },
        OptionalNode::Missing => OptionalNode::Missing
    }
}


fn insert_into_list(node: OptionalNode, val: i32) -> OptionalNode {
    let new_node = Rc::new (
        Node { data: val, next: node }
        );
    OptionalNode::Value(new_node)
}


fn main() {
    let mut last = Rc::new (
        Node { data: 11, next: OptionalNode::Missing }
    );
    for x in 0..10 {
        let next_node = Rc::new (
            Node { data: 10 - x, next: OptionalNode::Value(last.clone()) }
        );
        last = next_node;
    }
    let start = Rc::new (Node { data: 0, next: OptionalNode::Value(last.clone()) } );
    let start_opt = OptionalNode::Value(start);
    pretty_print_list(&start_opt);
    let node = delete_from_list(&start_opt, 5);
    pretty_print_list(&node);
    let new_node = insert_into_list(node, 18);
    pretty_print_list(&new_node);

}
