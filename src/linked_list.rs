use std::rc::Rc;
use std::option::Option;

struct Node {
    data: i32,
    next: Option<Rc<Node>>,
}


fn pretty_print_list(node: &Option<Rc<Node>>) {
    match *node {
        Option::Some(ref n) => {
            print!("{{ {} }} -> ", n.data);
            pretty_print_list(&n.next);
        },
        Option::None => println!("{{}}")
    };

}


fn copy_list(node: &Option<Rc<Node>>) -> Option<Rc<Node>> {
    match *node {
        Option::Some(ref n) => {
            let copy_val = n.data;
            let new_node = Rc::new (
                Node { data: copy_val, next: copy_list(&n.next) }
                );
            Option::Some(new_node)
        },
        Option::None => Option::None
    }
}


fn delete_from_list(node: &Option<Rc<Node>>, val: i32) -> Option<Rc<Node>> {
    match *node {
        Option::Some(ref n) => {
            if n.data == val {
                copy_list(&n.next)
            } else {
                let copy_val = n.data;
                let new_node = Rc::new (
                    Node { data: copy_val, next: delete_from_list(&n.next, val) }
                    );
                Option::Some(new_node)
            }
        },
        Option::None => Option::None
    }
}


fn insert_into_list(node: Option<Rc<Node>>, val: i32) -> Option<Rc<Node>> {
    let new_node = Rc::new (
        Node { data: val, next: node }
        );
    Option::Some(new_node)
}


fn main() {
    let mut last = Rc::new (
        Node { data: 11, next: Option::None }
    );
    for x in 0..10 {
        let next_node = Rc::new (
            Node { data: 10 - x, next: Option::Some(last.clone()) }
        );
        last = next_node;
    }
    let start = Rc::new (Node { data: 0, next: Option::Some(last.clone()) } );
    let start_opt = Option::Some(start);
    pretty_print_list(&start_opt);
    let node = delete_from_list(&start_opt, 5);
    pretty_print_list(&node);
    let new_node = insert_into_list(node, 18);
    pretty_print_list(&new_node);

}
