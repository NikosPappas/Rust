use std::io;
#[derive(PartialEq)]
enum States{
  StartState,
  State12,
  State13,
  State14,
  State15,
  State16,
  State17,
  State18,
  State19,
  ErrorState,
  FinalState,
}
fn main() {
    println!("Type a number please: ");
    let mut _user_input:String=String::new();
    io::stdin().read_line(&mut _user_input).expect("Failed to read line");
    let mut _start:usize=0;
    let mut _end:usize=0;
    let _chars:Vec<char>=_user_input.chars().collect();
    let mut _st:States=States::StartState;
    _end=_chars.len()-1; 
    while _start<=_end && _st!=States::FinalState && _st!=States::ErrorState  {
         if _st==States::StartState{
            _st=_state_12(_chars[_start]);
         }
         else if _st==States::State12{
            _st=_state_12(_chars[_start]);
         }
         else if _st==States::State13{
            _st=_state_13(_chars[_start]);
         }
         else if _st==States::State14{
            _st=_state_14(_chars[_start])
         }
         else if _st==States::State15{
            _st=_state_15(_chars[_start])
         }
         else if _st==States::State16{
            _st=_state_16(_chars[_start])
         }
         else if _st==States::State17{
            _st=_state_17(_chars[_start]);
         }
         else if _st==States::State18{
            _st=_state_18(_chars[_start]);
         }
         else if _st==States::State19{
            _st=_state_19(_chars[_start]);
         }
        _start+=1;
    }
}

fn _state_12(c:char)->States{
   let _st:States;
   if c>='0' && c<='9'{
      _st=States::State13;
   }//end if 
   else{
    _st=States::ErrorState; 
    println!("Error");
   }
   _st
}
fn _state_13(c:char)->States{
   let _st:States;
   if c>='0' && c<='9'{
     _st=States::State13;
   }//end if
   else if c=='.'{
      _st=States::State14;
   }
   else if c==' '{
      _st=States::State16;
   }
   else{
      _st=States::FinalState;
       println!("Final state");
   }
  _st
}
fn _state_14(c:char)->States{
  let _st:States;
  if c>='0' && c<='9'{
   _st=States::State15;
  }
  else{
   _st=States::ErrorState;
   println!("Error");
  }
  _st
}
fn _state_15(c:char)->States{
  let _st:States;
  if c>='0' && c<='9'{
   _st=States::State15;
  }
  else if c=='E'{
    _st=States::State16;
  }
  else{
   _st=States::FinalState;
   println!("Final state");
  }
  _st
}
fn _state_16(c:char)->States{
 let _st:States;
   if c=='-' || c=='+'{
      _st=States::State17;
   }
   else if c>='0' && c<='9'{
     _st=States::State18;
   }
   else{
     _st=States::ErrorState;
     println!("Error");
   }  
 _st
}
fn _state_17(c:char)->States{
  let _st:States;
   if c>='0' && c<='9'{
      _st=States::State18;
   }
   else{
       _st=States::ErrorState; 
      println!("Error");
     }
  _st
}
fn _state_18(c:char)->States{
   let _st:States;
   if c>='0' && c<='9'{
       _st=States::State18;
   }
   else{ 
       _st=States::State19;
   }
   _st
}
fn _state_19(_c:char)->States{
  let _st:States;
  _st=States::FinalState;
 println!("Final state");
  _st
}



