use crate::{Output::Failure, Shape::Circle};

fn main() {

    let mut s = Shape::Circle(12.1);

    match Area_output(s){
         Output::Success(a, r)=>println!("Area of {} = {}",r,a),
         Output::Failure(r )=>println!("Area of {}",r),
    }

  //  s = Shape::Rect(12.2,13.4); // It is not assigining value, rather it is assigne  the entire Shape enum with the internal data

    match Area_output(s){
         Output::Success(a, r)=>println!("Area of {} = {}",r,a),
         Output::Failure(r )=>println!("Area of {}",r),
    }

}


enum Output {
    Success(f64,String),
    Failure(String),
}

#[derive(Debug)]
enum Shape {
    Cuboid(f32, f32, f32), // 12 bytes
    Rect(f32, f32),
    Circle(f32),
    Square(f32),
   // shape(&'a str),
}

impl Copy for Shape{   
}

impl Clone for Shape{
 fn clone(&self) -> Self{
    let s = self;
    *s
 }
}

fn Area_output(shape: Shape) -> Output {
    println!("{:p}",&shape);
    match shape {
        Shape::Rect(l, w) => {
            if l <=0.0 || w<=0.0{
                return Output::Failure("invalid Length or Width of a Rect".to_string())
            }
            return Output::Success((l*w)as f64, "Rect".to_string())
        }
        Shape::Square(s) => {
            if s<0.0{
                return Output::Failure("invalid Side of the Square".to_string())
            }
            return Output::Success((s*s)as f64, "Square".to_string())

        }
        Shape::Circle(r) => {
             if r<0.0{
                return Output::Failure("invalid Side of the Circle".to_string())
            }
            return Output::Success((3.14*r*r)as f64, "Circle".to_string())
        }
        Shape::Cuboid(l, w, h) => {
            if l <=0.0 || w<=0.0 || h<=0.0{
                return Output::Failure("invalid Length or Width or Height of a Cuboid".to_string())
            }
            return Output::Success((l*w*h)as f64, "Cuboid".to_string())
        }
        // Shape::shape(s)=>{
        //     return Failure(s)
        // }
    }
}

