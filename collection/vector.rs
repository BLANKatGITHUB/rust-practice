fn main(){
    let  mut v = vec![1,2,3,4];
    let mut v1:Vec<i32> = Vec::new();
    v1.push(6);
    v.insert(0,5);
    let mut x = &mut v[0];
    println!("{}",x);
    v.push(8);
    *x = 7;
    println!("{}",x);
    *x = 10;
    println!("{}",v[0]);
    
}