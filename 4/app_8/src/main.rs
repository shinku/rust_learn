//使用字符串存储 UTF-8 编码的文本

fn main() {
   
    let data="hello utf-8";
    println!("{:?}",data);
    let mut data2=String::from(data);
    data2.push_str(" this is shin");
    let str3=" bar";
    data2.push_str(str3);
    println!("{:?}",str3);
    println!("{:?}",data2.to_string());
    let s1=String::from("hello,");
    let s2=String::from("shinshin 小欣");
    let s3=format!("{}{}",s1,s2);
    println!("s1 is {},s2 is {}, s3 is {:?},s3.len is {:?}",s1,s2,s3,s3.len());
    println!("{:?}",&s3[..s3.len()]);
    //输出字符
    for c in s3.chars() {
        println!("{}",c);
    }
    //输出 字节
    for c in s3.bytes() {
        println!("{}",c);
    }

}

