use std::io;
fn main() {
   const N:usize=20;
   let mut arr:[i32;N]=[0;N];
   let mut i:usize=0;
   let mut sum:i32=0;
   let mut input;

   while i<N{
       println!("Type the {} element of the array: ",i+1);
       input=String::new();
       io::stdin()
           .read_line(&mut input)
           .expect("Failed to read line.");
       let input:i32=input.trim().parse().expect("Type a number please");
       arr[i]=input;
       i=i+1;
   }
   i=0;
   while i<N{
       if arr[i]>5 && arr[i]<40{
           sum=sum+arr[i];
           println!("{} is greater than 5 and lower than 40",arr[i]);
       }
       i=i+1;
   }
   println!("The sum of numbers greater than 5 and lower than 40 is {}",sum);
}
