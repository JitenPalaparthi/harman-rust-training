fn main() {
    
    let arr2d:[[i32;2];2] = [[1,2],[3,4]];

    let arr3d = [[[1,2],[3,4],[4,5]],[[6,7],[8,9],[10,11]]];

    for a1 in arr2d{
        for e in a1{
            print!("{} ",e);
        }
        println!();
    }

    for a1 in arr3d{
        for a2 in a1{
            for e in a2{
                print!("{} ",e);
            }  
        }
        println!();
    }

    let mut arr1 = [10,20,30,40,50,60,70,80,90,100];

    let slice1 = &arr1[..];
    let slice2 = &arr1[..5];
    let slice3 = &arr1[3..=8];
    let slice4 = &arr1[5..];

    println!("slice1:{:?} address:{:p} len:{}",slice1,slice1.as_ptr(),slice1.len());
    println!("slice2:{:?} address:{:p} len:{}",slice2,slice2.as_ptr(),slice2.len());
    println!("slice3:{:?} address:{:p} len:{}",slice3,slice3.as_ptr(),slice3.len());
    println!("slice4:{:?} address:{:p} len:{}",slice4,slice4.as_ptr(),slice4.len());


    // raw poiners 
    // imutable and mutable raw pointers
    let p1: *const i32 = slice1.as_ptr();

    println!("{:p}",p1);

    let slice5: &mut [i32] = &mut arr1[3..=8];

    let p2: *mut i32 = slice5.as_mut_ptr(); // mutable raw pointer

    unsafe{
        *p2 = 9999;
    }

    println!("{:?}",arr1);

    let mut num:i32 = 100;

    let ptr:*const i32= std::ptr::null();

    println!("{:p}",ptr);

    
    let  mut ptr1:*mut i32 = std::ptr::null_mut();

    unsafe{
        ptr1= &mut num;
    }

}


