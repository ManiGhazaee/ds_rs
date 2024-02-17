use std::{rc::Rc, time::Instant};

fn main() {
    let string = String::from("value");
    let rc1 = Rc::new(string);
    let inst = Instant::now();
    let rc2 = Rc::clone(&rc1); 
    dbg!(inst.elapsed().as_nanos());
    let inst = Instant::now();
    let rc3 = rc1.clone(); 
    dbg!(inst.elapsed().as_nanos());

    dbg!(rc1 == rc2);
    dbg!(rc1 == rc3);
}
