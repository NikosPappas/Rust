use std::io;
fn main() {
    const N:usize=20;
    let mut table:[i32;N]=[30,12,54,6,9,3,2,0,4,7,9,56,23,12,47,81,31,68,98,89];
    quick_sort(& mut table,0,N-1);
    //println!("{:?}",table);
}
fn quick_sort(a: &mut [i32],start:usize,end:usize){
     if start<=end{
       let mut pi:usize=partition(a,end,start);
       quick_sort(a,start,pi-1);
       quick_sort(a,pi+1,end);
     }
}
fn partition(a: &mut [i32],start:usize,end:usize) -> usize{
      let mut pivot= a[end];
      let mut j=start;
      let mut i=start;
      while j<=end{
        if a[j] < pivot{
           i=i+1;
          let temp:i32=a[i];
          a[i]=a[j];
          a[j]=temp;
        }
         j=j+1;
      }
     if a[end] < a[i] { 
       let temp=a[i+1];
           a[i+1]=a[j];
           a[j]=temp;
     }
     return (i+1) as usize;
}
