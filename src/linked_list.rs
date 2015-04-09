use std::rc::Rc;
use std::cmp::PartialEq;
use std::fmt::{self, Display};


struct LinkedList<T:PartialEq+Clone+Display> {
    data: T,
    next: Option<Rc<LinkedList<T>>>,
}

impl<T> LinkedList<T> 
        where T: PartialEq + Clone + Display{

    fn delete_val(&self, val: &T) -> Option<Rc<LinkedList<T>>> {
        if val == &self.data {
            match self.next {
                Some(ref n) => {
                    Some(n.clone())
                },
                None => None
            }
        } else {
            match self.next {
                Some(ref n) => {
                   Some(Rc::new (
                        LinkedList {data: self.data.clone(), next: n.delete_val(val)}
                        ))
                }
                None => {
                   Some(Rc::new (
                        LinkedList { data: self.data.clone(), next: None }
                        ))
                }
            }
        }
    }

    fn insert_val(&self, val: T) -> LinkedList<T> {
        LinkedList { data: val, next: Some(Rc::new(self.clone())) }
    }

    fn length(&self) -> i32 {
        match self.next {
            Some(ref n) => 1 + n.length(),
            None => 1
        }
    }

    /// Get a node from the list
    /// Return the node at position idx 
    /// Returns Option::None if the index is out of bounds
    fn get(&self, idx: i32) -> Option<LinkedList<T>> {
        if idx == 0 {
            Some(self.clone())
        } else {
            match self.next {
                Some(ref n) => n.get(idx-1),
                None => None
            }
        }
    }

    /// Find the position of a value
    /// Return the position of the first element in the list with value val
    /// Returns None if the value val is not in the list
    fn find(&self, val: T) -> Option<i32> {
        if self.data == val {
            Some(0)
        } else {
            match self.next {
                Some(ref n) => {
                    let pos = n.find(val);
                    match pos {
                        Some(ref n) => Some(n + 1),
                        None => None
                    }
                },
                None => None
            }
        }
    }
}

impl<T> Display for LinkedList<T>
    where T: Clone + PartialEq + Display {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Ignore the result as we return it later
            // The assignment avoids the must use compiler warning
            let _ = write!(f, "{{ {} }} -> ", self.data);
            match self.next {
                Some(ref n) => n.fmt(f),
                None => write!(f, "{{}}")
            }
        }
}


impl<T> PartialEq for LinkedList<T>
    where T: Clone + PartialEq + Display {
        fn eq(&self, other: &Self) -> bool {
            match self.next {
                Some(ref n) => {
                    match other.next {
                        Some(ref m) => {
                            (self.data == other.data) && (n.eq(m))
                        },
                        None => false
                    }
                },
                None => {
                    match other.next {
                        Some(_) => false,
                        None => self.data == other.data
                    }
                }
            }
        }
}

impl<T> Clone for LinkedList<T> 
    where T: Clone + PartialEq + Display{
        fn clone(&self) -> LinkedList<T> {
            let copy_val = self.data.clone();
            match self.next {
                Some(ref n) => {
                    LinkedList { data: copy_val, next:  Some(n.clone())}

                },
                None => {
                    LinkedList { data: copy_val, next: None}
                }
            }
        }
}

fn create_i32_list(start: i32, len: i32) -> Rc<LinkedList<i32>> {
    let mut last = Rc::new (
        LinkedList { data: start + len - 1, next: None }
    );
    for x in 0..(len - 2) {
        let next_node = Rc::new (
            LinkedList { data: start + len - 2 - x, next: Some(last.clone()) }
        );
        last = next_node;
    }
    Rc::new (LinkedList { data: start, next: Some(last.clone()) } )
}

#[test]
fn test_length() {
    let list = create_i32_list(0, 10);
    assert!(list.length() == 10);
    let list2 = create_i32_list(0, 15);
    assert!(list2.length() == 15);
}

