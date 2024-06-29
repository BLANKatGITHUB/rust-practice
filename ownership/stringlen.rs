fn main(){
    let s= String::from("hello, world");
    let n = length(&s);
    println!("the lenth of string \" {} \" is {}",s,n);
}

fn length(s1: &String)-> usize{
    s1.len()
}