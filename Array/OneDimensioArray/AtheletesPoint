//The following program computes the mark of 
//an athlete in his/her sport in gymnastic
//juded by comitee of 10 jury
use std::io;
fn main() {
    const N:usize=10;
    let mut table: [f64;N]=[0.0;N];
    let mut user_input;
    let mut average: f64=0.0_f64;
    let mut sum: f64=0.0_f64;
    
    for i in 0..table.len(){
       println!("Type the mark of the {} judje: ",i+1);
       user_input=String::new();
       io::stdin()
          .read_line(&mut user_input)
          .expect("Failed to read line");
       let user_input: f64=user_input.trim().parse().expect("Type a number please");
       //check user input
       while user_input <=0.0 && user_input>=10.0{
           println!("You entered a wrong value!");
           println!("Type a value between 0 and 10 please:");
           let mut user_input=String::new();
           io::stdin()
               .read_line(&mut user_input)
               .expect("Failed to read line");
           let user_input: f64 = user_input.trim().parse().expect("Type a number please");
       }//end while
       table[i]=user_input;
    }//end for i loop

    for i in 0..table.len(){
       sum=sum+table[i];
    }//end of for loop

    average=(sum/(N as f64)) as f64;
    println!("The Average mark for the athlete is {} ",average);
    
}//end of function main
