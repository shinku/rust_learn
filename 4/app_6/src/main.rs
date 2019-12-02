///this is a test of cargo package

mod sound;
use std::collections::HashMap;
mod plant {
     #[derive(Debug)]
    pub struct Vegetables {
        pub name:String,
        id:i32
    }
    impl Vegetables {
        // add code here
        pub fn new (name:&str)->Vegetables{
            return Vegetables {
                name:String::from(name),
                id:1
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    sound::guitar();
    let mut v=plant::Vegetables::new("shin");
    println!("{:?}",v);
    println!("vegetable is {:?}",v.name);
    let mut map=HashMap::new();
    map.insert(1,2);
    println!("map is {:?}",map);
}