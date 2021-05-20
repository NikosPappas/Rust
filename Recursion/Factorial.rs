use std::io;
fn main() {
    let mut _user_input;
    println!("Type the the number n: ");
    _user_input=String::new();
    io::stdin()
       .read_line(&mut _user_input)
       .expect("Failed to read line");
    let _user_input: u32=_user_input.trim().parse().expect("Type a positive number please");
    let result:u32 = factorial(_user_input);
    println!("The factorial of {} is {} ",_user_input,result);
}

fn factorial(x:u32)->u32{
  if x==0 || x==1{
   return 1;
  }
 else{
   return x*factorial(x-1);
 }
}
