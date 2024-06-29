fn main(){
    let result:i32 = factorial(5);
    println!("The factorial of number 5 is {}",result);
}

fn factorial(mut x:i32)->i32{
    let mut fact=1;
    loop{
        if x==0 {
            break;
        }

        fact=fact*x;
        x-=1;
    }
    fact
}