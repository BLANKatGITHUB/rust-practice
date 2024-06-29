fn main(){
   let s1 = student_info("Aaishwarya Mishra".to_string(),1,"cse".to_string());
   println!("Student name {} roll number {} branch {}",s1.name,s1.roll_no,s1.branch);

}

struct Student{
    name:String,
    roll_no:u32,
    branch:String,
}

fn student_info(name:String,roll_no:u32,branch:String)->Student{
   let s1 = Student{
        name,
        roll_no,
        branch,

    };

    s1

}