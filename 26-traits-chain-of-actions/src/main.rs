use std::ops::{
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
};
use std::fmt::{Display,Formatter};
use std::fmt::Result;
//use std::io::Result as IOResult;

trait TCalc<T> {
    fn add(&mut self, d: T) -> &mut Self;
    fn sub(&mut self, d: T) -> &mut Self;
    fn mul(&mut self, d: T) -> &mut Self;
    fn div(&mut self, d: T) -> &mut Self;
    fn get(&self) -> T;
    fn display(&self);
}


struct Calc<T> {
    data: T,
}

impl<T> Calc<T> {
    fn new(d: T) -> Self {
        Calc { data: d }
    }
}

impl<T> std::fmt::Display for Calc<T> where T:Display{
     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({})", self.data) // f32 or f64 or i32 or any number type
    }
}

impl<T> TCalc<T> for Calc<T>
where
    T: AddAssign + SubAssign + MulAssign + DivAssign + Copy+std::fmt::Display,
{
    fn add(&mut self, d: T) -> &mut Self {
        self.data += d;
        self
    }

    fn sub(&mut self, d: T) -> &mut Self {
        self.data -= d;
        self
    }

    fn mul(&mut self, d: T) -> &mut Self {
        self.data *= d;
        self
    }

    fn div(&mut self, d: T) -> &mut Self {
        self.data /= d;
        self
    }

    fn get(&self) -> T {
        self.data
    }
    fn display(&self){
        println!("Calc:{}",self)
    }
}

fn main() {
    let mut c1: Calc<i32> = Calc::new(1);

    let r = c1
        .add(1)
        .sub(1)
        .add(2)
        .mul(2)
        .div(2)
        .get();

    println!("r: {}", r);
    c1.display();

    let mut c2 = Calc::new(10.0);

    
    let mut r2: f64 = c2
        .add(5.0)
        .sub(2.0)
        .mul(3.0)
        .div(2.0)
        .get();

        c2.display();
    println!("r2: {}", r2);
}