use std::thread;
use std::time::Duration;
use std::io;
fn main() {
    let handle=thread::spawn(||{
       const N:usize=10;
       let mut _i:usize;
       let mut _array:[i32;N]=[0;N];
       let mut _user_input:String;
       for _i in 0..N{
          println!("Type a number please: ");
          _user_input=String::new();
          io::stdin().read_line(&mut _user_input).expect("Failed to read user's input");
          let _user_input:i32=_user_input.trim().parse().expect("Type a number please");
           _array[_i]=_user_input;
         
       }//end for loop
       println!("Found an array {:?}",_array);
       thread::sleep(Duration::from_millis(2));
       let mut _sum:i32=0;
       for _i in 0..N{
            _sum+=_array[_i];
       }
       println!("The sum is {}",_sum);
    });
    handle.join().unwrap();
}
