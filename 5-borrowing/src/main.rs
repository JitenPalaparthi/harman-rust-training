fn main() {
   let mut s1:String = String::new();

   s1.push_str("Hello World");

   {
   let s2 = &s1; // Borrowing , not ownership transfer
    // s1.push_str(" How are you doing!");
   }

   let s3= &s1;
   let s4 = &s1;
   let l =  get_length(&s1);

   println!("len:{}",l);
   println!("{}",s1); // automatically the ownsership is with s1
//    let mut st1: &str ="Hello World";
//    st1 = "hello rust folks";
  

// let a = 10;
// let a = 10.10;
// let a = "Hey A";

let r = &mut 100;
*r = 200; // deferefence it 
println!("{}",*r);

let s2 = &mut s1;
s2.push_str(" How are you doing!");

let s3 = &s1;
let s4: &String = &s1;
let s5 = &s1;

let l = get_len_mut(&mut s1, "I am from India");
let s6 = &mut s1;

println!("{}",s6);
}

fn get_length(s: &String)->usize{
    return s.len();
}

fn get_len_mut(s:&mut String,st:&str)->usize {
    s.push_str(st);
    return s.len()
}

// borrowing rules
/* 

 With in a scope , scope either you create or the compiler thinks(latest version of rust)
 there can be any number of immutable borrows
 or 
 there can be only one mutable borrow
*/

