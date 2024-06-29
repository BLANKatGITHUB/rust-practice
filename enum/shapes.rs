#[derive(Debug)]
enum Shape{
    Circle(f32),
    Rectangle(f32,f32,f32),
    
}

fn main(){
    let s1 = Shape::Circle(2.5);
    println!("The shape s1 is {:?}",s1);
}