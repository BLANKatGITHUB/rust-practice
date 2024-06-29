use std::{fs::File,io::ErrorKind};


fn main() {
    let check = File::open("hello.txt");

    let check = match check{
        Ok(file) => file,
        Err(err) => match err.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(f) => f,
                Err(e) => panic!("error occured {:?}",e),
            },

            diferr => panic!("error of {:?}",diferr),
        },
    };
}