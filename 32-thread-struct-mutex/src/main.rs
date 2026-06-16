use std::{thread, time::Duration};
use std::sync::{Arc,Mutex};

//use std::rc::Rc
static mut Counter:i32=0; // Data Segment
fn main() {
    let counter = Arc::new(Mutex::new(Counter::new(unsafe{Counter})));
    let counter_clone1=counter.clone();
    let counter_clone2=counter.clone();

    let handle1 = thread::spawn(move || {
        for i in 1..=1000001 {
            //thread::sleep(Duration::from_millis(50));
          let mut counter_data = counter_clone1.lock().unwrap();
          (*counter_data).data+=1;
        }
    });

    let handle2 = thread::spawn( move || {
        for i in 1..= 1000000{
          let mut counter_data = counter_clone2.lock().unwrap();
          (*counter_data).data-=1;
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Counter.Data: {}", counter.lock().unwrap().data);

    let i = 100; // stack

    let mut box_i = Box::new(i); // heap allocated

    *box_i=200;

    println!("{i}");
}


struct Counter{
    data:i32
}

impl Counter{
    fn new(d:i32)->Self{
        return Counter { data: d }
    }
}