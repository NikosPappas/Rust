fn main() {
    let mut _i:u64;
    const E:f64=2.718f64;
    println!("________________________________________________________________________");
    println!("|-----------------------------------------------------------------------|");
    println!("|n\t\t(n!)^1/2n\t\t\tn/e\t\t\t|");
    println!("|-----------------------------------------------------------------------|");
    for _i in 1..25{
      println!("|{}\t\t{:.6e}\t\t\t{:.6e}\t\t|",_i,sqrt_factorial(_i).round()/1000.00f64,(_i as f64)/E);
      println!("|-----------------------------------------------------------------------|");
    }
    print!("|_______________________________________________________________________|");
    println!("");
}
 
fn sqrt_factorial(x:u64)->f64{
  return f64::powf(factorial(x) as f64,1.00f64/2.00*(x as f64));
}

fn factorial(x:u64)->f64{
  let mut _f:f64=1.00f64;
  let mut _i:u64=1;
  for _i in 1..x{
     _f=_f*(_i as f64);
  }
 return _f as f64;
}