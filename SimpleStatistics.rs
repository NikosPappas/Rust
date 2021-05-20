use std::io;

fn main() {
    println!("How often do you use the subway?");
    println!("Type 1 if you use daily,2 sometimes and 3 not at all:");
    let mut one: i32=0;
    let mut two: i32=0;
    let mut three: i32=0;
  //let mut odd: i32=0;
  //let mut even: i32=0;
    let mut i: i32=0;
    loop{//start of loop
    let mut usage=String::new();
    io::stdin()
        .read_line(&mut usage)
        .expect("Failed to read line:");
    let usage: i32=usage.trim().parse().expect("Type a number please");
    if usage==1{
        one=one+1;
    }
    else if usage==2{
        two=two+1;
    }
    else{
        three=three+1;
    }

    i=i+1;
    if i==49{
        break;
    }
   
   }//end of loop
   println!("Results:");
   println!("Daily:{}\nSometimes:{}\nNo usage:{}\n",one,two,three);
   let mut even: i32 = 0;
   let mut odd: i32=0;
   let mut zero: i32=0;
   i=0;
   loop{
       println!("Please enter an integer,type 100 to exit:");
       let mut user_input=String::new();
       io::stdin()
           .read_line(&mut user_input)
           .expect("Failed to read line");
       let user_input: i32=user_input.trim().parse().expect("Type a number please");
       if user_input%2==0{
           even=even+1;
       }
       if user_input%2==1{
           odd=odd+1;
       }
       if user_input==0{
           zero=zero+1;
       }
       i=i+1;
       if i==51{
           break;
       }
   }//end of loop
   let average: f64 = ((even as f64)+(odd as f64))/(i as f64);
   println!("Average:{}\nEven:{}\nOdd:{}\nZero:{}\n",average,even,odd,zero);

}//end of main program 
