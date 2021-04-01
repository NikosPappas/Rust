use std::io;
fn main() {
    let mut c1: i32 =0;//count number between 1-20
    let mut c2: i32 =0;//count number between 21-50
    let mut c3: i32 =0;//count number between 51-80
    loop{
        println!("Type a number (type zero to exit): ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: i32 = input.trim().parse().expect("Type a number please");
        if input == 0{
            break;
        }
        else if input>=1 && input<=20{
            c1=c1+1;
        }
        else if input>=21 && input<=50{
            c2=c2+1;
        }
        else  if input>=51 && input<=80{
            c3=c3+1;
        }
        else{
            continue;
        }
        
    }
    println!("Numbers between 1-20:{}\nNumber between 21-50:{}\nNumbers between 51-80:{}",c1,c2,c3);
}
