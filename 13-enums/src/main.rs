use crate::Shape::Circle;

fn main() {
    let mut d: Direction = Direction::East;
    To_Travel(d);
    d = Direction::South;
    To_Travel(d);

    let t = (10, 20);
    println!("{:?}", t);

    let mut s = Shape::Circle(10.23);

    s = Shape::Cuboid(12.2, 12.4, 12.9);

    let a = Area(&s);

    println!("Area of {}={:.3}",a.1,a.0);

    s= Shape::Circle(12.4);

    let a = Area(&s);

    println!("Area of {}={:.3}",a.1,a.0);


    let mut a:f64=0.0;
    let mut r:String="".to_string();
    (a,r, s) = Area_Transfer(s);



    println!("Address of enum shape:{:p} size:{}",&s,std::mem::size_of::<Shape>());
    

    s = Shape::Cuboid(-1.0, 10.0, -1.1);
    let r: Output = Area_output(&s);

     match r{
         Output::Success(r,s )=> println!("Area of {} :{:.3}",r,s),
         Output::Failure(f)=>println!("{}",f),
     }


    let day: u8 = 10;
    match day {
        1 => println!("Sunday"),
        2 => println!("Monday"),
        3 => println!("Tuesday"),
        4 => println!("Wednesday"),
        5 => println!("Thursday"),
        6 => println!("Friday"),
        7 => println!("Saturday"),
        _ => println!("noday"), // default , _ is called blank identifer  is used as a default arm
    }
}

fn To_Travel(direction: Direction) {
    println!("To travel {:?}", direction)
}

#[derive(Debug)] // The compiler implements the Debug trait for the Direction
enum Direction {
    East,
    West,
    South,
    North,
}
enum Output {
    Success(f64,String),
    Failure(String),
}

enum Shape {
    Cuboid(f32, f32, f32), // 12 bytes
    Rect(f32, f32),
    Circle(f32),
    Square(f32),
}

fn Area(shape: &Shape) -> (f64,String) {
    match shape {
        Shape::Rect(l, w) => return ((l * w) as f64,"Rect".to_string()),
        Shape::Square(s) => return ((s * s) as f64,"Square".to_string()),
        Shape::Circle(r) => return ((3.14 * r * r) as f64,"Circle".to_string()),
        Shape::Cuboid(l, w, h) => return ((l * w * h) as f64,"Cuboid".to_string())
    }
}

fn Area_Transfer(shape: Shape) -> (f64,String,Shape) {
    match shape {
        Shape::Rect(l, w) => return ((l * w) as f64,"Rect".to_string(),shape),
        Shape::Square(s) => return ((s * s) as f64,"Square".to_string(),shape),
        Shape::Circle(r) => return ((3.14 * r * r) as f64,"Circle".to_string(),shape),
        Shape::Cuboid(l, w, h) => return ((l * w * h) as f64,"Cuboid".to_string(),shape)
    }
}

fn Area_output(shape: &Shape) -> Output {
    match shape {
        Shape::Rect(l, w) => {
            if *l <=0.0 || *w<=0.0{
                return Output::Failure("invalid Length or Width of a Rect".to_string())
            }
            return Output::Success((l*w)as f64, "Rect".to_string())
        }
        Shape::Square(s) => {
            if *s<0.0{
                return Output::Failure("invalid Side of the Square".to_string())
            }
            return Output::Success((s*s)as f64, "Square".to_string())

        }
        Shape::Circle(r) => {
             if *r<0.0{
                return Output::Failure("invalid Side of the Circle".to_string())
            }
            return Output::Success((3.14*r*r)as f64, "Circle".to_string())
        }
        Shape::Cuboid(l, w, h) => {
            if *l <=0.0 || *w<=0.0 || *h<=0.0{
                return Output::Failure("invalid Length or Width or Height of a Cuboid".to_string())
            }
            return Output::Success((l*w*h)as f64, "Cuboid".to_string())
        }
    }
}

