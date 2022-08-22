pub mod safe_collections;

#[cfg(test)]
mod tests {

    use super::safe_collections::SafeList;

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
}
