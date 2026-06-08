const PI: f32 = 3.14;

static DEFAULT: i32 = 0;

static GLOBAL_DEFAULT: i32 = 100;

fn main() {
    // function stack frame
    let num1: i32 = 4343;
    let char1 = 'A';
    let str1 = "Hello World";
    let float1: f32 = 234.2323;
    let ok1 = true;
    let float2 = 234.2323;
    //let ok2 = true;

    let b0 = 100;
    let b1 = Box::new(100);
    {
        let s1 = "Hello World".to_string();
        println!("{} {}", s1, b1);
    }

    println!("{PI},{DEFAULT} {GLOBAL_DEFAULT} {num1} {char1} {str1} {float1} {ok1} {float2}");

    let c = add(12.23, 43.23);

    let s = Sq(10);
    
    // drop(s);
}

fn add(a: f32, b: f32) -> f32 {
    a + b
}

fn Sq(a: i32) -> Box<i32> {
    let b = Box::new(a * a);
    return b;
}

// fn Sq1(a: i32) ->&i32 {
//     let b = a * a;
//     return &b;
// }


// type inference

// numbers --> i8,i16,132,i64,i128 ,u8,u16,u32,u64,u128,isize,usize,f32,f64
// bool -> true/fase
// string  -> &str,String
// char -> utf-8 chars
