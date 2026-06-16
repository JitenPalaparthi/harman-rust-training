use std::{thread, time::Duration};
use std::sync::{Arc,Mutex};

//use std::rc::Rc

fn main() {
    let mut Counter = 0;

    let counter = Arc::new(Mutex::new(0));

    let counter_clone1=counter.clone();
    let counter_clone2=counter.clone();

    let handle1 = thread::spawn(move || {
        for i in 1..=1000001 {
            //thread::sleep(Duration::from_millis(50));
          let mut data = counter_clone1.lock().unwrap();
          *data+=1   
        }
    });

    let handle2 = thread::spawn( move || {
        for i in 1..= 1000000{
          let mut data = counter_clone2.lock().unwrap();
            *data-=1;
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Result: {}", *counter.lock().unwrap());
}
