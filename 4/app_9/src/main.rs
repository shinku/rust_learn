use std::collections::HashMap;
fn main() {
    let mut scores=HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("red"),50);
    println!("Hello, HashMap!");
    println!("{:?}",scores);

    let blue:String=String::from("blue");
    let red:String=String::from("red");
    let teams= vec![blue,red];
    let scores=vec![100,500];
    let scores: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();

    println!("{:?}",scores);
    println!("{:?}",scores);

    let text="hello wordld wonderful word";
    let mut map= HashMap::new();

    for word in text.split_whitespace() {
        println!("{:?}", word);
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("{:?}", map);

    let v=vec![1,2,4];
    let _av=get_averange(&v);
    println!("the max number is ,{:?}",find_the_max(&v));
    let mut v2=vec![12,6,3,4,200,400,29,199,192,23,12,100,3001,100,80];
    let len=v2.len()/2;
    println!("the sort,{:?}",sort(&mut v2[..]))
}
fn get_averange(vec:&Vec<i32>)->f32{
    let mut num:i32=0;
    for i in vec {
        //println!("{:?}",i);
        num+=i;
    }
    //num=num/vec.len();
    return  (num as f32)/vec.len() as f32;
}
fn find_the_max(vec:&Vec<i32>)->i32{
    let mut max:i32=0;
    for i in vec {
       if i>=&max {
           max=*i ;
       }
    }
    return max;
}
//冒泡排序
fn sort( vec:&mut [i32])->Vec<i32>{
    
    for i in 0..vec.len(){
        //println!{"{}",i};
        for j in i+1..vec.len() {
            //if(vec[j..j+1])
            //println!("{:?}",vec[j]);
            if vec[i] > vec[j] {
                let num=vec[i];
                vec[i]=vec[j];
                vec[j]=num;
            }
            println!("{:?}",vec);
            
        }
    }

    return vec.to_vec();
    
}

