use ds_rs::LinkedList;


fn main() {
    let mut ll: LinkedList<&str> = LinkedList::new(); 

    ll.push("v1");
    ll.push("v2");
    dbg!(&ll.head());
    dbg!(&ll.tail());
}