#[test]
fn test_get() {
    let list = create_i32_list(0, 10);
    for x in 0..list.length() {
        // unwrap will panic if the node isn't retrieved properly
        assert!(list.get(x).unwrap().data == x);
    }
}

#[test]
fn test_get_oob() {
    let list = create_i32_list(0, 10);
    let get_val = list.get(15);
    match get_val {
        Some(_) => panic!("Index {} should be out of bounds!", 15),
        None => ()
    }
}

#[test]
fn test_find_exists() {
    let list = create_i32_list(0, 10);
    for x in 0..list.length() {
        assert!(list.find(x).unwrap() == x);
    }
}

#[test]
fn test_find_not_exists() {
    let list = create_i32_list(0, 10);
    let idx = list.find(15);
    match idx {
        Some(_) => panic!("Should not find 15"),
        None => ()
    }
}

#[test]
fn test_eq() {
    let list = create_i32_list(0, 10);
    let list2 = create_i32_list(0, 10);
    assert!(list == list2);
    let list3 = Rc::new ( LinkedList { data: 15, next: Some(list2) } );
    assert!(list != list3);
}

#[test]
fn test_clone() {
    let list = create_i32_list(0, 10);
    let list2 = list.clone();
    assert!(list2.length() == list.length());
    assert!(list == list2);
}

#[test]
fn test_insert_val() {
    let insert_val = 15;
    let list = create_i32_list(0, 10);
    let list2 = list.insert_val(insert_val);
    assert!(list.length() + 1 == list2.length());
    let find_idx = list2.find(insert_val);
   let get_val = match find_idx {
                     Some(n) => list2.get(n),
                     None => panic!("Should have found the value")

                 };
    match get_val {
        Some(ref n) => assert!(n.data == insert_val),
        None => panic!("Index should not be out of bounds")
    }

}

#[test]
fn test_delete_found() {
    let del_value = 5;
    let list = create_i32_list(0, 10);
    let del_list = list.delete_val(&del_value);
    match del_list {
        Some(ref n) => {
            let find_idx = n.find(del_value);
            match find_idx {
                Some(_) => panic!("Should not find deleted value"),
                None => ()
            }
            assert!(n.length() == list.length() - 1)
        },
        None => panic!("Should successfully return a value!")
    }
}

#[test]
fn test_delete_not_found() {
    let del_value = 15;
    let list = create_i32_list(0, 10);
    let del_list = list.delete_val(&del_value);
    match del_list {
        Some(ref n) => {
            let find_idx = n.find(del_value);
            match find_idx {
                Some(_) => panic!("Should not find value, it was never in the list"),
                None => ()
            }
            assert!(n.length() == list.length())
        },
        None => panic!("Should successfully return a value!")
    }
}

#[test]
fn test_delete_only_element() {
    let list = Rc::new ( LinkedList { data: 0, next: None});
    let del_list = list.delete_val(&0);
    match del_list {
        Some(_) => {
            panic!("The only element should have been deleted!")
        },
        None => ()
    }
}


fn main() {
    // An example creating a multi-dimensional structure
    let len = 10;
    let mut last = Rc::new (
        LinkedList { data: create_i32_list((len - 1) * 10, 10), next: None }
    );
    for x in 0..(len - 2) {
        let next_node = Rc::new (
            LinkedList { data: create_i32_list((len - x - 2) * 10, 10), next: Some(last.clone()) }
        );
        last = next_node;
    }
    let start = Rc::new (LinkedList { data: create_i32_list(0, 10), next: Some(last.clone()) } );

    for i in 0..10 {
        let row = start.get(i).unwrap().data;
        println!("{:-<60}", " ");
        for j in 0..10 {
            print!("| {: <3} ", row.get(j).unwrap().data);
        }
        println!("|");
    }
    println!("{:-<60}", " ");

    println!("Position [3,7]: {}", start.get(3).unwrap().data.get(7).unwrap().data);
}
