fn main(){
    let mut x:[i32;5] = [0;5];
    x[0]=1;
    x[1]=2;
    x[2]=3;
    x[3]=4;
    x[4]=5;

    for element in x {
        println!("{}", element);
    }

}