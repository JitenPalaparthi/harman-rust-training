fn main() {
    println!("Hello, world!");

    let fn1: fn(i32, i32) -> i32 = add;
    let r = fn1(10, 20);
    println!("{}", r);

    let r = calc(10, 20, add);

    println!("{}", r);

    let fn1 = calcR(10, 20);
    let r = fn1(10, 20);
    println!("{}", r);
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn calc(a: i32, b: i32, f: fn(i32, i32) -> i32) -> i32 {
    // fn functional pointer has some limintations
    return f(a, b);
}

fn calcR(a: i32, b: i32) -> fn(i32, i32) -> i32 {
    return |i: i32, j: i32| -> i32 { return i + j };
}
