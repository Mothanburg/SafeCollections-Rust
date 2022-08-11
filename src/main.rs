use linked_list::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    list.push_back(3);
    list.push_back(20);
    push(&mut list, 100);
    println!("{:?}", &list);
    list.pop_back();
    list.pop_back();
    list.push_back(50);
    println!("{:?}", &list);
}

fn push(list: &mut LinkedList<i32>, i: i32) {
    list.push_back(i);
}