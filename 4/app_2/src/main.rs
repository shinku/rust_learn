

//learn 所有权
fn main() {
   
    let s=String::from("shin");
    
    //以下两个方法如果顺序相反，会报错，take_ownership_point 的参数为内存指向，不会释放所有权。take_ownership 会释放所有权
    take_ownership_point(&s);
    take_ownership(s);
    //    take_ownership(s);
    //    take_ownership_point(&s);
    //let s2=danger();//wrong;
    let s2:&String=& no_danger();//right;
    take_ownership_point(s2);
    show_string_bytes(s2);
    println!("first_word {}",first_word_index(s2));

    let s3=String::from("shin is here~");
    let slice=&s3[0..2];
    println!("{}",slice);
    let slice=&s3[0..=2];
    println!("{}",slice);
    let slice=&s3[..];
    println!("{}",slice);
    println!("first word of {} is {}",s3,first_word(&s3));
    //println!("{}",s3);
    

}
fn show_string_bytes(str:&String){
    let bytes=str.as_bytes();
    println!("bytes is {:?}",bytes);
    for(i,&item) in bytes.iter().enumerate(){
        //b"str"是个运算法，将双引号内的字符串转化为u32类型字节码。
        println!("{:?}",b"h");
        println!("{}",&item);
        println!("{}",i)
    }
}
fn first_word_index(str:&String)->usize{
    let bytes=str.as_bytes();
    for(i,&item) in bytes.iter().enumerate(){
        //b"str"是个运算法，将双引号内的字符串转化为u32类型字节码。
        if item == b' '
        {
            return i;
        }
    }
    return str.len();
}

fn first_word(s:&String)-> &str{
    let bytes=s.as_bytes();
    for(i,&item) in bytes.iter().enumerate(){
        if item==b' '{
            return &s[0..i]
        }
    }
    return &s[..];
}

fn take_ownership(somthing:String)
{
    println!(" take_ownership {} ",somthing)
}
fn take_ownership_point(sm:&String)
{
    println!(" take_ownership point {} ",& sm)
}

//fn danger()-> &String{
//    let s=String::from('hello');
//    &s;
    //会报错。因为s在danger 执行后会被释放掉，所以针对他的指针引用指向了一个无效的string
//}
fn no_danger()->String{
    let s=String::from("hello there");
    return s;
    //不会报错。retun 直接将s的所有权移交。
}