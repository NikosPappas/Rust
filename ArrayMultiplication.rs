use std::io;
fn main() {
   const N:usize=5;
   const M:usize=5;//M and K must be equal for matrix multiplication
   const K:usize=5;
   const L:usize=5;
   let mut i:usize;
   let mut j:usize;
   let mut k:usize=0;
   let mut user_input;
   let mut table1:[[i32;M];N]=[[0;M];N];
   let mut table2:[[i32;K];L]=[[0;K];L];
   let mut table3:[[i32;N];L]=[[0;N];L];
   let mut sum:i32=0;

   for i in 0..N{
      for j in 0..M{
        println!("Type the [{}][{}] element of the array: ",i,j);
        user_input=String::new();
        io::stdin()
           .read_line(&mut user_input)
           .expect("Failed to read line");
       let user_input: i32 = user_input.trim().parse().expect("Type a number please");
       table1[i][j]=user_input;
      } //end for j loop
   }//end for i loop
   for i in 0..table1.len(){
      for k in 0..table2[i].len(){
         println!("Type the [{}][{}] element of the array: ",i,k);
         user_input=String::new();
         io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
         let user_input: i32=user_input.trim().parse().expect("Type a number please");
         table2[i][k]=user_input;
      }//end for k loop
   }//end for i loop
   for i in 0..table1.len(){
      for j in 0.. table2.len(){
         sum=0;
         for k in 0..table3.len(){
            sum=sum+table1[i][j]*table2[j][k];
      }//end for k loop
       table3[i][k]=sum;
     }//end for j loop
   }//end for i loop
}//end of main functions
