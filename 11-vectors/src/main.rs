static Counter: i32 = 0;
fn main() {
    let mut v1: Vec<i32> = vec![10, 20, 30, 40, 50];
    let mut arr1 = [10, 11, 12, 13, 14, 15];
    let s1 = "hello World";

    let mut arr2: [Box<i32>; 2] = [Box::new(10), Box::new(20)];

    v1.push(60);
    v1.push(70);
    println!("{:?}", v1);

    v1.pop();

    println!("{:?}", v1);

    println!("{:p} {:p}", v1.as_ptr(), &v1);
    println!("{:p} ", arr1.as_ptr());
    println!("{:p} {:p}", s1.as_ptr(), &s1);
    println!("{:p}", &Counter);

    let mut v1 = Vec::from_iter(arr1); //Vec::<i32>::new();
    v1.insert(0, 100);

    println!("{:?}", v1);

    unsafe {
        // let mut arr2 = [10, 20];
        let mut v1 =vec![10,20,30];
        let v2 = Vec::from_raw_parts(v1.as_mut_ptr(), v1.len(), v1.capacity());
        println!("{:?}", v2);
    }

    println!("Done");
}

// vec can grow at runtime
// vec is heap allocated
// the vec data, not the vec the variable
// vec header
// ptr 8 bytes
// len 8 bytes
// cap 8 bytes
