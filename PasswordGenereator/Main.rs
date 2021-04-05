extern crate rand;
use std::io;
use rand::Rng;
fn main() {
   const N:usize=71;
   let chars:[char;N]=['a','b','c','d','e','f','g','h','i','j','k','l','m',
                       'n','o','p','q','r','s','t','u','v','w','x','y','z',
                       'A','B','C','D','E','F','G','H','I','J','K','L','M',
                       'N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
                       '1','2','3','4','5','6','7','8','9','0','!','@','#',
                       '$','%','^','&','*','_'];
   let mut i:usize;
   let mut j:usize;
   let mut random_index:usize;
   let mut _user_input;
   
   println!("Type the length of the password: ");
   _user_input=String::new();
   io::stdin()
      .read_line(&mut _user_input)
      .expect("Failed to read line");
   let _user_input: usize = _user_input.trim().parse().expect("Type a number please:");
   
   for i in 0..1000{
      for j in 0.._user_input{
        random_index=rand::thread_rng().gen_range(0,71);
        print!("{}",chars[random_index]);
      }
     println!();
   }
   println!("Done generating passwords of length {} ",_user_input);
}
