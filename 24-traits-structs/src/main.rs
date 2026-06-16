use std::fmt::Display;

fn main() {

    let mut r1 = Rect::new(10.12, 12.34);

    let a1 = r1.area();

    println!("{}:",a1);

}

trait Shape {
    type Output; // Assertive type
    fn area(&mut self) -> Self::Output;
    fn perimeter(&mut self) -> Self::Output;
}

#[derive(Debug)]
struct Rect {
    l: f32,
    b: f32,
    a: f32,
    p: f32,
}

impl Rect {
    fn new(l: f32, b: f32) -> Rect {
        return Rect {
            l,
            b,
            a: 0.0,
            p: 0.0,
        };
    }
}

impl Shape for Rect {
    type Output = f64;
    
    fn area(&mut self) -> f64 {
        self.a = self.b * self.b;
        return self.a as f64;
    }

    fn perimeter(&mut self) -> Self::Output {
        self.p = 2.0 * (self.b + self.b);
        return self.p as f64;
    }
}


fn print_shape_impl(s:&mut impl Shape<Output = f64>){
    println!("Area:{:?}",s.area());
}

fn print_shape_gen<'a,'b ,S:Shape<Output = f64>+Display+Copy>(s:&'a mut S){
    println!("Area:{}",s.area());
}


