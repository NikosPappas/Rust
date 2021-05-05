use std::io;
fn main() {
    let mut _a;
    let mut _b;
    println!("Type the starting point of the interval: ");
    _a=String::new();
    io::stdin().read_line(&mut _a).expect("Failed to read starting point");
    let mut  _a:f64=_a.trim().parse().expect("Type a number please");
    println!("Type the ending point of the interval: ");
    _b=String::new();
    io::stdin().read_line(&mut _b).expect("Failed to read ending point");
    let mut  _b:f64=_b.trim().parse().expect("Type a number please");
    bisection_root(&mut _a,&mut _b);
    
}
fn y(x:f64)->f64{
 return f64::exp(x)+f64::sin(x);
}
fn bisection_root(_a:&mut f64,_b:&mut f64){
  let mut _middle:f64=0.0f64;
  let mut _counter:u32=0;
  if y(*_b)*y(*_a)>0.00{
    println!("Your guessed the wrong interval.");
    return ;
  }
  else{
    println!("iterations\t\ta\t\tb\t\tmiddle\t\tf(a)\t\tf(b)\t\tf(middle)");
    while *_b-*_a>=0.00001{
       _counter=_counter+1;
       _middle=(*_b+*_a)/2.00;
       if y(_middle)==0.00{
            break;
       }
       else if y(_middle)*y(*_a)<0.00{
           *_b=_middle;
       }
      else{
         *_a=_middle;
      }
      println!("{}\t\t{:.2e}\t\t{:.2e}\t\t{:.2e}\t\t{:.2e}\t\t{:.2e}\t\t{:.2e}",_counter,*_a,*_b,_middle,y(*_a),y(*_b),y(_middle));
   }
  }
  println!("The value of the root  is {}",_middle);
}
