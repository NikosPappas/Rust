//The following programm stores the temperatures
//of month January in array collected at 8am.
//Then it calculates and prints the average value of temperatures
//and prints the days with temperature lower than zero
use std::io;
fn main() {
    const N:usize=31;
    let mut table:[f64;N]=[0.0;N];
    let mut i:usize;
    let average_temperature:f64;
    let mut user_input;
    let mut sum:f64=0.0;
    let mut lower_than_zero:f64;
    for i in 0..table.len(){
       println!("Type the temperature of the {} day: ",i+1);
       user_input=String::new();
       io::stdin()
          .read_line(&mut user_input)
          .expect("Failed to read line");
       let user_input: f64=user_input.trim().parse().expect("Type a number please");
       table[i]=user_input;
    }
   //calculate the average temperature of month January
   for i in 0..table.len(){
      sum=sum+table[i];
   }
   average_temperature=sum/(N as f64);
   println!("The average temperature of month January is {}",average_temperature);
   //find temperature lower than zero
   for i in 0..table.len(){
    if table[i]<0.0{
       println!("Temperature lower than zero: {}",table[i]);
       println!("Day:{}",i+1);
       println!("-------------------------------");
    }//end if
   }//end for loop
}//end function main
