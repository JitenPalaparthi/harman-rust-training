fn main() {
   let mut r1 = Rect::new(10.45,76.43); // kind of a static funtion
   let s1 = Square::new(10.5);

   print_shape1(&r1);
   print_shape2(&s1);
   print_shape3(&r1);
   print_shape4(&r1);

//    print_shape_rect(&r1);
//    print_shape1_square(&s1);
//    print_shape1_rect(&r1);

}

#[derive(Debug)]
struct Rect{
    L:f32,
    B:f32,
}

trait Shape:What{
    type Output;
    fn area(&self)->Self::Output;
    fn perimeter(&self)->Self::Output;
}

trait What{
 fn what(&self)->String{
        return "Some Trait".to_string()
    }
}

impl Rect{
    fn new(l:f32,b:f32)->Rect{
        Self { L: l, B: b }
    }
   
}

impl Shape for Rect{
    type Output = f64;
     fn area(&self)->f64{
        return (self.L*self.B)as f64;
     }

    fn perimeter(&self)->f64{
        return (2.0 * (self.L+self.B)) as f64;
    }
}

impl What for Rect{
    fn what(&self)->String{
        return "Rect".to_string()
    }
}

struct Square(f32); // unit structure

impl Square{

    fn new(s:f32)->Self{
        return Self(s)
    }
}

impl What for Square{
    fn what(&self)->String{
        return "Square".to_string()
    }
}

impl Shape for Square{
    type Output = f64;
     fn area(&self)->f64{
        return (self.0*self.0)as f64
     }

    fn perimeter(&self)->f64{
        return (4.0 * (self.0))as f64;
    }
}






// This kind of a dependency injection
fn print_shape1<S:Shape<Output = f64>>(t:&S){ // static dispatch
    println!("Area of {}: {:?}",t.what(),t.area());
    println!("Perimeter of {}:{}",t.what(),t.perimeter());
}

fn print_shape2<S>(t:&S) where S:Shape<Output = f64>{ // static dispatch
    println!("Area of {:?}: {:?}",t.what(),t.area());
    println!("Perimeter of {}:{}",t.what(),t.perimeter());
}


fn print_shape3(t:&impl Shape<Output=f64>){ // static dispatch
    println!("Area of {}: {}",t.what(),t.area());
    println!("Perimeter of {}:{}",t.what(),t.perimeter());
}

fn print_shape4(t:&dyn Shape<Output=f64>){ // dynamic dispatch
    println!("Area of {}: {}",t.what(),t.area());
    println!("Perimeter of {}:{}",t.what(),t.perimeter());
}
