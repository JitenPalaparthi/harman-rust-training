fn main() {
    let mut c1 = Calc::new(1);

    let r = c1.add(1).sub(1).add(2).mul(2).div(2).get();

    println!("r:{}", r);

    //let rows= db.select("from products").filter(".id>1000").limit(10).offset(10).where("status=="active").exec()
}
// fluent api or chain of actions

// impl is a code should be existed,code should be there or code would be generated
trait TCalc {
    fn add(&mut self, d: i32) -> &mut impl TCalc;
    fn sub(&mut self, d: i32) -> &mut impl TCalc;
    fn mul(&mut self, d: i32) -> &mut impl TCalc;
    fn div(&mut self, d: i32) -> &mut impl TCalc;
    fn get(&self) -> i32;
}
// Working design but not a good design
trait ICalc {
    fn addt(&mut self, d: i32) -> &mut Calc;
    fn subt(&mut self, d: i32) -> &mut Calc;
    fn mult(&mut self, d: i32) -> &mut Calc;
    fn divt(&mut self, d: i32) -> &mut Calc;
    fn gett(&self) -> i32;
}

struct Calc {
    data: i32,
}

impl Calc {
    fn new(d: i32) -> Self {
        return Calc { data: d };
    }
}

impl TCalc for Calc {
    fn add(&mut self, d: i32) -> &mut impl TCalc {
        // dyn
        self.data += d; // AddAssign
        return self; // returning a mutable self which has implemented TCalc trait
    }
    fn sub(&mut self, d: i32) -> &mut impl TCalc {
        // dyn
        self.data -= d; // SubAssign
        return self;
    }
    fn mul(&mut self, d: i32) -> &mut impl TCalc {
        // dyn
        self.data *= d; //MulAssign
        return self;
    }
    fn div(&mut self, d: i32) -> &mut impl TCalc {
        // dyn
        self.data /= d; // DivAssign
        return self;
    }
    fn get(&self) -> i32 {
        // dyn
        return self.data;
    }
}

impl ICalc for Calc {
    fn addt(&mut self, d: i32) -> &mut Calc {
        // dyn
        self.data += d; // AddAssign
        return self; // returning a mutable self which has implemented TCalc trait
    }
    fn subt(&mut self, d: i32) -> &mut Calc {
        // dyn
        self.data -= d; // SubAssign
        return self;
    }
    fn mult(&mut self, d: i32) -> &mut Calc {
        // dyn
        self.data *= d; //MulAssign
        return self;
    }
    fn divt(&mut self, d: i32) -> &mut Calc {
        // dyn
        self.data /= d; // DivAssign
        return self;
    }
    fn gett(&self) -> i32 {
        // dyn
        return self.data;
    }
}

impl Calc {
    fn addc(&mut self, d: i32) -> &mut Calc {
        // dyn
        self.data += d; // AddAssign
        return self; // returning a mutable self which has implemented TCalc trait
    }
    fn subc(&mut self, d: i32) -> &mut Calc {
        // dyn
        self.data -= d; // SubAssign
        return self;
    }
    fn mulc(&mut self, d: i32) -> &mut Calc {
        // dyn
        self.data *= d; //MulAssign
        return self;
    }
    fn divc(&mut self, d: i32) -> &mut Calc {
        // dyn
        self.data /= d; // DivAssign
        return self;
    }
    fn getc(&self) -> i32 {
        // dyn
        return self.data;
    }
}

// Concrete type Calc -> VTable
