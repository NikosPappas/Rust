use std::io;
fn main() {
    const N:usize=15;
    let table:[i32;N]=[1,2,3,5,6,7,8,9,12,13,19,20,50,53,60];
    let mut _user_input;
    println!("Type the number you are searching for: ");
    _user_input=String::new();
    io::stdin()
       .read_line(&mut _user_input)
       .expect("Failed to read line");
    let _user_input: i32 = _user_input.trim().parse().expect("Type a number please");
    binary_search(table,0,N,_user_input);
      
}
fn binary_search(a: [i32;15],start: usize,end: usize,value: i32){
   if start<end{
      let middle:usize=start+(end-start)/2;
      if a[middle]==value{
         println!("The value you are searching for is at index {}",middle+1);
      }
      else if value < a[middle]{
        return  binary_search(a,start,middle-1,value);
      }
      else{
        return  binary_search(a,middle+1,end,value);
      }
     
   }
 
}
