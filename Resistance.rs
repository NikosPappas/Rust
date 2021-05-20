use std::io;

fn main() {
    println!("Type the value of the first resistance: ");
    let mut r1 = String::new();
    io::stdin()
        .read_line(&mut r1)
        .expect("Failed to read line");
    println!("Type the value of the second resistance: ");
    let mut r2 = String::new();
    io::stdin()
       .read_line(&mut r2)
       .expect("Failed to read line");
    println!("Type 1 for serial connection and 2 for parallel connection:");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line: ");
    let r1:i32 = r1.trim().parse().expect("Please type a number!");
    let r2:i32 = r2.trim().parse().expect("Please type a number!");
    let choice: i32 = choice.trim().parse().expect("Please type a number!");
    let r:i32;
    if choice == 1{
       r=r1+r2;
       println!("Total resistace is: {}",r);
    }
    else{
        r=(r1*r2)/(r1+r2);
        println!("Total resicstance is {}",r);
    }
}
