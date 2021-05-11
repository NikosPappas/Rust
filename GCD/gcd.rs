use std::io;
fn main() {
    let mut _m;
    let mut _n;
    println!("Welcome to the gcd program.");
    println!("Type the first number please: ");
     _m=String::new();
    io::stdin().read_line(&mut _m).expect("Failed to read line");
    let mut _m: u64=_m.trim().parse().expect("Type a number please");
    println!("Type the second number please: ");
    _n=String::new();
    io::stdin().read_line(&mut _n).expect("Failed to read line");
    let mut _n: u64=_n.trim().parse().expect("Type a number please");
    let a=_m;
    let b=_n;
    println!("The greatest common divisor of {} and {} is {}",a,b,gcd(&mut _m,&mut _n));
}
fn gcd(m:&mut u64,n:&mut u64)->u64{
   assert!(*n!=0 && *m!=0);
   while *m!=0{
     if *m<*n{
       let t=*m;
       *m=*n;
       *n=t;
     }
    *m=*m%*n;
   } 
  return *n;
}
