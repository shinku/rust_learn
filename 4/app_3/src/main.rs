
// radom
use rand::prelude::*;
fn main() {
    println!("Hello, rand!");
    if rand::random()
    {
        println!("{}",rand::random::<u8>());
    }
    let mut rng=rand::thread_rng();
    let y:f64=rng.gen();
    println!("y is {}",y*100.0);
    let y:f64=rng.gen_range(0.0,100.0);
    println!("y is {}",y);
    let mut nums:Vec<i32>=(0..=1000).collect();
    println!("nums is {:?}",nums);
    
}
