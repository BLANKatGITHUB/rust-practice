fn main(){
    let x : Option<i8>= Some(5);
    let y : Option<i8> = None;

    let x = match x {
        Some(c) => c,
        None => 0,
    };

    

    println!("{:?}",x);
    println!("{:?}",y);
}