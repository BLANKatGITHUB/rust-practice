fn main(){
    //an statement in rust perform some task and dont return value example:
    let x = 5;
    println!("{}",x);
    //unlike other languages assignment/reassignment dont return value 
    //so x=y=5 assigns 5 to both x and y in c is invalid in c
  
    // an expression evaluate a value and dont include semicilon
    //expression can be part of statement like 8 is an expression in this example
    let mut y = 8;

    //expression can be scope with curly bracets,calling macros,calling  function 

    y ={
        let z=1;
        
        println!("this is an expression");
        1+z
        //last line dont have semicolon adding will turn it into statement
        //statement dont return value
      
       };

       println!("{}",y);
    }
    //you can use return keyword or use expression as last line of scope or function to retuen a value
    