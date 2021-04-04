use std::io;
fn main() {
       const N:usize=10;
       let mut table:[i32;N]=[0;N];
       let mut user_input;
       let mut i:usize;
       let mut j:usize; 
       let mut index:usize;
       let mut temp:i32=0;
       for i in 0..table.len(){
            println!("Type the {} element of the array: ",i+1);
            user_input=String::new();
            io::stdin()
               .read_line(&mut user_input)
               .expect("Failed to read line"); 
            let user_input: i32=user_input.trim().parse().expect("Type a number please"); 
            table[i]=user_input;
       }
       //selection sort
       for i in 0..table.len(){
           index=i;
           for j in i+1..table.len(){
                 if table[j] < table[index]{
                    index=j;
                 }
           }
           temp=table[i];
           table[i]=table[index];
           table[index]=temp;
       }
     println!("{:?}",table);
}
