fn main() {
   let a = 100;
   let b = a;
   println!("a:{} b:{}",a,b);

   let s1 = "Hello World! 你好吗? ".to_string(); //UTF-8
   let s2: String = s1;
   println!(" s2:{}",s2);

   let l = get_length(s2);

   println!("l:{} s2:{}",l,s2);

}

fn get_length(s:String)->usize{
    return s.len();
}
// ownership
// move
// borrow -> mutable and immutable
// lifetimes