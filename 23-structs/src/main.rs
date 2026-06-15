use std::sync::Arc;

fn main() {

    // let p1 = P1{};
    // let p2 = P2{};

    // println!("{:p}",&p1);
    // println!("{:p}",&p2);

    // 0x16b4322d6
    // 0x16b4322d7

    let r = P1::area(10.34, 12.34);
    println!("Area:{}",r);
    let r = P2::area(12.234);
    println!("Area:{}",r);

    // area1(123.23,23.234);
    // area2(123.23);

}

struct P1 {} // empty structure
struct P2 {}

struct Shape{
    p1:P1,
    p2:P2
}

impl P1 {
    fn area(l: f32, b: f32) -> f64 { // only the size of the function stack frame
        return (l * b) as f64;
    }
}

impl P2 {
    fn area(s: f32) -> f64 { // only the size of the function stack frame
        return (s * s) as f64;
    }
}

// area(f32,f32)->f64
// area(f32)->f64

//  fn area1(l: f32, b: f32) -> f64 { // only the size of the function stack frame
//         return (l * b) as f64;
//     }

// fn area2(s: f32) -> f64 { // only the size of the function stack frame
//         return (s * s) as f64;
//     }