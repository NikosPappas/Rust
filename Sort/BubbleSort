use std::io;
fn main(){
    const N:usize=15;
    let mut table:[i32;N]=[5,3,1,27,15,4,9,8,7,12,32,10,5,4,2];
    let mut _j:usize;
    let mut _i:usize;
    let mut temp:i32;
    //bubble sort asceding order
     _i=0;
     while _i<table.len(){
        _j=table.len()-1;
        while _j>_i{
            if table[_j-1]>table[_j]{
                temp=table[_j];
                table[_j]=table[_j-1];
                table[_j-1]=temp;
            }//end if
           _j=_j-1;
        }//end while j
        _i=_i+1;
    }//end while i
    
     println!("Array in asceding order:");
    for _i in 0..table.len(){
        println!("{} ",table[_i]);
    }
    
    //buble sort descending order
      _i=0;
    while _i<table.len(){
        _j=table.len()-1;
        while _j>_i{
           if table[_j-1]<table[_j]{
              temp=table[_j];
              table[_j]=table[_j-1];
              table[_j-1]=temp;
           }//end if
           _j=_j-1;
        }//end while j
        _i=_i+1;
    }//end while i
    println!("Array in descending order:");
    for _i in 0..table.len(){
      println!("{}",table[_i]);
    }
} 
