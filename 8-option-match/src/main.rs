fn main() {
    let mut num:Option<i32> = None;
    num = Some(100);
    
    let sq = Sq(num);

    println!("{}",sq);

    match num{
        None => {
            println!("{}",Sq1(0))
        },
        Some(v)=>println!("{}",Sq1(v)),
    }

    num = None;
  //  num.unwrap_or_default(0)

    let r = Sq1(num.unwrap_or_default());
     println!("default:{}",r);

    let r = Sq1(num.unwrap()); // unwrap doens not work
     println!("unwrap:{}",r);

    let r = Sq1(num.expect("Something went wrong"));
    println!("is num panic!no result:{}",r);

}

// There is no null in rust

fn Sq(num:Option<i32>)->i32{
    match num {
       None => 0,
       Some(v)=> v * v 
    }
}

fn Sq1(num:i32)->i32{
    num * num
}