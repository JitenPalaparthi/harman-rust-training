fn main() {

   let mut c= 100;
   let fn1=move |a:i32,b:i32|->i32{ // FnOnce
    return a+b+c;
   };
   let c = calc1(100, 200, |a:i32,b:i32|->i32{ // Fn
    return a+b+c;
   }); // static dispatch

   println!("{}",c);
   let c = calc2(100, 200,&|a:i32,b:i32|->i32{ // Fn
    return a+b+c;
   }); // dynamic dispatch
    println!("{}",c);

    let c = calc3(100,200,Box::new(&fn1));
     println!("{}",c);

     let mut vfunc: Vec<Box::<dyn Fn()>>=Vec::new();

     vfunc.push(Box::new(||{
        println!("Hello World");
     }));

       vfunc.push(Box::new(||{
        let (a,b)=(10,20);
        let c = a+b;
        println!("c:{}",c)
     }));


     for f in &vfunc{
        f();
     }


}


fn calc1(a:i32,b:i32,fn1: impl Fn(i32,i32)->i32)->i32{ // static dispatch
    fn1(a,b)
}

fn calc2(a:i32,b:i32,fn1: &dyn Fn(i32,i32)->i32)->i32{ // static dispatch
    fn1(a,b)
}

fn calc3(a:i32,b:i32,fn1: Box::<&dyn Fn(i32,i32)->i32>)->i32{ // static dispatch
    fn1(a,b)
}



// Fn(a:i32,b:i32,fn1: impl Fn(i32,i32)->i32)->i32 Fn or FnMut of FnOnce
// Fn  
// FnMut
// FnOnce

