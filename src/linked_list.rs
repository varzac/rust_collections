use std::rc::Rc;
use std::option::Option;
use std::clone::Clone;
use std::cmp::PartialEq;
use std::fmt::Display;

struct Node<T:PartialEq+Clone+Display> {
    data: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> Node<T> 
        where T: PartialEq + Clone + Display{
    fn print(&self) {
        print!("{{ {} }} -> ", self.data);
        match self.next {
            Option::Some(ref n) => n.print(),
            Option::None => println!("{{}}")
        };
    }

    fn delete_val(&self, val: T) -> Option<Rc<Node<T>>> {
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
                        Node {data: self.data.clone(), next: n.delete_val(val)}
                        ))
                }
                Option::None => {
                   Option::Some(Rc::new (
                        Node { data: self.data.clone(), next: Option::None }
                        ))
                }
            }
        }
    }

    fn insert_val(&self, val: T) -> Node<T> {
        Node { data: val, next: Option::Some(Rc::new(self.clone())) }
    }
}

impl<T> Clone for Node<T> 
           where T: Clone + PartialEq + Display{
    fn clone(&self) -> Node<T> {
        let copy_val = self.data.clone();
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
