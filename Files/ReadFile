use std::fs::File;
use std::io::prelude::*;
fn main()->std::io::Result<()> {
  let path="/proc/cpuinfo";
  let mut file=File::open(&path)?;
  let mut buffer_reader:String=String::new();
  file.read_to_string(&mut buffer_reader)?;
  print!("{}",buffer_reader);
  drop(file);
  Ok(())
}
