use std::io;
fn main() {
    let mut _user_input;
    let result:u32;
    println!("Type a positive number please: ");
    _user_input=String::new();
    io::stdin()
       .read_line(&mut _user_input)
       .expect("Failed to read line.");
    let _user_input:u32 = _user_input.trim().parse().expect("Type a positive number please");
    result=sum(_user_input);
    println!("The sum of the first {} is {}",_user_input,result);
}
fn sum(n: u32) -> u32{
  if n==0{
    return 0;
  }
  else{
    return n+sum(n-1);
  }
}
