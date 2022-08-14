use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node<T> {
    item: T,
    prev: RefCell<Weak<Node<T>>>,
    next: RefCell<Option<Rc<Node<T>>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Rc<Node<T>>>,
    tail: Option<Rc<Node<T>>>,
    size: u32,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn front(&self) -> &T {
        if let None = &self.head {
            panic!("");
        } else {
            &self.head.as_ref().unwrap().item
        }
    }

    pub fn push_head(&mut self, item: T) {
        if self.size == 0 {
            let new_node = Rc::new(Node {
                item,
                prev: RefCell::new(Weak::new()),
                next: RefCell::new(None),
            });
            self.tail = Some(Rc::clone(&new_node));
            self.head = Some(Rc::clone(&new_node));
            self.size += 1;
            return;
        }

        let new_node = Rc::new(Node {
            item,
            prev: RefCell::new(Weak::new()),
            next: RefCell::new(Some(Rc::clone(&self.head.as_ref().unwrap()))),
        });
        *self.head.as_ref().unwrap().prev.borrow_mut() = Rc::downgrade(&new_node);
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop_head(&mut self) {
        if self.size == 0 {
            panic!("");
        }

        if let Some(node) = &*self.head.clone().as_ref().unwrap().next.borrow() {
            self.head = Some(Rc::clone(node));
            self.size -= 1;
        } else {
            self.head = None;
            self.tail = None;
            self.size -= 1;
        }
    }

    pub fn back(&self) -> &T {
        if let None = &self.tail {
            panic!("");
        } else {
            &self.tail.as_ref().unwrap().item
        }
    }

    pub fn push_back(&mut self, item: T) {
        if self.size == 0 {
            let new_node = Rc::new(Node {
                item,
                prev: RefCell::new(Weak::new()),
                next: RefCell::new(None),
            });
            self.tail = Some(Rc::clone(&new_node));
            self.head = Some(Rc::clone(&new_node));
            self.size += 1;
            return;
        }

        let new_node = Rc::new(Node {
            item,
            prev: RefCell::new(Rc::downgrade(&self.tail.as_ref().unwrap())),
            next: RefCell::new(None),
        });

        *self.tail.as_ref().unwrap().next.borrow_mut() = Some(Rc::clone(&new_node));
        self.tail = Some(new_node);
        self.size += 1;
    }

    pub fn pop_back(&mut self) {
        if self.size == 0 {
            panic!("");
        }

        if let Some(node) = self.tail.clone().unwrap().prev.borrow().upgrade() {
            *node.next.borrow_mut() = None;
            self.tail = Some(node);
            self.size -= 1;
        } else {
            self.head = None;
            self.tail = None;
            self.size -= 1;
        }
    }
}
