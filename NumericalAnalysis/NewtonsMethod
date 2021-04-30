fn main() {
    let mut x0:f64=1.00f64;
    netwons_method(&mut x0);
}

fn y(x: f64)->f64{
  return f64::powf(x,2.00)-3.00;
}
fn derivative(x:f64)->f64{
  return 2.00*x;
}
fn netwons_method(x0:&mut f64){
  let mut i:u64=0;
  let mut xi:f64;
  while i<10{
    xi=*x0-y(*x0)/derivative(*x0); 
    println!("{}\t\t{:.5e}\t\t{:.5e}\t\t{:.5e}\t\t{:.5e}",i,*x0,y(*x0),derivative(*x0),xi);
    *x0=xi;
    i=i+1;
  }  
}
