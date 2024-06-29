//loop can return value with break

fn main()
{
    let mut a = 0;
    println!("all the odd number from 1 to 10");
    loop{
      if a==10{
            break;
          }
        if a%2==0{
        a+=1;
        continue;
        }
    println!("{}",a);
    
      a+=1;
    }
    a=0;
    println!("even numbers from 1 to 10");
    while a<=10{
      if a%2!=0{
        a+=1;
        continue;
      }
      println!("{}",a);
      a+=1;
    }
// range starts counting from start and end  before the last 
//1..4 this will start from 1 and end at 3 , 1,2,3
//rev method is used to reverse the code .rev()
println!("Reverse count down ");
    for element in (1..11).rev(){
      println!("{element}");
    }
}