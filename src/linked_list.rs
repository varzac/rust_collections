use std::rc::Rc;
use std::option::Option;
use std::clone::Clone;

struct Node {
    data: i32,
    next: Option<Rc<Node>>,
}

impl Node {
    fn print(&self) {
        print!("{{ {} }} -> ", self.data);
        match self.next {
            Option::Some(ref n) => n.print(),
            Option::None => println!("{{}}")
        };
    }

    fn delete_val(&self, val: i32) -> Option<Rc<Node>> {
        if val == self.data {
            match self.next {
                Option::Some(ref n) => {
                    Option::Some(n.clone())
                },
                Option::None => Option::None
            }
        } else {
            match self.next {
                Option::Some(ref n) => {
                   Option::Some(Rc::new (
                        Node {data: self.data, next: n.delete_val(val)}
                        ))
                }
                Option::None => {
                   Option::Some(Rc::new (
                        Node { data: self.data, next: Option::None }
                        ))
                }
            }
        }
    }

    fn insert_val(&self, val: i32) -> Node {
        Node { data: val, next: Option::Some(Rc::new(self.clone())) }
    }
}

impl Clone for Node {
    fn clone(&self) -> Node{
        let copy_val = self.data;
        match self.next {
            Option::Some(ref n) => {
                Node { data: copy_val, next:  Option::Some(n.clone())}

            },
            Option::None => {
                Node { data: copy_val, next: Option::None}
            }
        }
    }
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
    let copy = start.clone();
    start.print();
    let copy = copy.delete_val(5);
    match copy {
        Option::Some(ref n) => n.print(),
        Option::None => println!("Empty List")
    }
    let start = start.insert_val(12);
    start.print();

}
