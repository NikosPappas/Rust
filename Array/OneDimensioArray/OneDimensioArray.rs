
use std::io;
fn main(){
    println!("We will find the sum of the array elements");
    const N:usize=10;
    let mut i:usize=0;
    let mut arr:[i32;N]=[0;N];
    let mut sum:i32=0;
    let mut value;

    while i < N{
        println!("Enter the {} element of the array: ",i+1);
        value=String::new();
        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line.");
        let value:i32=value.trim().parse().expect("Type a number please");
        arr[i]=value;
        sum=sum+arr[i];
        i=i+1;
    }//end of while loop
    println!("Array is {:?}",arr);
    println!("The sum is {}",sum);
}
