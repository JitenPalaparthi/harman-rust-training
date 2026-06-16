use std::{thread, time::Duration};

#[tokio::main]
async fn main() { 
    let task1 = async{
        println!("Hello World") // is there anything to block?
    };

    let t1 = thread::spawn(||->i32{
            return sum_of_t(20);
    });
    
    let task_spawn= tokio::spawn(async move {
            // Process each socket concurrently.
            return sum_of(20).await;
    });

   let task2= sum_of(100);
   //println!("Task2 Sum:{}",task2);
   let task3= sum_of(100);

   

   let s=t1.join().unwrap();
     println!("thread sum:{}",s);
   let h=tokio::join!(task1,task3,task_spawn,task2);

    println!("task1 sum:{}",h.1);
    println!("spawn_task sum:{}",h.2.unwrap());
     println!("task1 sum:{}",h.3);
   // println!("task2 sum:{}",h.2);
}

async fn sum_of(r:i32)->i32{
    let mut sum = 0;
    for i in 1..=r{
         tokio::time::sleep(Duration::from_millis(100)).await;
        // thread::sleep(Duration::from_millis(100));
        sum+=i
    }
    return sum;
}

 fn sum_of_t(r:i32)->i32{
    let mut sum = 0;
    for i in 1..=r{
        // tokio::time::sleep(Duration::from_millis(100));
         thread::sleep(Duration::from_millis(10));
        sum+=i
    }
    return sum;
}

