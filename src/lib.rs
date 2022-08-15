use std::{
    fmt::Display,
    sync::{Arc, Mutex, Weak},
};

#[derive(Debug)]
struct Node<T> {
    item: T,
    prev: Mutex<Weak<Node<T>>>,
    next: Mutex<Option<Arc<Node<T>>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Arc<Node<T>>>,
    tail: Option<Arc<Node<T>>>,
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
            let new_node = Arc::new(Node {
                item,
                prev: Mutex::new(Weak::new()),
                next: Mutex::new(None),
            });
            self.tail = Some(Arc::clone(&new_node));
            self.head = Some(Arc::clone(&new_node));
            self.size += 1;
            return;
        }

        let new_node = Arc::new(Node {
            item,
            prev: Mutex::new(Weak::new()),
            next: Mutex::new(Some(Arc::clone(&self.head.as_ref().unwrap()))),
        });
        *self.head.as_ref().unwrap().prev.lock().unwrap() = Arc::downgrade(&new_node);
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop_head(&mut self) {
        if self.size == 0 {
            panic!("");
        }

        if let Some(node) = &*self.head.clone().as_ref().unwrap().next.lock().unwrap() {
            self.head = Some(Arc::clone(node));
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
            let new_node = Arc::new(Node {
                item,
                prev: Mutex::new(Weak::new()),
                next: Mutex::new(None),
            });
            self.tail = Some(Arc::clone(&new_node));
            self.head = Some(Arc::clone(&new_node));
            self.size += 1;
            return;
        }

        let new_node = Arc::new(Node {
            item,
            prev: Mutex::new(Arc::downgrade(&self.tail.as_ref().unwrap())),
            next: Mutex::new(None),
        });

        *self.tail.as_ref().unwrap().next.lock().unwrap() = Some(Arc::clone(&new_node));
        self.tail = Some(new_node);
        self.size += 1;
    }

    pub fn pop_back(&mut self) {
        if self.size == 0 {
            panic!("");
        }

        if let Some(node) = self.tail.clone().unwrap().prev.lock().unwrap().upgrade() {
            *node.next.lock().unwrap() = None;
            self.tail = Some(node);
            self.size -= 1;
        } else {
            self.head = None;
            self.tail = None;
            self.size -= 1;
        }
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.size == 0 {
            write!(f, "[]")
        } else {
            let mut s = String::new();
            let mut cur = Some(Arc::clone(&self.head.as_ref().unwrap()));
            loop {
                let next = cur.as_ref().unwrap().next.lock().unwrap().clone();
                if let None = &next {
                    s += format!("{}", &cur.as_ref().unwrap().item).as_str();
                    break;
                }
                else {
                    s += format!("{}, ", &cur.as_ref().unwrap().item).as_str();
                }
                cur = next;
            }
            write!(f, "[{}]", s)
        }
    }
}
