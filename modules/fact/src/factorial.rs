pub fn fact(x:i32)->i32{
    if x== 0 {
       return 1;
    }

    x*fact(x-1)
}