fn main(){
    //types of integers 8,16,32,64,128 bits with prefix i and u for singed and unsigned
    //example i32,i64
    let x:i32 = -4;
    let y:u32 = 4;
    
    // for flot prefix is f with sizes 32 and 64
    let z:f64=0.1;

    //for character char with 4 byte size 
    let a:char='A';

    //for bool 
    let b:bool = false;

    //for tuple which is a compond datatype
    let tup:(i8,char,bool) = (1,'a',true);// can be declared as let tup= (1,'a',true)
    //also let tup:(x,y)=(5,6) , where x=5 ,y = 6

    println!("value of tup.0 {}, tup.1 {}, tup.2 {} ",tup.0,tup.1,tup.2);

    //array let a= [3;5] is equivalent of a=[3,3,3,3,3]
    let a:[i32;5];
}