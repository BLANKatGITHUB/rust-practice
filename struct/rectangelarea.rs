//to implement debug on struct type for display of struct
#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}

fn main(){
    let r1 = Rectangle{
        width:50,
        height:100,
    };

    let r2 = (50,100);

    println!("The area of the rectangle r1 is {}",area(&r1));
    println!("The area of the rectangle r2 is {}",tarea(r2));
    println!("The  r1 is {:?}",r1);//using :? to print in format easier for debug
    println!("The  r1 is {:#?}",r1);// making it look pretier :3

    //dbg! takes ownership and returns ownershhip  and gives output in standard error console stream
    let i = 5;
    let r3 = Rectangle{
        width:dbg!(50*i),
        height:1000,
    };

    dbg!(&r3);


    
}

fn area(rectangle:&Rectangle)->u32{
    rectangle.width*rectangle.height
}

fn tarea(dimensions:(i32,i32))->i32{
    dimensions.0*dimensions.1
}