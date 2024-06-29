fn main(){

    //variables are immutable by default 
    let a = 5;
    println!("the value of a {}",a);

    //we use mut to make variables 
    let mut y = 6;
    y = 7 ;
    println!("the value of y {}",y);

    //const are always immutable and always have to specify annotation during declaration
    const X:i32= 7;
    println!("the value of x is {cat}", cat=X);

    //we can shadow a variable by using let again its like making a new variable
    //when the new variable goes out of store the old variable is unshadowed again
    let z = 8;
    let z = 'a';
    println!("the value of z is {0} and x {1} ",z,X);
    //where (0) are positions of variables staring from zero 
    // like println("{0}{1}",x,y) where 0=x, 1=y so on
}