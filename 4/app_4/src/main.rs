
// struct

fn main() {
    println!("Hello, Struct!");
    let mut user=User{
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };
    user.email=String::from("shin.gu@innotechx.com");
  
    println!("user is {:?}",user);
    let user2=User{
        ..user
    };
    println!("user2 is {:?}",user2);
    let black=Color(0,0,0);
    let origin=Point(1,1,1);
    println!("black and origin is {:?} and {:?}",black,origin);
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
    let rect=Rectangle{
        width:123,
        height:563
    };
    println!("rect's area is {}",rect.area())
}
fn area(dimer:(u32,u32))->u32{
    return dimer.0*dimer.1
}
#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32
}
impl Rectangle {
    // add code here
    fn area(&self)->u32{
        return self.width*self.height;
    }
}
#[derive(Debug)]
struct User {
    username:String,
    email:String,
    sign_in_count:u64,
    active:bool,
}
#[derive(Debug)]
struct Color (i32,i32,i32);
#[derive(Debug)]
struct Point(i32,i32,i32);