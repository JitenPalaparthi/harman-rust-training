use std::iter::Sum;

fn main() {
    let arr1= [10,20,30];
    let vec1: Vec<i32> = vec![10,20,30];

    let r = get_max_min_by_slice(&vec1);
    println!("{:?}",r);
     let r = get_max_min_by_slice(&arr1);
    println!("{:?}",r);
    
    for v in &vec1{
        println!("{}",v);
    }

    let s = Sumof(&vec1);
    let s = Sumof(&vec1);
    let s = Sumof(&arr1);

    //let v1 = Vec::with_capacity(100);
}

fn Sumof(v:&[i32])->i64{
    let mut sum=0;
    for i in v{
        sum+=i;
    }
    return sum as i64;
}
fn get_max_min_by_slice(slice:&[i32])->(i32,i32){
    let (mut max,mut min) =(slice[0],slice[0]);
    for e in slice{
        if *e>max{
            max = *e;
        }
        if *e<min{
            min=*e;
        }
    }
    (max,min)
}

// slice is always borrowed