use std::io;
fn main() {
   const N:usize=15;//number of floors
   const M:usize=30;//number of rooms in each floor
   let mut hotel:[[u32;N];M]=[[0;N];M];
   let i:usize;
   let j:usize;
   let mut user_input;
   let mut free_rooms_per_floor:[u32;M]=[0;M];
   let mut total_free_rooms:u32=0;
  
   for i in 0..N{
      for j in 0..M{
         println!("Type 1 if room {} in floor {} has customer or 0 if it is empty: ",i,j);
         user_input=String::new();
         io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
         let user_input: u32=user_input.trim().parse().expect("Type 0 if room is free or 1 else");
         hotel[i][j]=user_input;
      }
   }
   //find free rooms per floor
   for i in 0..N{
      for j in 0..M{
          if hotel[i][j]==0{
             free_rooms_per_floor[i]=free_rooms_per_floor[i]+1;
          }
      }
     println!("Total free rooms in floor {} :{}",i,free_rooms_per_floor[i]);
   }
   //print the total free rooms
  for i in 0..N{
       total_free_rooms=total_free_rooms+free_rooms_per_floor[i];
  }
  println!("Total free rooms: {}",total_free_rooms);
 //find   
}
