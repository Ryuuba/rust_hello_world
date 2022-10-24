// This module implements a doubly-linked list
// Author: Adán G. Medrano-Chávez


// RefCell: A mutable memory location with dynamically checked borrow rules
// Rc: A single-threaded reference-counting pointer. ‘Rc’ stands for ‘Reference Counted’.
// Weak: a version of Rc that holds a non-owning reference to the managed allocation
use std::{
    cell::RefCell,
    rc::{Rc, Weak}
};

pub fn test() {
    println!("Test function can be accessed");
}



// This generic structure must implement the Copy trait to insert copies of the
// objects to be stored in the List
struct Node<T: Copy> {
    //Options means data could be None or Some.
    pub data: T,
    // A smart-pointer pointing to the next node if any.
    pub next: Option<Rc<RefCellNode<T>>>, 
    // A smart-pointer pointing to the prev node if any.
    pub prev: Option<Rc<Node<T>>>
}

impl<T> Node<T>{

    // builds a new node
    fn new(data: T, next: Option<Rc<Node<T>>>, prev: Option<Rc<Node<T>>>) -> Self {
        Self { data: data, next: next, prev: prev}
    }

}

pub struct DLList<T> {
    head: Option<Rc<Node<T>>>,
    tail: Option<Rc<Node<T>>>,
    len: usize
}

impl<T> DLList<T> {
    // Initializes an empty list
    fn new() -> Self {
        Self { 
            head: None, 
            tail: None, 
            len: 0 
        } 
    }

    fn push_front(&mut self, data: T) {
        let node: Node<T> = Node::new(data, None, None);
        match &self.head {
            None => {
                self.head = Some(Rc::new(node));
                self. = Some(Rc::new(node));
                self.tail = Some(Rc::clone(self.head.as_ref().unwrap()));

            }
            Some(ptr) => {
                &node.next = Some(Rc::clone(ptr.a);
            }
        }
        
        self.len += 1;
    }

}