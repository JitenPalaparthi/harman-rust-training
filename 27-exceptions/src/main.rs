fn main() {
    // let mut num =10;
    // num = num-num;
    // println!("Hello, world!{}",10/num);

    let arr1 = [10,20];

    let mut i =0;
    loop{
        if i == 4{
            break;
        }
        println!("arr1:{}",arr1[i]);
        i+=1;
    }

    let r = divide1(10,0).unwrap();

    let r1 = divide3(10, 2);

    // match r {
    //     MyResult::Ok(v) => println!("Result:{}", v),
    //     MyResult::Err(e) => println!("Error: Code:{} Msg:{}", e.code, e.Msg),
    // }

    let r = r1.unwrap(); // very bad code to use

    println!("Result:{}", r);

    let r = divide5(10, 0); // very bad code to use

    match r {
        MyResult::Ok(v) => println!("Result:{}", v),
        MyResult::Err(e) => println!("Error: Code:{} Msg:{}", e.code, e.Msg),
    }
}

fn divide1(n: i32, d: i32) -> Result<i32, String> {
    if d == 0 {
        return Err("divide by zero is not allowed".to_string());
    } else {
        let r = n / d;
        return Ok(r);
    }
}

fn divide2(n: i32, d: i32) -> i32 {
    let mut r = 0;
    (|| {
        if d == 0 {
            // send information to all threads
            panic!("panic:divide by zero") // the reason and also th
        } else {
            r = n / d;
        }
    })();
    return r;
}

fn divide3(n: i32, d: i32) -> MyResult<i32, ErrorInfo> {
    if d == 0 {
        return MyResult::Err(ErrorInfo::new(100, "Divide by zero".to_string()));
    } else {
        let r = n / d;
        return MyResult::Ok(r);
    }
}

fn divide4(n: i32, d: i32) -> Result<i32, ErrorInfo> {
    if d == 0 {
        return Err(ErrorInfo::new(100, "Divide by zero".to_string()));
    } else {
        let r = n / d;
        return Ok(r);
    }
}

fn divide5(n: i32, d: i32) -> MyResult<i32, ErrorInfo> {
    // let r = divide4(n, d)?; // success or failure --> Ok, Err
    // Ok(r)

    match divide4(n, d){
        Ok(r)=>MyResult::Ok(r),
        Err(e)=>MyResult::Err(e)
    }
}

// errors those can be handled
// errors those cannot be handled --> panic! , crash the application ,exit with nonzero

// pub enum Result<T, E> {
//     Ok( /* … */ ), // No error
//     Err( /* … */ ), // Error
// }

#[derive(Debug)]
struct ErrorInfo {
    code: i32,
    Msg: String,
}

impl ErrorInfo {
    fn new(code: i32, msg: String) -> Self {
        return Self { code, Msg: msg };
    }
}

enum MyResult<T, E> {
    Ok(T),  // No error
    Err(E), // Error
}

impl<T,E> MyResult<T,E>{
    fn unwrap(self) -> T
    where
        E: std::fmt::Debug,
    {
        match self {
            MyResult::Ok(t) => t,
            MyResult::Err(e) => panic!("something went wrong"),
        }
    }
}


//GDB