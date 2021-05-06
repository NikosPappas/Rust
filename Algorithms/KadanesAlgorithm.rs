use std::io;

fn main() {
   const N:usize=20;
   let mut _table:[i32;N]=[0;N];
   let mut _user_input;
   let mut _i:usize;
   for _i in 0..N{
      println!("Type the {} element of the array: ",_i);
      _user_input=String::new();
     io::stdin().read_line(&mut _user_input).expect("Failed to read line");
     let _user_input: i32=_user_input.trim().parse().expect("Type an integer please");
     _table[_i]=_user_input;
   }
   let result=kadenes_algorithm(&mut _table,0,N);
   println!("Maximum subarray sum is {}",result);
}
fn kadenes_algorithm(_a:&mut [i32],_start:usize,_end:usize)->i32{
  let mut _i:usize;
  let mut _max_so_far:i32=i32::MIN;
  let mut _max_ending_here:i32=0;
  for _i in _start.._end{
    _max_ending_here=_max_ending_here+_a[_i];
   if _max_so_far < _max_ending_here{
      _max_so_far = _max_ending_here;
   }
   if _max_ending_here < 0{
      _max_ending_here=0;
   }
  }
  return _max_so_far;
}
