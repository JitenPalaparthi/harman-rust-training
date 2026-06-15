
use std::cell::RefCell;
use std::rc::Rc;


fn main() {
   // let mut s1 = "Hello World! ".to_string();


    let s1: Rc<RefCell<String>> = Rc::new(RefCell::new(String::from("Hello World ")));
    let s1_clone1 = Rc::clone((&s1));
    let s1_clone2 = Rc::clone((&s1));
    let s1_clone3 = Rc::clone((&s1));

    
    // what is the count of RC --> 3

    // let mut s1 = "hello world".to_string();
    // let mut s2 = s1.clone();
    

    let mut some_fn = move |s: &str| {
       s1_clone1.borrow_mut().push_str(s);
       println!("s1:{}",s1_clone1.borrow());
    }; // rc 2

    some_fn("How are you doing");

    // println!("{}",s1);
    let mut some_fn = move |s: &str| {
       s1_clone2.borrow_mut().push_str(s);
        println!("s1:{}",s1_clone2.borrow());
    }; // rc 1
    some_fn("How are you doing");

    // rc:0
    // the moment ref count is 0, drop on the original data is called --> the original data is String, allocated on heap so that gets dropped
   
}

// core::ops::function
// pub trait Fn<Args>
// where
//     Self: FnMut<Args>,
//     Args: Tuple,
