fn main(){    
let mut a = String::from("hello");
let newref = fact(&mut a);
println!("{}",newref);
}

fn fact(x:&mut String)->&mut String{
    x.push_str("world");
    x
}
