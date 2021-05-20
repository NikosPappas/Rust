fn main() {
    let mut sum: i32=0;
    let mut number: i32=1;
    loop{
       if is_prime(number)==true{
           sum=sum+number;
       }//end if
       number=number+1;
       if number <= 1000{
           break;
       }//end if
    }//end loop
    println!("The sum of prime number smaller than 1000 is: {}",sum);
}
fn is_prime(number: i32)->bool{
   let mut is_prime_number: bool=true;
   let mut i: i32=2;
   
   while i<number{
    if number%i==0{
        is_prime_number=false;
        break;
    }
    i=i+1;
   }//end of while loop
  return is_prime_number;
}//end of isPrime function 
