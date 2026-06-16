use std::{thread, time::Duration};

static mut Counter: i32 = 0; // 'static

fn main() {
    //let mut Counter = 0;

    let handle1 = thread::spawn( || {
        for i in 1..=1000000 {
            //thread::sleep(Duration::from_millis(50));
            unsafe {
                Counter += 1;
            }
        }
    });

    let handle2 = thread::spawn( || {
        for i in 1..= 1000000{
            //thread::sleep(Duration::from_millis(50));
            unsafe {
                Counter -= 1;
            }
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Counter:{}", unsafe{Counter});
}
