use std::io;
fn main() {
    let mut _user_input;
    println!("Type a positive number please: ");
    _user_input=String::new();
   io::stdin()
      .read_line(&mut _user_input)
      .expect("Failed to read line");
   let _user_input: u32 = _user_input.trim().parse().expect("Type a number please");
   print_binary_form(_user_input);
   println!();
}
fn print_binary_form(_user_input: u32){
   if _user_input==0{
      print!("{}",_user_input);
   }
   else{
        print_binary_form(_user_input/2);
        print!("{}",_user_input%2);
   }
}
