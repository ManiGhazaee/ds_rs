use std::time::Instant;

fn main() {
    let inst = Instant::now();
    let mut l = ds_rs::linked_list::LinkedList::new();
    for i in 0..10000 {
        l.push_back(i);
    }

    // let mut _sum = 0;
    // for i in l.iter() {
    //     _sum += i;
    // }
    dbg!(inst.elapsed().as_micros());

    let inst = Instant::now();
    let mut l = std::collections::linked_list::LinkedList::new();
    for i in 0..10000 {
        l.push_back(i);
    }

    // let mut _sum = 0;
    // for i in l.into_iter() {
    //     _sum += i;
    // }
    dbg!(inst.elapsed().as_micros());
}
