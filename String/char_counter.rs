use std::io;
fn main() {
    let mut _user_input:String=String::new();
    io::stdin().read_line(&mut _user_input).expect("Failed to read user's inpu");
    let chars:Vec<char>=_user_input.chars().collect();
    let mut _i:usize=0;
    let mut char_counter:u32=0;
    while _i<chars.len(){
          char_counter+=1;
          _i+=1;
    }//end while loop
     println!("{}",char_counter);
}
