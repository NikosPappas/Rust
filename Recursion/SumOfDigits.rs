use std::io;
fn main() {
    let mut _user_input;
       println!("Type a positive number please: ");
       _user_input=String::new();
      io::stdin()
         .read_line(&mut _user_input)
         .expect("Failed to read line");
      let _user_input:i32 = _user_input.trim().parse().expect("Failed to read line");
      let result:i32=find_sum_of_digits(_user_input);
      println!("The sum of digits of {}  is {}",_user_input,result);
}
fn find_sum_of_digits(n: i32) -> i32{
    if n==0{
       return 0;
    }
    else{
       return n%10+find_sum_of_digits(n/10);
    }
}
