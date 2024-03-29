use std::io;
fn main() {
  const N:usize=50;
  let mut table:[i32;N]=[0;N];
  pritnln!("{} "fibonacci(&mut table);
}

fn fibonacci(table: &mut [i32])-> i32{
  let mut i:usize;
  table[0]=0;
  table[1]=1;
  for i in 2..table.len(){
      table[i]=table[i-2]+table[i-1];
  }
  return table[table.len()-1];
}
fn fibonacci_two(n:usize)->i32{
 let mut a:i32=0;
 let mut b:i32=1;
 let mut c:i32=0;
 let mut i:usize;
 for i in 2..n{
   c=a+b;
   a=b;
   b=c;
 }
 return c;
}
