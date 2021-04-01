use std::num::sqrt;
use std::io;
fn main() {
   let height: u64 = read_hight();
   let result: u64 = calculate_velocity(height);
   print_result(result);

}

fn read_hight() -> u64 {
    println!("Enter the hight: ");
    let mut height = String::new();
    io::stdin()
       .read_line(&mut height)
       .expect("Failed to read line");
    let height: u64=height.trim().parse().expect("Please type a number!");
    return height;
}

fn calculate_velocity(hight: u64) -> u64{
    let mut result: u64=sqrt(2.00*9.81*hight);
    return result;
}

fn print_result(a: u64 ){
    println!("The velocity is {}",a);
}
