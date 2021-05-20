use std::io;
fn main() {
    const N:usize=20;
    let mut table:[i32;N]=[0;N];;
    let mut i:usize;
    let mut user_input;
    for i in 0..table.len(){
        println!("Type the {} element of the array: ",i+1);
        user_input=String::new();
        io::stdin()
           .read_line(&mut user_input)
           .expect("Failed to read line");
        let user_input: i32=user_input.trim().parse().expect("Type a numnber please");
        if user_input==0{ table[0]=table[0]+1;}
        else if user_input==1{table[1]=table[1]+1;}
        else if user_input==2{table[2]=table[2]+1;}
        else if user_input==3{table[3]=table[3]+1;}
        else if user_input==4{table[4]=table[4]+1;}
        else if user_input==5{table[5]=table[5]+1;}
        else if user_input==6{table[6]=table[6]+1;}
        else if user_input==7{table[7]=table[7]+1;}
        else if user_input==8{table[8]=table[8]+1;}
        else if user_input==9{table[9]=table[9]+1;}
        else if user_input==10{table[10]=table[10]+1;}
        else if user_input==11{table[11]=table[11]+1;}
        else if user_input==12{table[12]=table[12]+1;}
        else if user_input==13{table[13]=table[13]+1;}
        else if user_input==14{table[14]=table[14]+1;}
        else if user_input==15{table[15]=table[15]+1;}
        else if user_input==16{table[16]=table[16]+1;}
        else if user_input==17{table[17]=table[17]+1;}
        else if user_input==18{table[18]=table[18]+1;}
        else {table[19]=table[19]+1;}
    }
    println!("Creating histogram");
    println!("__________________");
    for i in 0..table.len(){
       print!("{}",i+1);
       print!("\t");
      for j in 0..table[i]{
        print!("*");
     }
    println!();
   }
    

}
