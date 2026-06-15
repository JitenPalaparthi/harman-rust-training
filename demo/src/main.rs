fn main() {
    println!("Hello, world!");

    let s1: &i32 = &100;
    {
        let num1: i32 = 100;
       // s1 = &num1;
    }

    let s2: i32 = 200;

    let s3 = make_big(s1, &s2);
    println!("{}",s3);

    let fn1 =||{
        println!("Hello World");
    };

    fn1();


    let fn1 = ||->Vec<i32>{
        let mut r=vec![];
        let (mut a,mut b)=(0,1);
        for i in 1..10{
            r.push(a);
            let t=a;
            a=b;
            b=t+b;
        }
        return r;
    };

    let v =fn1();
    println!("{:?}",v);


    //  let mut v1 = vec![];

    // let mut fn2 = || -> &[i32] {

    //     let (mut a, mut b) = (0, 1);

    //     for _ in 1..=10 {

    //         v1.push(a);

    //         let t = a;

    //         a = b;

    //         b = t + b;

    //     }
    //     v1.as_slice()
    // };

    // let result = fn2();

    // println!("{:?}", result);

     // v1.push(100);


     let mut counter=1;

     let mut fn1 = ||{
        counter+=1;
        println!("counter:{}",counter);
     };

     fn1();
     fn1();

     execute(fn1);


let add=|i:i32,j:i32|->i32{
    return i+j;
};

let r=calc(10,20,add);

println!("Result:{}",r);

let r=calc3(10,20,add);

println!("Result:{}",r);

let r=calc4(10,20,&add);

println!("Result:{}",r);



}


fn execute<F:FnMut()>(mut f:  F){
    f();
}

fn calc(a:i32,b:i32,fn1:fn(i32,i32)->i32)->i32{
    return fn1(a,b);
}


fn calc3(a:i32,b:i32,fn1:impl Fn(i32,i32)->i32)->i32{
    return fn1(a,b);
}

fn calc4(a:i32,b:i32,fn1:&dyn Fn(i32,i32)->i32)->i32{
    return fn1(a,b);
}

fn calc5(a:i32,b:i32,fn1:Box::<dyn Fn(i32,i32)->i32>)->i32{
    return fn1(a,b);
}

fn calc6(a:i32,b:i32,fn1:Box::<&dyn Fn(i32,i32)->i32>)->i32{
    return fn1(a,b);
}




fn calc1(a:i32,b:i32)->Box<dyn Fn(i32,i32)->i32>{
    let f = |a:i32,b:i32|->i32{
        a+b
    } ;
    return  Box::new(f);
}

fn make_big<'a,'b>(s1:&'a i32,s2:&'b i32 )->&'a i32 where 'b:'a{
   if s1>s2{
    return s1;
   }else{
    s2 
   }
}