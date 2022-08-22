use std::sync::{Arc, Mutex, Weak};

type MovBox<T> = Option<T>;

#[derive(Debug)]
struct Node<T> {
    element: MovBox<T>,
    prev: Mutex<Weak<Node<T>>>,
    next: Mutex<Option<Arc<Node<T>>>>,
}

#[derive(Debug)]
pub struct SafeList<T> {
    head: Option<Arc<Node<T>>>,
    back: Option<Arc<Node<T>>>,
    len: usize,
}

impl<T> SafeList<T> {
    pub fn new() -> SafeList<T> {
        SafeList {
            head: None,
            back: None,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn front(&self) -> Option<&T> {
        self.head
            .as_ref()
            .map(|node| node.element.as_ref().unwrap())
    }

    pub fn push_front(&mut self, elt: T) {
        match self.head.as_ref() {
            None => {
                let node = Node {
                    element: Some(elt),
                    prev: Mutex::new(Weak::new()),
                    next: Mutex::new(None),
                };
                self.head = Some(Arc::new(node));
                self.back = Some(Arc::clone(self.head.as_ref().unwrap()));
            }
            Some(head) => {
                let node = Node {
                    element: Some(elt),
                    prev: Mutex::new(Weak::new()),
                    next: Mutex::new(Some(Arc::clone(head))),
                };
                let node_arc = Arc::new(node);
                *head.prev.lock().unwrap() = Arc::downgrade(&node_arc);
                self.head = Some(node_arc);
            }
        }
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|mut head| {
            // 更新头节点
            self.head = head
                .next
                .lock()
                .unwrap()
                .as_ref()
                .map(|node| Arc::clone(node));

            match self.head.as_ref() {
                // 当链表只有一个元素时，头节点为None，消除掉对将要pop掉的节点的strong引用
                None => self.back = None,
                // 消除将要pop掉的的头节点的weak引用
                Some(cur_head) => *cur_head.prev.lock().unwrap() = Weak::new(),
            };
            debug_assert_eq!(Arc::weak_count(&head), 0);
            debug_assert_eq!(Arc::strong_count(&head), 1);

            // 只有消除了所有的weak引用同时只剩一个strong引用时，才可以获取Arc的可变引用
            let head_node = Arc::get_mut(&mut head).unwrap();
            self.head = head_node.next.lock().unwrap().take();
            self.len -= 1;
            head_node.element.take().unwrap()
        })
    }

    pub fn back(&self) -> Option<&T> {
        self.back
            .as_ref()
            .map(|node| node.element.as_ref().unwrap())
    }

    pub fn push_back(&mut self, elt: T) {
        match self.back.as_ref() {
            None => {
                let node = Node {
                    element: Some(elt),
                    prev: Mutex::new(Weak::new()),
                    next: Mutex::new(None),
                };
                self.head = Some(Arc::new(node));
                self.back = Some(Arc::clone(self.head.as_ref().unwrap()));
            }
            Some(back) => {
                let node = Node {
                    element: Some(elt),
                    prev: Mutex::new(Arc::downgrade(back)),
                    next: Mutex::new(None),
                };
                let node_arc = Arc::new(node);
                *back.next.lock().unwrap() = Some(Arc::clone(&node_arc));
                self.back = Some(node_arc);
            }
        }
        self.len += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.back.take().map(|mut back| {
            // 更新尾节点
            self.back = back.prev.lock().unwrap().upgrade();

            // 消除待pop的节点的strong引用
            match self.back.as_ref() {
                None => self.head = None,
                Some(cur_back) => *cur_back.next.lock().unwrap() = None,
            };
            self.len -= 1;

            debug_assert_eq!(Arc::strong_count(&back), 1);
            let back_node = Arc::get_mut(&mut back).unwrap();
            back_node.element.take().unwrap()
        })
    }
}
