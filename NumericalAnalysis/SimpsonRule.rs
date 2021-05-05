fn  main() {
  let result:f64=simpson(1.00f64,2.00f64,1000.0f64);
  println!("The area is {} ",result);
}
fn y(x:f64)->f64{
  return f64::powf(x,2.00);
}
fn simpson(start:f64,end:f64,_number_of_intervals:f64)->f64{
  let mut _sum_even:f64=0.0f64;
  let mut _sum_odd:f64=0.0f64;
  let mut _sum:f64=0.0f64;
  let mut _i:i32=1;
  _sum=y(start)+y(end);
  while _i<(end as i32){
      if _i%2==1{
        _sum_odd=_sum_odd+4.00*y(_i as f64);
      }
      else{
        _sum_even=_sum_even+2.00*y(_i as f64);
      }
   _i=_i+1;
  }
  _sum=_sum_even+_sum_odd;
  return ((end-start)/(_number_of_intervals*3.00f64))*_sum;  
}
