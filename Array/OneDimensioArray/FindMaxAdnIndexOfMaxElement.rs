use std::io;
fn main() {
    const N:usize=30;
    let mut i:usize;
    let mut table:[i32;N]=[0;N];
    let mut max:i32;
    let mut index:usize;
    let mut user_input;
    for i in 0..table.len(){
         println!("Type the {} element of the array: ",i+1);
         user_input=String::new();
         io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let user_input: i32=user_input.trim().parse().expect("Type a number please");
        table[i]=user_input;
    }//end for loop
   max=table[0];
   index=0;
   for i in 1..table.len(){
       if table[i]>max{
          max=table[i];
          index=i; 
       }
   }
  println!("Maximum element: {}",max);
  println!("Index of maximum element:{}",index);
}
