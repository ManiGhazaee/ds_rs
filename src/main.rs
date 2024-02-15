use ds_rs::Queue;

fn main() {
    let mut q: Queue<Vec<&str>> = Queue::new(10);
    q.enq(vec!["v1"]).unwrap();
    q.enq(vec!["v2"]).unwrap();
    q.enq(vec!["v3"]).unwrap();
    q.enq(vec!["v4"]).unwrap();
    println!("{:?}", q);
}
