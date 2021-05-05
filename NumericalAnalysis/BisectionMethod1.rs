use std::io;

fn main() {
  let mut _a;
  let mut _b;
  println!("Type the starting point of the inteval: ");
  _a=String::new();
  io::stdin().read_line(&mut _a).expect("Failed to read line");
  let mut  _a: f64=_a.trim().parse().expect("Type a number please");
  println!("Type the ending point of the interval: "); 
  _b=String::new();
  io::stdin().read_line(&mut _b).expect("Failed to read line");
  let mut _b: f64=_b.trim().parse().expect("Failed to read line");
  println!("Trying to find the root");
  bisection_root(&mut _a,&mut _b);
 
}

fn y(x:f64)->f64{
  return f64::powf(x,2.00)-3.00;
}
fn bisection_root(a: &mut f64,b:&mut f64){
   let mut middle:f64=0.0f64;
   let mut counter:u32=0;  
  if y(*a)*y(*b)>0.00{
    println!("You have not guessed right");
   }
  else{
    println!("iterations\ta\t\tb\t\tmiddle\t\tf(a)\t\tf(b)\t\tf(middle)");
     while *b-*a>=0.00001{
         counter+=1;
         middle=(*b+*a)/2.00;
         if y(middle)==0.00{
            break;
         }
         else if y(middle)*y(*a)<0.00{
            *b=middle;
         }
         else{
            *a=middle;
         }
         println!("{}\t\t{:.2e}\t\t{:.2e}\t\t{:.2e}\t\t{:.2e}\t{:.2e}\t\t{:.2e}",counter,*a,*b,middle,y(*a),y(*b),y(middle));
     }
     println!("The value of the root is  {}",middle);
  }
}
