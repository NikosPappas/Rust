use std::io;
fn main() {
    let _start:f64=1.00f64;
    let _end:f64=3.00f64;
    let _number_of_intervals=1000.00f64;
    println!("The area is {} ",trapezoidal_rule(_start,_end,_number_of_intervals));
}

fn y(x:f64)->f64{
  return f64::powf(x,2.00);
}
fn trapezoidal_rule(start:f64,end:f64,_number_of_intervals:f64)->f64{
  let mut _sum:f64=0.00f64;
  let mut _i:i32;
  _sum=y(start)+y(end);
  for i in 1..(end as i32)-1{
     _sum=_sum+2.00*y(i as f64);
  }
  return ((end-start)/(2.00*_number_of_intervals))*_sum;
}
