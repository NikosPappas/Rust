use std::env;
use std::process;
fn main() {
     let arguments:Vec<String> = env::args().collect();
     let number1:f64=arguments[1].parse::<f64>().unwrap();
     let number2:f64=arguments[3].parse::<f64>().unwrap();
     if arguments[2]=="+"{
       let sum=number1+number2;
       println!("{}+{}={}",number1,number2,sum);
     }
     else if arguments[2]=="-"{
       let dif=number1-number2;
       println!("{}-{}={}",number1,number2,dif);
     }
     else if arguments[2]=="x"{
      let mul=number1*number2;
      println!("{}*{}={}",number1,number2,mul);
     }
     else if arguments[2]=="/"{
      assert!(number2!=0.0);
      let div=number1/number2;
      println!("{}/{}={}",number1,number2,div);
     }
     else if arguments[2]=="^"{
       let pow=f64::powf(number1,number2);
       println!("{}^{}={}",number1,number2,pow);
     }
     else{
       println!("Error!Exiting program.");
       std::process::exit(0);
     }
     
}
