use std::time::Duration;
#[tokio::main]
async fn main() {

   let (tx,mut rx) = tokio::sync::mpsc::channel(10);
   
   let publisher = async{
         let mut i =0;
            loop{
            i+=1;
            tx.send(format!("Publisher-1 >> {i}")).await; // dont use unwrap , write proper error handling
        //}
        }
   };

   let receiver = async{
        while let Some(i) = rx.recv().await{
            tokio::time::sleep(Duration::from_millis(10)).await;
            println!("Received:{}",i);
        }
   };

 tokio::join!(publisher,receiver);
   
}
