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

    fn get(&self, idx: i32) -> Option<Node<T>> {
        // Return the node at position idx 
        // Option::None if the index is out of bounds
        if idx == 0 {
            Option::Some((self.clone()))
        } else {
            match self.next {
                Option::Some(ref n) => n.get(idx-1),
                Option::None => Option::None
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

#[test]
fn test_length() {
    let list = create_test_list(10);
    assert!(list.length() == 10);
    let list2 = create_test_list(15);
    assert!(list2.length() == 15);
}

#[test]
fn test_get() {
    let list = create_test_list(10);
    for x in 0..list.length() {
        let get_val = list.get(x);
        match get_val {
            Option::Some(ref n) => assert!(n.data == x),
            Option::None => panic!("Index {} should not be out of bounds", x)
        }
    }
}

#[test]
fn test_get_oob() {
    let list = create_test_list(10);
    let get_val = list.get(15);
    match get_val {
        Option::Some(_) => panic!("Index {} should be out of bounds!", 15),
        Option::None => ()
    }
}

#[test]
fn test_find_exists() {
    let list = create_test_list(10);
    for x in 0..list.length() {
        assert!(list.find(x) == x);
    }
}

#[test]
fn test_find_not_exists() {
    let list = create_test_list(10);
    assert!(list.find(15) == -1);
}

#[test]
fn test_eq() {
    let list = create_test_list(10);
    let list2 = create_test_list(10);
    assert!(list == list2);
    let list3 = Rc::new ( Node { data: 15, next: Option::Some(list2) } );
    assert!(list != list3);
}

#[test]
fn test_clone() {
    let list = create_test_list(10);
    let list2 = list.clone();
    assert!(list2.length() == list.length());
    assert!(list == list2);
}

#[test]
fn test_insert_val() {
    let insert_val = 15;
    let list = create_test_list(10);
    let list2 = list.insert_val(insert_val);
    assert!(list.length() + 1 == list2.length());
    let find_idx = list2.find(insert_val);
    assert!(find_idx != -1);
    let get_val = list2.get(find_idx);
    match get_val {
        Option::Some(ref n) => assert!(n.data == insert_val),
        Option::None => panic!("Index should not be out of bounds")
    }

}

#[test]
fn test_delete_found() {
    let del_value = 5;
    let list = create_test_list(10);
    let del_list = list.delete_val(del_value);
    match del_list {
        Option::Some(ref n) => {
            assert!(n.find(del_value) == -1);
            assert!(n.length() == list.length() - 1)
        },
        Option::None => panic!("Should successfully return a value!")
    }
}

#[test]
fn test_delete_not_found() {
    let del_value = 15;
    let list = create_test_list(10);
    let del_list = list.delete_val(del_value);
    match del_list {
        Option::Some(ref n) => {
            assert!(n.find(del_value) == -1);
            assert!(n.length() == list.length())
        },
        Option::None => panic!("Should successfully return a value!")
    }
}

#[test]
fn test_delete_only_element() {
    let list = Rc::new ( Node { data: 0, next: Option::None});
    let del_list = list.delete_val(0);
    match del_list {
        Option::Some(_) => {
            panic!("The only element should have been deleted!")
        },
        Option::None => ()
    }
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
