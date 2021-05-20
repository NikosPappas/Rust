use std::io;
struct Complex{
  real: f64,
  imaginary: f64,
}
impl Complex{
  
   fn init_complex(&mut self,a:f64,b:f64){
     (*self).real=a;
     (*self).imaginary=b;
   }

 fn add_complex(z1: &mut Complex,z2: &mut Complex)->Complex{
      let mut z=Complex{real:0.0,imaginary:0.0};
      z.real=(*z1).real+(*z2).real;
      z.imaginary=(*z1).imaginary+(*z2).imaginary;
      return z;
 }

 fn sub_complex(z1: &mut Complex,z2: &mut Complex) -> Complex{
     let mut z=Complex{real:0.0,imaginary:0.0};
     z.real=(*z1).real-(*z2).real;
     z.imaginary=(*z1).imaginary-(*z2).imaginary;
      return z;
 }

 fn complex_multiplication(z1: &mut Complex,z2: &mut Complex)->Complex{
      let mut z=Complex{
                       real:(*z1).real*(*z2).real-(*z1).imaginary*(*z2).imaginary,
                      imaginary:(*z2).imaginary*(*z2).real+(*z1).real*(*z2).imaginary,
                    };
      return z;
 }
 fn complex_division(z1: &mut Complex,z2: &mut Complex)->Complex{
  let mut z=Complex{
                     real:((*z1).real+(*z2).real)/(f64::powf((*z2).real,2.00)+f64::powf((*z2).real,2.00)),
                     imaginary:((*z1).imaginary*((*z2).real))/(f64::powf((*z2).real,2.00)+f64::powf((*z2).imaginary,2.00)),
                   };
  return z; 
 } 
 fn print_complex(&self){
  if self.imaginary<0.0f64{
    print!("{}-{}i",(*self).real,(*self).imaginary);
  }
  else{
   print!("{}+{}i",(*self).real,(*self).imaginary);
  }
 }//end of functions implementation
}
fn main() {
   let mut z1=Complex{
     real: 1.2f64,
     imaginary: 3.2f64,
   };
   let mut z2=Complex{
     real:1.2f64,
     imaginary:1.1f64,
     };
  let mut sum=Complex{real:0.0f64,imaginary:0.0f64,};
  sum=Complex::add_complex(&mut z1,&mut z2);
  print!("The sum of");
  z1.print_complex();
  print!(" and ");
  z2.print_complex();
  print!(" is ");
  sum.print_complex();
  let mut dif=Complex::sub_complex(&mut z1,&mut z2);
  println!();
  print!("The difference of ");
  z1.print_complex();
  print!(" and ");
  z2.print_complex();
  print!(" is ");
  dif.print_complex();
  println!();
  let mut mul=Complex{real:0.0f64,imaginary:0.0f64};
  mul=Complex::complex_multiplication(&mut z1,&mut z2);
  print!("The product of ");
  z1.print_complex();
  print!(" and ");
  z2.print_complex();
  print!(" is ");
  mul.print_complex();
  println!();
  let mut div=Complex{real:0.0f64,imaginary:0.0f64};
  div=Complex::complex_division(&mut z1,&mut z2); 
  print!("The division of ");
  z1.print_complex();
  print!(" and ");
  z2.print_complex();
  print!(" is ");
  div.print_complex();
  println!();
}//end of function main
