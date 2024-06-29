use std::fs;
use::std::collections::HashMap;

fn main(){
     let  s = String::from("/home/Blanky/coding/rust/error handling/hello.txt");
     let items = fs::read_to_string(s).unwrap();
     let mut  map = HashMap::new();

     for word in items.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
     }

     println!("{:?}",map);
}