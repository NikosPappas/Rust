use std::io;
fn main() {
    const N:usize=20;
    let mut table:[i32;N]=[0;N];
    let mut _i:usize;
    let mut index:isize=-1;
    let mut user_input;
    let mut key;
    for _i in 0..table.len(){
        println!("Type the {} of the array: ",_i+1);
        user_input=String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let user_input:i32=user_input.trim().parse().expect("Type a number please");
        table[_i]=user_input;
    }

    println!("Type the value to be searched: ");
    key=String::new();
    io::stdin()
        .read_line(&mut key)
        .expect("Failed to read line");
    let key:i32 = key.trim().parse().expect("Type a number please");

    _i=0;
    while _i<table.len() && index==-1{
        if table[_i]==key{
            index=_i as isize;
        }
        else{
            _i=_i+1;
        }
    }//end of while loop
    if index==-1{
        println!("Element not found ");
    }
    else{
        println!("Element found in position {}",index+1);
    }

}
