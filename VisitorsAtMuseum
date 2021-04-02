//The following program saves the visitors 
//of a museum for one day.Each visitor visit one or more
//of museum's hall.The program computes the
//hall with the maximum visitors
use std::io;
fn main() {
   const N:usize=10;
   let mut visitors:[i32;N]=[0;N];
   let mut i:usize;
   let mut number_of_visitors;
   let mut max_visitors:i32;
   let mut total_visitors:i32;
   let mut index:i32=0;
   for i in 0..visitors.len(){
       println!("Type the number of visitors for hall number {}",i+1);
       number_of_visitors=String::new();
       io::stdin()
          .read_line(&mut number_of_visitors)
          .expect("Failed to read line");
       //shadowing
       let number_of_visitors: i32 = number_of_visitors.trim().parse().expect("Type a number please");
       visitors[i]=number_of_visitors;
   }//end for loop
   max_visitors=visitors[0];
   for i in 1..visitors.len(){
       if visitors[i]>max_visitors{
        max_visitors=visitors[i];//find maximum visitors
        index=i as i32;//save the hall
      }
   }
   println!("The hall with the maximum visitors is hall number{} ",index+1);
   total_visitors=0;
   for i in 0..visitors.len(){
      total_visitors=total_visitors+visitors[i];
   }
   println!("The total visitors for the day are {} visitors",total_visitors);

}
