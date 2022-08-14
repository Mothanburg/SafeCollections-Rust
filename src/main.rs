use linked_list::LinkedList;

fn main() {
    let mut list = LinkedList::new();

    list.push_back(3);
    list.push_back(4);
    list.push_back(5);
    list.pop_head();
    list.push_head(6);
    list.pop_head();
    list.push_back(7);
    list.pop_back();
    println!("{:?}", &list);
}
