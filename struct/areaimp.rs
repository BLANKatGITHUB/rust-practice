struct Rectangle{
    width:u32,
    height:u32,
}

fn main(){
   
    let r1 =  Rectangle::rectangle(20,50);
    let r2 = Rectangle::rectangle(10,40);

    println!("The area of the rectangle r1 is {}",r1.area());
    println!("The area of tr1 is bigger than r2  {}",r1.compare(r2));
   
    
}

impl Rectangle{

fn area(&self)->u32{
    self.width*self.height
}
fn rectangle(width:u32,height:u32)->Self{
    Self{
        width,
        height,
    }
}

fn compare(&self,other:Rectangle)->bool{
    if self.width*self.height > other.width*other.height
    {
        true
    }

    else{
        false
    }
}

}