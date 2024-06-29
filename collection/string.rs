fn main(){
    let mut s1 = String::new();
    s1 = "hello".to_string();

    s1.push_str(" world");

    let s2 =  String::from("I am learning");
    let s3 = String::from(" Rust");

    let s4 = s2 + &s3;
    //s2 goes out of scope bc add(self,s:&str) self tates ownership
    
    //returns string instead of printing it
    let s5 = format!("{s1}\n{s4}");

    println!("{s5}");

    for c in s5.chars(){
        println!("{c}");
    }

    for c in s5.bytes(){
        println!("{c}");
    }
}