use std::thread;
use std::time::Duration;
use std::io;
fn main() {
 let handle=thread::spawn(||{
        let mut _sum:i32=0;
        let mut _a;
        let mut _b;
        println!("Type the first number please: ");
        _a=String::new();
        io::stdin().read_line(&mut _a).expect("Failed to read line");
        let _a:i32=_a.trim().parse().expect("Type a number please");
        println!("Type the second number please: ");
        _b=String::new();
        io::stdin().read_line(&mut _b).expect("Failed to read line");
        let _b:i32=_b.trim().parse().expect("Type a number please");
        _sum=_a+_b;
        println!("The sum is {} ",_sum);
        thread::sleep(Duration::from_millis(1));
     });
    handle.join().unwrap(); 
   println!("Done");  
}
