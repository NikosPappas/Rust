use std::io;
use std::fs::File;
fn main()->std::io::Result<()> {
 println!("Welcome to the file creator in Rust:");
 println!("Type the name of the file please: ");
 let mut _user_input:String=String::new();
 io::stdin()
    .read_line(&mut _user_input)
    .expect("Failed to read line");
 let file=File::create(_user_input);
 drop(file);//close file
 println!("File is created!!!"); 
 Ok(())
  
}
