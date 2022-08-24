pub mod safe_list;

pub use safe_list::SafeList;

#[cfg(test)]
mod list_tests {

    use std::{
        sync::{Arc, Mutex},
        thread, vec,
    };

    use super::SafeList;

    #[test]
    fn test_push_and_pop() {
        let mut list = SafeList::new();
        for i in 1..=1000 {
            list.push_back(i);
            assert_eq!(*list.back().unwrap(), i);
        }
        for j in 1001..=2000 {
            list.push_front(j);
            assert_eq!(*list.front().unwrap(), j);
        }
        assert_eq!(list.len(), 2000);
        for n in (1..=1000).rev() {
            let i = list.pop_back().unwrap();
            assert_eq!(n, i);
        }
        for n in (1001..=2000).rev() {
            let i = list.pop_front().unwrap();
            assert_eq!(n, i);
        }
        assert_eq!(list.len(), 0)
    }

    #[test]
    fn test_iter() {
        let mut list = SafeList::new();
        for i in 1..=100 {
            list.push_back(i);
        }
        let mut iter1 = list.iter();
        for i in 1..=100 {
            assert_eq!(i, *iter1.next().unwrap());
        }
        assert_eq!(None, iter1.next());
        for (i, j) in list.into_iter().zip(1..=100) {
            assert_eq!(i, j);
        }
    }

    #[test]
    fn test_from_and_append() {
        let vec1 = vec![1, 2, 3, 4, 5];
        let mut list = SafeList::from_iter(vec1);
        for (i, j) in list.iter().zip(1..=5) {
            assert_eq!(*i, j);
        }

        let vec2 = vec![5, 4, 3, 2, 1];
        let mut list2 = SafeList::from_iter(vec2);
        list.append(&mut list2);
        assert!(list2.is_empty());
        for (i, j) in list.into_iter().zip(vec![1, 2, 3, 4, 5, 5, 4, 3, 2, 1]) {
            assert_eq!(i, j);
        }
    }

    #[test]
    fn test_multithread() {
        let list: Arc<Mutex<SafeList<i32>>> = Arc::new(Mutex::new(SafeList::new()));
        let mut threads = vec![];

        for i in 0..100 {
            let l = Arc::clone(&list);
            let handle = thread::spawn(move || {
                l.lock().unwrap().push_back(i);
            });
            threads.push(handle);
        }
        for t in threads {
            t.join().unwrap();
        }

        let mut threads = vec![];
        for _ in 0..100 {
            let l = Arc::clone(&list);
            let handle = thread::spawn(move || {
                l.lock().unwrap().pop_back();
            });
            threads.push(handle);
        }
        for t in threads {
            t.join().unwrap();
        }
        assert_eq!(list.lock().unwrap().len(), 0);
    }
}
