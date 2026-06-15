use std::fmt;
use std::ops::AddAssign;
use std::{fmt::Display, ops::Add};

fn main() {
    let r = add(10, 20);
    //let r= add::<&str>("hello","World");

    let a: i32 = 10;
    let b: i32 = 20;

    let c = addg::<i32>(a, b);
    println!("c:{}", c);

    let c = addg::<f32>(12.34, 12.34);
    println!("c:{}", c);

    let c = addg(12.34, 12.34);
    println!("c:{}", c);

    let c: u16 = addg::<u16>(250 as u16, 132 as u16);
    println!("c:{}", c);

    // as long as it is a nubmer type that means, as long as it implements Add trait , that type can be passed as an input argument

    // Add trait

    //let c = addg::<bool>(true,false); // it is not at runtime..it is at compile time

    let p1 = Point::new(10.1, 12.3);
    let p2 = Point::new(14.5, 16.5);

    let p4 = Point::new(14.5, 16.5);

    // let p3= p1.add(p2);

    // let p3 = p1+p2;

    let p3: Point = addg(p1, p2);

    println!("{}", p3);
    println!("{:?}", p3);

    if p1 == p2 {
        println!("p1 and p2 are equal");
    } else {
        println!("p1 and p2 are not equal");
    }

    // let b= p2.eq(&p4);

    if p2 == p4 {
        // converted into the above
        println!("p2 and p4 are equal");
    } else {
        println!("p2 and p4 are not equal");
    }

    let mut p1 = Point::new(10.1, 12.1);
    let mut p2 = Point::new(10.1, 12.5);
    p1 += p2;
    println!("{p1}");

    p1.add_assign(p2);

    println!("{p1}");
}
// Trait based system.

fn add(a: i32, b: i32) -> i32 {
    a + b
}

// fn addg1<T>(a:T,b:T)->T{
//     return a+b; // It is arthemetic operation
// }

fn addg<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    return a + b;
}

fn subg<T: std::ops::Sub<Output = T>>(a: T, b: T) -> T {
    return a - b;
}

// T is generic type
// Add is a trait

#[derive(Debug, Copy, Clone)]
struct Point {
    X: f32,
    Y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        // it is a function, using it like a constructor
        // return Self { X: x, Y: y }
        Point { X: x, Y: y }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point: {{X:{}, Y:{}}}", self.X, self.Y)
    }
}

// There is a trait called Add
// operator overloading in c++
// struct BigPoint{
//    X:f64,
//    Y:f64
// }

// impl Add for Point{
//     type Output = BigPoint;
//     fn add(self, other: Self) -> BigPoint {
//         BigPoint {
//             X: (self.X + other.X) as f64,
//             Y: (self.Y + other.X) as f64,
//         }
//     }
// }

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            X: (self.X + other.X),
            Y: (self.Y + other.X),
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y
    }
}

/*

core::cmp::PartialEq
pub trait PartialEq<Rhs = Self>
pub fn eq(&self, other: &Rhs) -> bool
*/

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            X: self.X + other.X,
            Y: self.Y + other.Y,
        };
    }
}
