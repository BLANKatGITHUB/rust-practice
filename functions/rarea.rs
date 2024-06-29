fn main(){
    let (l,b)=(7,3);
    let area = area(l,b);
    println!("area of rectangle is {}",area);
}

fn area(x:i32, y:i32) -> i32{
    x*y
}~