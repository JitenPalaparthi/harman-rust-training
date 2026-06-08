fn main() {
   
   let t1 = ("Jiten",43,"JitenP@outlook.com","Trivandrum");
   let mut t2 = (100,true,"Hello World",423432);
   println!("{:?}",t1); // For the purpopse of debugging
   println!("Name : {} Age:{} Email:{} Address:{}",t1.0,t1.1,t1.2,t1.3);

   let (name,age,email,addr) = t1;
      println!("Name : {} Age:{} Email:{} Address:{}",name,age,email,addr);

      let  mut s1 = "hello World".to_string();
      let mut l:usize = 0;

      let s2 = s1 ;

      println!("Len:{}",s2.len());

      s1 = s2;

      (l,s1)=get_len(s1);

      println!("len: {} s1:{}",l,s1);

      let l = get_len_b(&mut s1);

      println!("len: {} s1:{}",l,s1);

}
 
// Display trait is not implemented for a tuple , so ask rust to print as a debug print

fn get_len(s:String)->(usize,String){
    return (s.len(),s);
}

fn get_len_b(s:&mut String)->usize{
    return s.len();
}