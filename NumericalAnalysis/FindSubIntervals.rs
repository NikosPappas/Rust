use std::io;
fn main(){
    let mut _a:String;
    let mut _b:String;
    let mut _error:String;
    let mut _m:String;
    println!("Type the starting point of the interval: ");
    _a=String::new();
    io::stdin().read_line(&mut _a).expect("Failed to read starting point line");
    let _a: f64=_a.trim().parse().expect("Type a number that corresponde to the starting point");
    println!("Type the ending point of the interval please:");
    _b=String::new();
    io::stdin().read_line(&mut _b).expect("Failed to read ending point of the interval");
    let _b: f64=_b.trim().parse().expect("Type a number that corresponde to the ending point");
    println!("Type the error value please: ");
    _error=String::new();
    io::stdin().read_line(&mut _error).expect("Failed to read error line");
    let _error: f64=_error.trim().parse().expect("Type a number that correspondes to the error");
   
    println!("Type the M please:");
    _m=String::new();
    io::stdin().read_line(&mut _m).expect("Failed to read M line");
    let _m: f64=_m.trim().parse().expect("Type a number please that correspondes to M:");
    let mut _trapezoidal=trapezoidal_rule_interval(_a,_b,_error,_m);
    println!("Intervals for trapezoidal rule: {}",_trapezoidal);
 
    println!("Type the M for simpson rule:");
    let mut _m=String::new();
    io::stdin().read_line(&mut _m).expect("Failed to read _m line");
    let _m: f64=_m.trim().parse().expect("Type a number for M please");
    let _simpson=simpsons_rule_interval(_a,_b,_error,_m);    
    println!("Intervals for simpson's rule: {}",_simpson);

}//end of main function


fn trapezoidal_rule_interval(_a:f64,_b:f64,_error:f64,_m:f64)->f64{
  let _nominator:f64=(_m*f64::powf(_b-_a,3.00))/12.00f64;
  let n:f64=f64::sqrt(_nominator*(1.00/_error));
  return n.round();
}//end of function;

fn simpsons_rule_interval(_a:f64,_b:f64,_error:f64,_m:f64)->f64{
  let _nominator:f64=(_m*f64::powf(_b-_a,5.00))/180.00;
  let _n:f64=f64::powf(_nominator*(1.00/_error),1.00/5.00);
  if _n.round()%2.00==0.00{
    return _n;
  }//end of if close
  else{
    return _n.round()+1.00;
  }//end of else close
}//end of function
