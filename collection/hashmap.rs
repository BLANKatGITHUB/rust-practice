use std::collections::HashMap;

fn main(){
    let mut score =  HashMap::new();

    score.insert("blue",10);
    score.insert("red",20);
    
    println!("{:?}",score);

    let x = score.get("blue").copied().unwrap_or(0);
    println!("{}",x);

    for (key,value) in &score{
        println!("{}:{}",key,value);
    }

    score.insert("blue",50);

    score.entry("blue").or_insert(10);

    for (key,value) in &score{
        println!("{}:{}",key,value);
    }

    let  text = "hello world wonderful world";

    for word in text.split_whitespace(){
        let count = score.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}",score);
}