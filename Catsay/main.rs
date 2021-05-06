extern crate structopt;
extern crate colored;
use colored::*;
use structopt::StructOpt;
#[derive(StructOpt)]
struct Options{
  #[structopt(default_value= "Meow!")]
  ///What does the cat say?
  message:String,
  #[structopt(short = "d", long = "dead")]
  ///Make the cat appear dead
  dead:bool,
  #[structopt(short "f", long="file", parse(from_os_str))]
  catfiel: Option<std::path::PathBuf>,
  #[structopt(short="i", long = "stdin")]
  //Read the message from STDIN instead of the argument
  stdin:bool,
}
use std::io;
fn main() {
    let options=Options::from_args();
    let message = String::new();
    if options.stdin{
      io::stdin().read_to_string(&mtu message)?;
    }
    else{
      message=options.message;
    }
    let eye = if options.dead {"x"} else {"o"};//[1]
    if message.to_lowercase() == "woof"{
     eprintln!("A cat shouldn't bark like a dog.");
    }
    println!("{}",message.bright_yellow().underline().on_purple()); 
    println!(" \\");
    println!("  \\");
    println!("    /\\_/\\");
    println!("   ( {eye}  {eye} )",eye=eye.red().bold());
    println!("   =( I )=");
   
}
