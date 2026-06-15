use std::collections::HashMap;

fn main() {
    let mut mymap: HashMap<&str, &str> = HashMap::new();

    mymap.insert("560086", "Bangalore-1"); // borrow instance
    mymap.insert("560096", "Bangalore-2");
    mymap.insert("560034", "Bangalore-3");
    mymap.insert("560052", "Bangalore-4");
    mymap.insert("522001", "Guntur-1");
    mymap.insert("695011", "Trivandrum-1");

    let v = mymap.get("560086").unwrap();

    match mymap.get("5640086") {
        Some(v)=>println!("{}",v),
        None => println!("key not found")
    }

    let mut m2 = HashMap::new();
    m2.insert(100, 200);

   match m2.get(&100){
     Some(v)=>println!("{}",v),
     None => println!("key not found")
   }

   // mymap.remove(k)

    println!("{}", v);

    for (k,v) in mymap{
            println!("key:{} value:{}",k,v);
    }


}

// reference 
