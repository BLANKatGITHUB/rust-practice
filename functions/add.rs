fn main(){
    let (a,b)=(4,8);
    let c;
    c=add(a,b);
    println!("{}",c);
}


fn add(x:i32,y:i32) -> i32 {
    x+y
}