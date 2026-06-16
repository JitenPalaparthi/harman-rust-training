use std::{thread, time::Duration};
fn main() {
    println!("Hello, world!");
    let mut sum = 0; // Copy trait
        
    let handler1= thread::spawn(move ||->i32{
            for i in 1..=10{
                thread::sleep(Duration::from_millis(100));
                println!("Excuted by Thread-1 >> {}",i);
                sum+=i
        }
        return sum;
    });

    let handler2= thread::spawn(move ||->i32{
    for i in 1..=10{
        thread::sleep(Duration::from_millis(100));
        println!("Excuted by Thread-2 >> {}",i);
        sum+=i
    }
    return sum;
    });

    let handler3= thread::spawn(||->i32{return sum_of()});

   

match handler1.join(){
        Ok(k)=>println!("Sum:{:?}",k),
        Err(e)=> println!("There seems to be error :{:#?}",e)
}
let s= handler2.join().unwrap();
println!("Sum:{}",s);

let s = handler3.join().unwrap();
println!("Sum:{}",s);

 println!("Global Sum:{}",sum);

} // exit of main -> exit the application



fn sum_of()->i32{
    let mut sum: i32= 0;
            for i in 1..=10{
                thread::sleep(Duration::from_millis(100));
                println!("Excuted by Thread-1 >> {}",i);
                sum+=i
        }
        return sum;
}

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