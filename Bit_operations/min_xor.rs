use std::io;
fn main() {
  const N:usize=10;
  let mut _array:[i32;N]=[0;N];
  let mut _i:usize=0;
  for _i in 0..N{
       println!("Type a number please: ");
       let mut _user_input:String = String::new();
       io::stdin().read_line(&mut _user_input).expect("Failed to read user's input");
       let _user_input: i32 = _user_input.trim().parse().expect("Type a number please");
       _array[_i]=_user_input;
   }//end of data input
   let mut _ret_val=min_xor(&mut _array);
   println!("Min xor: {}",_ret_val);
 }

fn min_xor(_array:&mut [i32])->i32{
   let mut _min_xor:i32=i32::MAX;
   let mut _j:usize=0; 
   for _i in 0.._array.len(){
      for _j in _i+1.._array.len(){
       _min_xor=std::cmp::min(_min_xor,_array[_i]^_array[_j]);
      }//end for _j loop
  }
   _min_xor
}//end of function main
