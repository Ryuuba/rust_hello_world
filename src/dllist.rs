// This module implements a doubly-linked list
// Author: https://www.youtube.com/c/nyxtom
// Extended by: Adán G. Medrano-Chávez

// RefCell: A mutable memory location with dynamically checked borrow rules
// Rc: A single-threaded reference-counting pointer. ‘Rc’ stands for ‘Reference Counted’.
// Weak: a version of Rc that holds a non-owning reference to the managed allocation
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

// This typedef
type NodePtr<T> = Rc<RefCell<Node<T>>>;

// Node TDA, objects of type T must implement the Copy trait to be stored in the
// list
// Is it possible to move the object? Sometimes, the stored object isn´t needed
struct Node<T: Copy> {
    pub data: T,
    // Options means data could be None or Some.
    // A smart-pointer to the next node if any.
    pub next: Option<NodePtr<T>>,
    // A smart-pointer to the prev node if any.
    pub prev: Option<Weak<RefCell<Node<T>>>>, // comma is idiomatic
}

impl<T: Copy> Node<T> {
    // builds a new node, returns a struct node
    fn new(data: T) -> Self {
        Node {
            data,
            next: None,
            prev: None,
        } // semicolon isn´t needed because the last statement is returned
    }
}

// The From trait allows for a type to define how to create itself from another
// type, hence providing a very simple mechanism for converting between several
// types. There are numerous implementations of this trait within the standard
// library for conversion of primitive and common types.
impl<T: Copy> From<Node<T>> for Option<NodePtr<T>> {
    fn from(node: Node<T>) -> Self {
        Some(Rc::new(RefCell::new(node)))
    }
}

impl<T: Copy> From<Node<T>> for Option<Weak<RefCell<Node<T>>>> {
    fn from(node: Node<T>) -> Self {
        Some(Rc::downgrade(&Rc::new(RefCell::new(node))))
    }
}

pub struct DLList<T: Copy> {
    head: Option<NodePtr<T>>,
    tail: Option<NodePtr<T>>,
    len: usize,
}

impl<T: Copy> DLList<T> {
    // Initializes an empty list
    pub fn new() -> Self {
        DLList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_back(&mut self, value: T) {
        let mut node = Node::new(value);

        match &mut self.tail.take() {
            None => {
                self.head = node.into();
                self.tail = self.head.clone();
            }
            Some(current_tail) => {
                node.prev = Some(Rc::downgrade(&current_tail));
                self.tail = node.into();
                current_tail.borrow_mut().next = self.tail.clone();
            }
        }
        self.len += 1;
    }

    pub fn push_front(&mut self, value: T) {
        // Creates the new node to be inserted
        let mut node: Node<T> = Node::new(value);
        // Implements the rules to replace the list's head
        match &mut self.head.take() {
            // Takes the actual value of self.head
            None => {
                self.head = node.into(); // A reference becomes into a ptr
                self.tail = self.head.clone();
            }
            Some(current_head) => {
                node.next = Some(current_head.clone());
                self.head = node.into();
                // The 'if let' construct reads: "if 'let' destructures
                // '&self.head' into 'Some(i)', evaluate the block ('{}').
                if let Some(h) = &self.head {
                    current_head.borrow_mut().prev = Some(Rc::downgrade(&h));
                }
            }
        }
        self.len += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match &mut self.tail.take() {
            None => None,
            Some(tail) => {
                let mut tail = tail.borrow_mut();
                let prev = tail.prev.take();
                match prev {
                    None => {
                        // If there is not a previous node, then the list
                        // will be empty once the pop operation takes place,
                        // in consequence, head points to none.
                        self.head.take();
                    }
                    Some(prev) => {
                        let prev = prev.upgrade();
                        if let Some(prev) = prev {
                            prev.borrow_mut().next = None;
                            self.tail = Some(prev);
                        }
                    }
                };
                self.len -= 1;
                Some(tail.data)
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match &mut self.head.take() {
            None => None,
            Some(head) => {
                let mut head = head.borrow_mut();
                let next = head.next.take();
                match next {
                    None => {
                        // tail pointer changes to none
                        self.tail.take();
                    }
                    Some(next) => {
                        next.borrow_mut().prev = None;
                        self.head = Some(next);
                    }
                }
                self.len -= 1;
                Some(head.data)
            }
        }
    }

    pub fn size(&self) -> usize {
        return self.len;
    }
}

pub fn test_back() {
    println!("Test back starts");
    let mut list = DLList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);
    println!("list size = {}", list.size());
    assert_eq!(list.pop_back(), Some(4));
    println!("list size = {}", list.size());
    assert_eq!(list.pop_back(), Some(3));
    println!("list size = {}", list.size());
    assert_eq!(list.pop_back(), Some(2));
    println!("list size = {}", list.size());
    assert_eq!(list.pop_back(), Some(1));
    println!("list size = {}", list.size());
    assert_eq!(list.pop_back(), None);
    println!("list size = {}", list.size());
}

pub fn test_front() {
    println!("Test front starts");
    let mut list = DLList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    list.push_front(4);
    println!("list size = {}", list.size());
    assert_eq!(list.pop_front(), Some(4));
    println!("list size = {}", list.size());
    assert_eq!(list.pop_front(), Some(3));
    println!("list size = {}", list.size());
    assert_eq!(list.pop_front(), Some(2));
    println!("list size = {}", list.size());
    assert_eq!(list.pop_front(), Some(1));
    println!("list size = {}", list.size());
    assert_eq!(list.pop_front(), None);
    println!("list size = {}", list.size());
}
