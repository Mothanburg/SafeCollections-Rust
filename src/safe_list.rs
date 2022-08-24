use std::{
    iter::FusedIterator,
    marker::PhantomData,
    mem,
    sync::{Arc, Mutex, Weak},
};

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

#[derive(Clone, Debug)]
pub struct Iter<'a, T> {
    head: Option<Arc<Node<T>>>,
    back: Option<Arc<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

// into 迭代器
#[derive(Clone, Debug)]
pub struct IntoIter<T> {
    safe_list: SafeList<T>,
}


// 实现链表
impl<T> SafeList<T> {
    #[inline]
    pub fn new() -> SafeList<T> {
        SafeList {
            head: None,
            back: None,
            len: 0,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        if cfg!(debug) {
            if self.len == 0 {
                debug_assert!(self.head.is_none());
                debug_assert!(self.back.is_none());
                true
            } else {
                false
            }
        } else {
            self.len == 0
        }
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

    pub fn clear(&mut self) {
        *self = SafeList::new();
    }

    pub fn append(&mut self, other: &mut SafeList<T>) {
        if let None = self.back {
            mem::swap(self, other)
        } else {
            if let Some(other_head) = other.head.take() {
                *self.back.as_ref().unwrap().next.lock().unwrap() = Some(Arc::clone(&other_head));
                *other_head.prev.lock().unwrap() = Arc::downgrade(self.back.as_ref().unwrap());
                self.back = other.back.take();
                self.len += mem::replace(&mut other.len, 0);
            }
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head.clone(),
            back: self.back.clone(),
            len: self.len,
            marker: PhantomData {},
        }
    }

    // 无法实现
    // pub fn iter_mut(&mut self) -> IterMut<'_, T> {

    // }
}


// iter的实现只能是unsafe的，想不到办法
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.len == 0 {
            None
        } else {
            self.head.take().map(|head| unsafe {
                self.head = head.next.lock().unwrap().clone();
                if let None = self.head {
                    self.back = None;
                }
                self.len -= 1;

                let pnode = Arc::into_raw(head);
                Arc::decrement_strong_count(pnode);
                (*pnode).element.as_ref().unwrap()
            })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    fn last(mut self) -> Option<&'a T> {
        self.next_back()
    }
}

// 反向迭代器同理，只有unsafe实现
impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<&'a T> {
        if self.len == 0 {
            None
        } else {
            self.back.take().map(|back| unsafe {
                self.back = back.prev.lock().unwrap().upgrade();
                if let None = self.back {
                    self.head = None;
                }
                self.len -= 1;

                let pnode = Arc::into_raw(back);
                Arc::decrement_strong_count(pnode);
                (*pnode).element.as_ref().unwrap()
            })
        }
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {}

impl<T> FusedIterator for Iter<'_, T> {}


impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.safe_list.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.safe_list.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {}

impl<T> FusedIterator for IntoIter<T> {}

// clone trait实现
impl<T: Clone> Clone for SafeList<T> {
    fn clone(&self) -> Self {
        self.iter().cloned().collect()
    }
}

// into iterator实现
impl<T> IntoIterator for SafeList<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        IntoIter { safe_list: self }
    }
}

impl<'a, T> IntoIterator for &'a SafeList<T> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<T> Extend<T> for SafeList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for elt in iter {
            self.push_back(elt);
        }
    }
}

impl<T> FromIterator<T> for SafeList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SafeList::new();
        list.extend(iter);
        list
    }
}
