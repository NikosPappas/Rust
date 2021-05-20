use std::io;
fn main() {
   let mut i: i32=1;
   let mut max: f64=-30.00;
   let mut min: f64=50.00;
   while i<365 {
       let mut input = String::new();
       io::stdin()
           .read_line(&mut input)
           .expect("Failed to read line");
       let input = input.trim().parse().expect("Please type a number");
       if input > max {
           max=input;
       }
       if input < min{
           min = input;
       }
       i=i+1;
   }//end of while loop
   println!("Max temperature:{} and Min temperature:{}",max,min);
}
