use std::env;
fn main() {
    let _args:Vec<String>=env::args().collect();
    if _args.len() < 3 {
      println!("Error.Usage ./main a b c");
      std::process::exit(0);
    }
    else{
       let mut _t:f64;
       let _a:f64=_args[1].parse::<f64>().unwrap();
       let _b:f64=_args[2].parse::<f64>().unwrap();
       let _c:f64=_args[3].parse::<f64>().unwrap();
       _t=(_a+_b+_c)/2.00;
       let _area:f64=f64::sqrt(_t*(_t-_a)*(_t-_b)*(_t-_c));
       let _perimeter:f64=_a+_b+_c;
       println!("The area of the triangle is {} ",_area);
       println!("The perimeter of the triangle is {}",_perimeter);    
    }
}
