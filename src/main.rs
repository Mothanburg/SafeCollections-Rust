use linked_list::LinkedList;
use rand::Rng;
use std::{
    sync::{Arc, Mutex},
    thread
};

fn main() {
    let linked_list = Arc::new(Mutex::new(LinkedList::new()));

    let mut producers = vec![];
    for _ in 0..20 {
        let list = Arc::clone(&linked_list);
        let handle = thread::spawn(move || {
            if rand::thread_rng().gen_bool(0.5) {
                list.lock()
                    .unwrap()
                    .push_back(rand::thread_rng().gen_range(1..=20));
            } else {
                list.lock()
                    .unwrap()
                    .push_head(rand::thread_rng().gen_range(1..=20));
            }
        });
        producers.push(handle);
    }

    for p in producers {
        p.join().unwrap();
    }

    println!("produced: {}", linked_list.lock().unwrap());

    let mut consumers = vec![];
    for _ in 0..10 {
        let list = Arc::clone(&linked_list);
        let handle = thread::spawn(move || {
            if rand::thread_rng().gen_bool(0.5) {
                list.lock().unwrap().pop_back();
            } else {
                list.lock().unwrap().pop_head();
            }
        });
        consumers.push(handle);
    }

    for c in consumers {
        c.join().unwrap();
    }

    println!("comsumed: {}", linked_list.lock().unwrap());
}
