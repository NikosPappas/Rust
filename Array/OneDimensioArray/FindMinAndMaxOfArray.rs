use std::io;
fn main() {
    const N:usize=30;
    let mut table:[i32;N]=[0;N];
    let i:usize;
    let mut user_input:String;
    let mut max:i32;
    let mut min:i32;
    for i in 0..table.len(){
        println!("Type the {} element of the array",i+1);
        user_input=String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let user_input: i32=user_input.trim().parse().expect("Type a number please");
        table[i]=user_input;
    }
    for i in 0..table.len(){
       println!("The {} element of the array is {}",i+1,table[i]);
    }
    max=table[0];
    min=table[0];
    for i in 0..table.len(){
       if table[i] > max {
          max=table[i];
       }
       if table[i]<min {
          min=table[i]
       }
    }
    println!("The maximum element of the array is {} and the minimum element of the array is {}",max,min);
}
