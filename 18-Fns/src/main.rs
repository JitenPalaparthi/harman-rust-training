fn main() {
    (|| {
        // inline
        println!("Hello How are you doing!");
    })(); // this is a closure , creating it and executing it directly without storing in any variable

    let c = (|a: i32, b: i32| -> i32 { return a + b })(10, 20); // this is a closure , creating it and executing it directly without storing in any variable

    println!("c:{c}");

    let greet = "Hello World!";

    let fn1 = || {
        // Fn
        println!("{}", greet);
    };

    let fn1 = |a: i32, b: i32| -> i32 { return a + b };

    let c = fn1(10, 20);

    println!("c:{c}");

    |i: i32, j: i32| -> i32 {
        return j - 1;
    }; // dead code

    let r = 10 as u32;

    let fib = || {
        let (mut a, mut b) = (0 as u32, 1 as u32);
        for i in 1..=r {
            let t = a;
            print!("{a} ");
            a = b;
            b = b + t;
        }
        println!();
    };

    fib();

    let mut sum: u32 = 0;

    let r = 10;

    let mut fib = || {
        let (mut a, mut b) = (0 as u32, 1 as u32);

        for i in 1..=r {
            let t = a;
            print!("{a} ");
            a = b;
            b = b + t;
            sum += a;
        }
        println!();
    };

    fib();
    println!("sum:{}", sum);

    // FnOnce

    let mut sum: u32 = 0;

    let mut sum1:u32=0;

    let r = 10;

    let mut fib = move || { // move moves the ownership to the function from the global scope. All variables? Yes, later it understands whicha ever the
        // variables are used ,  only those ownership is transfer
        // copy trait as well
        

        // creates a new sum = 0;
        let (mut a, mut b) = (0 as u32, 1 as u32);

        for i in 1..=r {
            let t = a;
            print!("{a} ");
            a = b;
            b = b + t;
            sum += a;
           // sum1+=a;
        }
        println!();
    };

    fib();
    println!("sum:{}", sum);


    let mut s1 = "Hello World! ".to_string();
    let mut some_fn= move |s:&str|{
        s1.push_str(s);
        println!("{}",s1);
    };

    some_fn("How are you doing");

    // This progam does not run
   // println!("{}",s1);

     let mut some_fn= move |s:&str|{
        s1.push_str(s);
        println!("{}",s1);
    };

    let mut s1 = "hello World!"; // RO

    let mut s2 = "hello World!"; // RO

    let mut s3 = "hello World!"; // RO

    s1 = "hello Universe!"; //RO

    //s1 = "Hello Universe!"



}
    

// core::ops::function
// pub trait Fn<Args>
// where
//     Self: FnMut<Args>,
//     Args: Tuple,
