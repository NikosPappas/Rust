se std::io;
fn main() {
    const N:usize=6;
    const M:usize=7;
    let A:[i32;N]=[1,3,5,7,9,11];
    let B:[i32;M]=[2,4,6,8,10,12,14];
    const size:usize=M+N;
    let mut C:[i32;size]=[0;size];
    let mut _i:usize;
    let mut _j:usize;
    let mut _k:usize;
    let mut _n:usize=0;
    _i=0;
    _j=0;
    _k=0;
    while _i<A.len() && _j<B.len(){
      if A[_i] < B[_j]{
         C[_k]=A[_i];
         _i=_i+1;
         _k=_k+1;
      }//end if
      else{
       C[_k]=B[_j];
         _k=_k+1;
         _j=_j+1;
      }//end else
    if _i>=A.len() {
           _n=_k;
      while _n < size{
         C[_n]=B[_j];
         _j+=1;
        _n=_n+1;
      }//end while
      
    }//end if
    else{
       _n=_k;
      while _n < size{
         C[_n]=A[_i];
        _i=_i+1;
        _n=_n+1;
     }//end for
    }//end else
  }//end of while loop
  
  for _i in 0..size{
    println!("{} ",C[_i]);
  }//end for 

}//end of main
