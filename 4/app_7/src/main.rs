// this is a test of Vector;
fn main() {
    println!("Hello, vector!");
    let mut v:Vec<i32>= Vec::new();
    v.push(1);
    v.push(3);
   
    //vectors  里面存放struct 对象
    let mut v2:Vec<newStruct>= Vec::new();
    let s1=newStruct { name:String::from("shin22")};
    let s2=newStruct { name:String::from("shin223")};
    let s3=newStruct { name:String::from("shin444")};
    v2.push(s1);
    v2.push(s2);
    v2.push(s3);
    println!("{:?}",v2);
    println!("{:?}",v2.get(2));
    match v2.get(2) {
        Some(s3)=> println!("2 index of v is {:?}",s3),
        None=> println!("There is no third element."),
    };
    let mut first = v.get(0);
   
    v.push(100);
    first = v.get(0);
    println!("{:?}",first);
    println!("{:?}",&v[2]);
    for i in &mut v {
       *i+=100;
    };
    
    println!("{:?}",v);
    //在vec中存储枚举，可以有效的实现在vec 中存放不同类型的数据
    let mut v3 = vec![
        IpAddress::ipv4(127,0,0,1),
        IpAddress::ipv6(String::from(":::1"))
    ];
    println!("{:?}",v3);
}
#[derive(Debug)]
enum IpAddress {
    ipv4(i32,i32,i32,i32),
    ipv6(String),
    address(u32),
}
#[derive(Debug)]
struct newStruct {
    name: String
}
