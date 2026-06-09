fn main() {
    let mut x: i32 = 100;
    {
        let r = &x;
    }

    let y = &x;

    // let mut x1 = &mut 100; // x1 life time is more than r1
    // {
    //     let mut r1 = 200;
    //     x1 = &mut r1; // x1 holds the ref of r1 which has the smaller lifetime than x1
    // } // r1 lives shorter than x1

    // println!("{}",x1);

    let s1 = "hello Wrold";

    let l = get_len(s1);

    let s1 = "Hello".to_string();
    let s2 = "World!".to_string();

    let s3 = get_max(&s1, &s2);


    {

        
        {
            let s4 = "Hello".to_string();
        {
                 let s5: String = "World!".to_string(); 
                  let s6 = get_max(&s4, &s5);
                  {
                     let s6 = get_max(&s4, &s5);
                  }
        }
    }

    }

    let fail= "mock failure".to_string();
    let O: Output = Output::Failure(&fail);
    println!("{}",fail);

}

fn get_len<'a>(s:&'a str)->usize{
     s.len()
}
// basic lifes are incorporated automaticalluy in rust using lifetime elision

fn get_max<'a>(s1:&'a  str,s2:&'a str)->&'a str{
    if s1.len()>s2.len(){
         return s1;
    }
   return s2;
}

enum Output<'a>{
    Failure(&'a String)
}

struct Person<'a>{
    Id:i32,
    Name:&'a str,
    Email:&'a str,
}

// strucs,enum,func,traits,generics,methods