fn main() {
    // arrays are stored on stack
    // fixed length
    // can create multi dimentional arrays 
    let mut arr1:[i32;5] = [103,120,330,40,504]; // compiler must know the length
    let mut sum = 0;
    for e in &arr1{ 
        sum+=e;
        println!("{}",e);
    }

    println!("Sum:{}",sum);

    let (max,min)=get_max_min(arr1);
    println!("max:{} min:{}",max,min);

    let arr2 = [32,4446,5,56,43,5,575,4545,55,3576,676,5,565];

   // let (max,min)get_max_min(arr2);

    

    let slice1 = &arr1[..];
    let slice2 = &arr2[..];

    let (max,min) = get_max_min_by_slice(&arr1);
    println!("max:{} min:{}",max,min);
    
    let (max,min) = get_max_min_by_slice(slice2);
    println!("max:{} min:{}",max,min);

    let s1 = "Hello World!";
    let mut s2: String = "Hello World".to_string();

    //(&mut s2).push_str(" How are you doing!"); // 2015

    s2.push_str("Rust folks!");

    let l = get_len(s1);
    println!("len s1 :{}",l);
    let l = get_len(&s2);
    println!("len s2 :{}",l);

    let mut num = 10;

    num = num *num;

}


fn get_max_min(arr:[i32;5])->(i32,i32){
    let (mut max,mut min) =( arr[0],arr[0]);
    for e in arr{
        if e>max{
            max = e;
        }
        if e<min{
            min=e;
        }
    }
    (max,min)
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

fn get_len(s:&str)->usize{
    s.len()
}
