use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Hello, world,please guess the number");
    
    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("the secret number is :{}",secret_number);

    loop {
        println!("please input your number");
        let mut guess= String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");
        println!("your guessed :{}",guess);
        let guess:u64= match guess.trim().parse() {
            Ok(num)=>num,
            Err(_)=>continue
        };
        match guess.cmp(&secret_number) {
            Ordering::Equal =>{
                println!("you win");
                break;
            }
            Ordering::Greater=> println!("too big"),
            Ordering::Less=> println!("too small")
        }
    }

}
