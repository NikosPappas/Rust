//insertion sort inmplementation in rust
use std::io;
fn main() {
    const N:usize=10; 
    let mut table:[i32;N]=[0;N];
    let mut i:usize;
    let mut j:usize;
    let mut temp:i32;
    let mut user_input;
    let mut key:i32;
    for i in 0..table.len(){
        println!("Type the {} element of the  array please: ",i+1);
        user_input=String::new();
        io::stdin()
           .read_line(&mut user_input)
           .expect("Failed to read line");
        let user_input: i32=user_input.trim().parse().expect("Type a number please");
        table[i]=user_input;
    }
    for i in 1..table.len(){
        j=i;
        while j>0 && table[j]<table[j-1]{
               temp=table[j];
               table[j]=table[j-1];
               table[j-1]=temp;
               j=j-1;
        }  
    }
    println!("{:?}",table);
}
