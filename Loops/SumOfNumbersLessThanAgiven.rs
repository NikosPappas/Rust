
use std::io;
fn main() {
    let mut sum: i32=0;
    loop {
        println!("Type a positive integer or a negative integer to exit");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: i32 = input.trim().parse().expect("Type an integer please");
        if input < 0{
           break;
        }
        sum=sum+input;
        if sum > 1000{
            break;
        }
    }
    println!("The sum is {}",sum);
}
