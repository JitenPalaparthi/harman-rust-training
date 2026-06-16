use std::{thread, time::Duration};
fn main() {
    println!("Hello, world!");
    let mut sum = 0; // Copy trait
        
    let handler1= thread::Builder::new().stack_size(8*1024*1024).spawn(move ||->i32{
            for i in 1..=10{
                thread::sleep(Duration::from_millis(100));
                println!("Excuted by Thread-1 >> {}",i);
                sum+=i
        }
        return sum;
    });

    match handler1{
        Ok(handler)=>{
           let sum_of= handler.join().unwrap();
           println!("Sum_of:{}",sum_of);
        },
        Err(err)=>println!("Some error:{}",err)
    }


 println!("Global Sum:{}",sum);

} // exit of main -> exit the application

// Each process at least a single thread
// Threads contains memory
// can have multipe threads
// Process
//   Text
//   Data
//   Heap
//    Thread-1
//       Stack 2 mb
// 
// PC -> Next instruction to execute
// IR --> 
// Registers --? Hold the ongoing context
// SP
// Status
// Ref 

// Context Switch
// Scheduling -> OS

// 1000 threads --> 8 core
// Multiplexing --> M:N


// No thread would wait for other thread to complete its execution