fn main() {
    
    let (age,gender)= (42,'f');

    // let t = (42,'M');

    let (a,b)=(10,20);

    let c = a+b/2+(a+b)*2 -(b-1)%2+100*4/2>100&&true;

    if age >=18 && gender=='F' || gender=='f'{
        println!("She is eligible for marriage age:{}",age);
    }else if age >=21 && gender=='M' || gender=='m'{
        println!("She is eligible for marriage age:{}",age);
    }else{
        println!("not a valid in")
    }    
    // arth , logical, comparision, bitwise 

    let r = if age >=18 && gender=='F' || gender=='f'{
        "She is Eligible"
    }else if age >=21 && gender=='M' || gender=='m'{
          "He is Eligible"
    }else{
         "Invalid"
    };

    println!("Eligibility : {}",r);

    let r =  {
            let c = a+b;
            let d =c*2;
            c/2+d -(b-1)%2+100*4/2>100&&true
    };
    println!("Result : {}",r);


    let s = "Hello, 世界 🚀 ❤️"; // s is a slice of a string

    println!("Len:{}",s.len());
    for c in s.chars(){
        println!("Char:{}",c);
    }

    for b in s.bytes(){
            println!("Byte:{}",b);
        }

    let mut c = 0;
        loop{
            if c>10{
                break
            }
            println!("This is a loop->{}",c);
            c+=1;
        }


let (mut c,mut d) = (1,1);

loop {
    if c<=5{
        loop{
            if d<=5{
                println!("c: {} d:{}",c,d);
                d+=1;
                
            }else{
                break;
            }
        }
        d=1;
        c+=1;
    }else{
            break;
        }
    }

    // let mut done= false;
    // for i in 1..=5{
    //     if done{
    //         break;
    //     }
    //     for j in 1..=5{
    //         println!("i:{} j:{}",i,j);
    //         if j==3{
    //             done = true;
    //             break;
    //         }
    //     }
    // }

    'out:
    for k in 1..=5{
    for i in 1..=5{
        for j in 1..=5{
            println!("k:{} i:{} j:{}",k,i,j);
            if j==3{
                break 'out;
            }
        }
    }
}

// use an expression from loop

let mut c=0;
let mut sum =0;
let r = loop{
    sum+=c;
    if c>5{
        break sum;
    }
    //println!("{}",sum);
    c+=1 
};

println!("{:?}",r);

}

// switch --> break (no switch in rust)

// loops  --> break

// continue in loops