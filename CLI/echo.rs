use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    for item in args.iter(){
        println!("{}",item);
    }
}
