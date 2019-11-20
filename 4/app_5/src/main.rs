fn main() {
    println!("Hello, ENUM!");
    let ipv4=IpAddress::V4;
    let ipv6=IpAddress::V6;
    println!("{:?}",ipv4);
    let home = IpAddr {
        kind:IpAddress::V4,
        address:String::from("127.0.0.1")
    };
    let loopback = IpAddr{
        kind:IpAddress::V6,
        address:String::from("::1")
    };
    let home2= IpAddress2::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddress2::V6(String::from("::1"));
    println!(" h is {:?} and l is {:?}",home2,loopback2);

    let home3 = IpAddress3::V4(127,0,0,1);
    let loopback3= IpAddress3::V6(String::from("::1"));
    println!(" h is {:?} and l is {:?}",home3,loopback3);

    let m= Message::Write(String::from("s: &str"));
    m.call();

    let q = Coin::Quater(UsState::Alabama(String::from("alabama")));
    println!("coin {:?}",value_in_cents(q));

    let five=Some(5);
    let six=plus_one(five);
    let none = plus_one(None);
    println!("six is {:?} none is {:?}",six.unwrap(),none.unwrap());

    /*let mut some_u8_value=Some(0u8);
    
    some_u8_value=Some(3);
    match some_u8_value {
        Some(3) => println!("three"),
        _=>()
    };*/
    let val=3;
    /*match val {
      3 => println!("three"),
      _ =>()
    };*/
    if let 3 = val {
        println!("it is three");
    }
    else{
        println!("__")
    }
    //println!("{}",val)
    
}
#[derive(Debug)]
enum IpAddress {
    V4,
    V6
}

struct IpAddr {
    kind:IpAddress,
    address:String,
}
#[derive(Debug)]
enum IpAddress2 {
    V4(String),
    V6(String)
}

#[derive(Debug)]
enum IpAddress3 {
    V4(u8,u8,u8,u8),
    V6(String)
}

#[derive(Debug)]
enum Message {
   Quit,
   Move {x:i32,y:i32},
   Write(String),
   ChangeColor(i32,i32,i32)
}
impl Message {
    // add code here
    fn call(&self){
        println!("{:?}",self)
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}
fn value_in_cents(coin:Coin) ->u32 {
    match coin {
        Coin::Penny=>{
            println!("lucky penny");1
        }
        Coin::Nickel=>5,
        Coin::Dime=>10,
        Coin::Quater(state) => {
            println!("State quarter from {:?}",state);
            25
        },
    }
}
#[derive(Debug)]
enum UsState {
   Alabama(String),
   Alaskas
}

fn plus_one(x:Option<i32>)->Option<i32>{
    match x {
        Some(i) => Some(i+1),
        None => Some(0),
    }
}