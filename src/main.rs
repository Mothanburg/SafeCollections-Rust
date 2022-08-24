use rand::Rng;
use serde::Serialize;
use std::collections::LinkedList;
use std::time::SystemTime;
use safe_collections_bin::safe_collections::SafeList;

#[derive(Serialize)]
struct BenchRecord {
    length: u32,
    time_std: u128,
    time_safe: u128,
}

fn main() {
    let record_file = if cfg!(debug_assertions) {
        "bench_debug.csv"
    } else {
        "bench_release.csv"
    };

    let mut wtr = csv::Writer::from_path(record_file).expect("无法创建文件！");

    for n in (100..1000).step_by(50) {
        let bench_len = n;
        let record = bench(bench_len);
        wtr.serialize(record).expect("Serilization Error!");
    }

    for n in (1000..10000).step_by(500) {
        let bench_len = n;
        let record = bench(bench_len);
        wtr.serialize(record).expect("Serilization Error!");
    }

    for n in (10000..100000).step_by(5000) {
        let bench_len = n;
        let record = bench(bench_len);
        wtr.serialize(record).expect("Serilization Error!");
    }

    for n in (100000..1000000).step_by(50000) {
        let bench_len = n;
        let record = bench(bench_len);
        wtr.serialize(record).expect("Serilization Error!");
    }

    for n in (1000000..10000000).step_by(500000) {
        let bench_len = n;
        let record = bench(bench_len);
        wtr.serialize(record).expect("Serilization Error!");
    }
}

fn bench(length: u32) -> BenchRecord {
    let mut nums1: Vec<u32> = vec![];
    let mut nums2: Vec<u32> = vec![];
    for _ in 0..length {
        nums1.push(rand::thread_rng().gen());
        nums2.push(rand::thread_rng().gen());
    }

    // println!("开始测试, 长度为{}", length);

    let start = SystemTime::now();
    let mut list = LinkedList::from_iter(nums1);
    for _ in 0..length {
        list.pop_front();
    }
    for i in nums2 {
        list.push_front(i);
    }
    for _ in 0..length {
        list.pop_back();
    }
    let time_std = start.elapsed().unwrap().as_micros();
    // println!("标准库链表用时：{}us", time_std);

    let mut nums1: Vec<u32> = vec![];
    let mut nums2: Vec<u32> = vec![];
    for _ in 0..length {
        nums1.push(rand::thread_rng().gen());
        nums2.push(rand::thread_rng().gen());
    }

    let start = SystemTime::now();
    let mut list1 = SafeList::from_iter(nums1);
    for _ in 0..length {
        list1.pop_front();
    }
    for i in nums2 {
        list1.push_front(i);
    }
    for _ in 0..length {
        list1.pop_back();
    }
    let time_safe = start.elapsed().unwrap().as_micros();
    // println!("安全的链表用时：{}us", time_safe);

    // println!("二者相差：{}us", time_std.abs_diff(time_safe));
    // println!(
    //     "标准库链表的用时是安全链表的用时的{:.2}%",
    //     (time_std as f64) / (time_safe as f64) * 100f64
    // );
    // println!(
    //     "安全链表的用时是安全链表的{:.2}倍",
    //     (time_safe as f64) / (time_std as f64)
    // );
    // println!();
    BenchRecord {
        length,
        time_std,
        time_safe,
    }
}
