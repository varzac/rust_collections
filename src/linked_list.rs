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

    fn length(&self) -> i32 {
        match self.next {
            Option::Some(ref n) => 1 + n.length(),
            Option::None => 1
        }
    }

    fn get(&self, idx: i32) -> Node<T> {
        if idx == 0 {
            self.clone()
        } else {
            match self.next {
                Option::Some(ref n) => n.get(idx-1),
                Option::None => panic!("Index out of bounds.")
            }
        }
    }

    fn find(&self, val: T) -> i32 {
        // return the position of the value passed in
        // -1 if it is not found
        if self.data == val {
            0
        } else {
            match self.next {
                Option::Some(ref n) => {
                    let pos = n.find(val);
                    if pos == -1 {
                        -1
                    } else {
                        1 + pos
                    }
                },
                Option::None => -1
            }
        }
    }
}

impl<T> PartialEq for Node<T>
    where T: Clone + PartialEq + Display {
        fn eq(&self, other: &Self) -> bool {
            match self.next {
                Option::Some(ref n) => {
                    match other.next {
                        Option::Some(ref m) => {
                            (self.data == other.data) && (n.eq(m))
                        },
                        Option::None => false
                    }
                },
                Option::None => {
                    match other.next {
                        Option::Some(_) => false,
                        Option::None => self.data == other.data
                    }
                }
            }
        }

        fn ne(&self, other: &Self) -> bool {
            !self.eq(other)
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

fn create_test_list(len: i32) -> Rc<Node<i32>> {
    let mut last = Rc::new (
        Node { data: len - 1, next: Option::None }
    );
    for x in 0..(len - 2) {
        let next_node = Rc::new (
            Node { data: len - 2 - x, next: Option::Some(last.clone()) }
        );
        last = next_node;
    }
    Rc::new (Node { data: 0, next: Option::Some(last.clone()) } )
}



fn main() {
    let start = create_test_list(10);
    let copy = start.clone();
    start.print();
    println!("Copy length: {}", copy.length());
    let copy = copy.delete_val(5);
    match copy {
        Option::Some(ref n) => {
            println!("Copy length: {}", n.length());
            n.print();
        },
        Option::None => println!("Empty List")
    }
    let start = start.insert_val(12);
    start.print();
}
