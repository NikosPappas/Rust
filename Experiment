//The following programm stores the measurment of 
//an experiment that measures a value 200 times.
//It calculates the average value of the measurement
//and the standard deviation
use std::io;
fn main() {
    const N:usize=200;
    let mut table:[f64;N]=[0.0;N];//initialize table
    let mut i:usize;
    let average:f64;
    let mut user_input;
    let mut sum:f64=0.0; 
    let standard_deviation:f64;
    for i in 0..table.len(){
       println!("Type the value of the {} measurement: ",i+1);
       user_input=String::new();
       io::stdin()
          .read_line(&mut user_input)
          .expect("Failed to read line");
       let user_input: f64=user_input.trim().parse().expect("Type a number please");
       table[i]=user_input;   
    }
    for i in 0..table.len(){
      sum=sum+table[i];
    }
    average=sum/(N as f64);
    println!("The average value of the measurement is {}",average);
    for i in 0..table.len(){
     sum=sum+f64::powf(table[i]-average,2.00);
    } 
    standard_deviation=1.00/((N as f64)-1.00)*sum;
    println!("The standard deviation is {} ",standard_deviation.sqrt());
    
}
