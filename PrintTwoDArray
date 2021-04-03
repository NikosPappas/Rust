use std::io;
fn main() {
    const M:usize=5;
    const N:usize=5;
    let mut table:[[i32;M];N]=[[0;N];M];
    let mut i:usize;
    let mut j:usize;
    for i in 0..N{
      for j in 0..M{
          table[i][j]=(i as i32)*(j as i32);
      }
    }
    for i in 0..N{
      for j in 0..M{
        print!("{} ",table[i][j]);
      }
      println!("");
    }
}
