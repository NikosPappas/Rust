use std::io;
#[derive(Clone,Copy,Debug)]
struct Point{
     x: f64,
     y: f64,
}
impl Point{
   fn init_point()->Point{
     Point{
          x:0.0,
          y:0.0,
       }
   }
   fn set_x(&mut self,x:f64){
     (*self).x=x;
   }
   fn set_y(&mut self,y:f64){
     (*self).y=y;
   }
   fn print_point(&self){
     println!("(x,y)=({},{})",(*self).x,(*self).y);
   }
}
fn main() {
    const N:usize=5;
    let mut _array_of_points:[Point;N]=[Point::init_point();N];
    let mut _i:usize;
    let mut _x:String;
    let mut _y:String;
    for i in 0..N{
       println!("Type the x coordinate of the point: ");
       _x=String::new();
       io::stdin().read_line(&mut _x).expect("Failed to read _x coordinate");
       let _x:f64=_x.trim().parse().expect("Type a number please for the x coordinate");
       println!("Type th y coordinate of the point: ");
       _y=String::new();
       io::stdin().read_line(&mut _y).expect("Failed to read _y coordinate");
       let _y: f64=_y.trim().parse().expect("Type a number please for the y coordinate");
       _array_of_points[i].x=_x;
       _array_of_points[i].y=_y;
    }
    let end=_array_of_points.len()-1;
    quick_sort(&mut _array_of_points,0,end);
    println!("Points are sorted in asceding order with key the x coordinate");
    for i in 0..N{
      _array_of_points[i].print_point();
    }

}

fn quick_sort(a:&mut [Point],start:usize,end:usize){
  if start<end{
    let mut pivot:i32=partition(a,start,end);
    quick_sort(a,start,(pivot-1) as usize);
    quick_sort(a,(pivot+1) as usize,end);
  }
}

fn partition(a:&mut [Point],start:usize,end:usize)->i32{
 let mut pivot:Point=a[end];
 let mut _i=start;
 let mut _j=start;
 while _j < end{
     if a[_j].x < pivot.x {
        swap(a,_i,_j);
        _i+=1;
     }
   _j+=1;
 }
  swap(a,_i,_j);
  return _i as i32;
}

fn swap(a: &mut [Point],_i:usize,_j:usize){
  let mut temp:Point=a[_i];
          a[_i]=a[_j];
          a[_j]=temp;
}
