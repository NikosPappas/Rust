use std::io;
fn main() {
    let mut sum: i32 =0;
    let mut average: f64=0.0;
    let mut plithos: i32=0;
    loop{
        println!("Type an integer please (type zero to exit):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: i32 = input.trim().parse().expect("Type an integer please:");
        if input == 0 {
            break;
        }
        else{
            plithos = plithos +1;
            sum = sum + input;
        }

    }
    average =(sum as f64)/(plithos as f64);
    println!("The sum is {}\nThe average is {}\nThe total number is {}",sum,average,plithos);
}
