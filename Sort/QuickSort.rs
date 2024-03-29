fn main() {
       const N:usize=10;
       let mut table:[i32;N]=[2,0,3,1,4,6,5,9,8,7];
       let last_index=table.len()-1;
       println!("{:?}",table);
       quick_sort(&mut table,0,last_index);
       println!("{:?}",table);   
}

fn quick_sort(a:&mut [i32;10],start:usize,end:usize){
    if start < end{
     let pi=partition(a,start,end);
     quick_sort(a,start,(pi-1) as usize);
     quick_sort(a,(pi+1) as usize,end);
    }
}
fn partition(a:&mut [i32],start:usize,end:usize)->i32{
  let mut pivot=a[end];
  let mut _i=start;
  let mut _j=start;
  while _j<end{
       if a[_j]<pivot{
          swap(a,_i,_j);
         _i+=1;
       }
      _j+=1;
  }
  _i+=1;
  swap(a,_i,_j);
  return _i as i32;
}

fn swap(_a:&mut [i32],_i:usize,_j:usize){
  let temp=_a[_i];
      _a[_i]=_a[_j];
      _a[_j]=temp;
}

