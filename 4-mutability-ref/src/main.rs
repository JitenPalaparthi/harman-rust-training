static COUNTER:i32=0;
static GCOUNTER:i32=999;
fn main() {
   
   let mut a = 10;
   a+=1;

    let mut s1: &str = "Hello World"; // RO Data Segment
    let s2 = "Hello World".to_string();

    s1 = "Hello World! ❤️ How are you doing?";
    
    s1 = "Hello world! ❤️ How are you doing?";

    let s3 = "Hello world! ❤️ How are you doing?";

    s1 = s2.as_str();

    let mut s4:String = s1.to_string();

    let s5:&str = "hello World";

    let num1 = 100;

    let ref_num1 = &num1;
    let ref_num2 = &num1;

    let s5:String = "hello how are you doing!".to_string();
    println!("{} {}",*ref_num1,ref_num2);
    println!("address of num1:{:p}",&num1);
    println!("address of Counter:{:p} Address of GCounter:{:p}",&COUNTER,&GCOUNTER);
    println!("address of S5:{:p}",s5.as_ptr());
    

}

// owned 
// borrowed

// nm

// 100038 518 -> 0x104aa0 518 
// 100038 51c -> 0x104aa0 51c
