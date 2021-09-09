use std::io;
fn main() {
    println!("Type a sentence please: ");
    let mut _user_input:String=String::new();
    io::stdin().read_line(&mut _user_input).expect("Failed to read user's input");
    for ch in _user_input.chars(){
     if ch=='a'{
        print!("A");
     }
     else if ch=='e'{
       print!("E");
     }
     else if ch=='i'{
       print!("I");
     }
     else if ch=='o'{
       print!("O");
     }
     else if ch=='u'{
       print!("U");
     }
     else{
       print!("{}",ch);
      }
    }
}
