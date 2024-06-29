fn max<T :std::cmp::PartialOrd>(list:&[T])-> &T {
    let mut max = &list[0];
    for item in list {
        if max < item{
            max = item ;
        }
    }

    max
}

fn main(){
    let a = vec![1,2,5,7,8,4];
    let b = vec!['a','b','c','m','z'];

    let result1 = max(&a);
    let result2 = max(&b);

    println!("{}{}",result1,result2);
}