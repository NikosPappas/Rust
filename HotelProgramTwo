use std::io;
fn main() {
    const N:usize=5;
    const MONTHS:usize=12;
    let mut _names:Vec<String>=vec![String::new();N];
    let mut _income:[[f64;MONTHS];N]=[[0.0f64;MONTHS];N];
    let mut user_input;
    let mut _i:usize;
    let mut _max_salary:[f64;MONTHS]=[0.0;MONTHS];
    let mut _salary;
    //read the names of the hotels
    for i in 0.._names.len(){
        println!("Type the name of hotel number {} ",i);
        user_input=String::new();
        io::stdin()
           .read_line(&mut user_input)
           .expect("Failed to read line");
        _names.insert(i,user_input);
    }
    //for each hotel read the monthly salary
   
    for _i in 0.._names.len(){
       println!("Hotel name: {} ",_names[_i]);
         for j in 0..MONTHS{
            println!("Type the amount of income: ");
            user_input=String::new();//return an empty string
            io::stdin()
               .read_line(&mut user_input)
               .expect("Failed to read line");
            let user_input: f64=user_input.trim().parse().expect("Type a number please");
            _income[_i][j]=user_input;
       }
    }
  for i in 0.._income.len(){
       _salary=_income[i][0];
     for j in 0..MONTHS{
      if _income[i][j]>_salary{
          _salary=_income[i][j];
          _max_salary[j]=_income[i][j];
      }
    }
  }
  
}//end of main
