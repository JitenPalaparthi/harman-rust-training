use std::{sync::mpsc, thread, time::Duration};

fn main() {

    let (tx1,rx)=mpsc::sync_channel(10);
    let tx2= tx1.clone();

    let producer_handle1: thread::JoinHandle<()> = thread::spawn(move||{
        // for i in 1..=100{
        let mut i =0;
       loop{
            //thread::sleep(Duration::from_millis(10));
            i+=1;
            tx1.send(format!("Publisher-1 >> {i}")).unwrap(); // dont use unwrap , write proper error handling
        //}
        }
    });

     let producer_handle2: thread::JoinHandle<()> = thread::spawn(move||{
         let mut i =0;
        loop{
           // thread::sleep(Duration::from_millis(10));
            i+=1;
            tx2.send(format!("Publisher-2 >> {i}")).unwrap(); // dont use unwrap , write proper error handling
        //}
        }
    });

    let receiver_handle= thread::spawn(move||{
        while let Ok(i) = rx.recv(){
            thread::sleep(Duration::from_millis(10));
            println!("Received:{}",i);
        }
    });

    producer_handle1.join().unwrap();
    producer_handle2.join().unwrap();
    receiver_handle.join().unwrap();

}
