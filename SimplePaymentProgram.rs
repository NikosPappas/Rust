use std::io;
fn main() {
    println!("Type the amount of money you will give to buy a new computer: ");
    let mut amount=String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read line");
    let amount: i32 = amount.trim().parse().expect("Please type a number: ");
    println!("Type the first cash installment: ");
    let mut first_cash_installment = String::new();
    io::stdin()
        .read_line(&mut first_cash_installment)
        .expect("Failed to read line");
    let mut first_cash_installment: f64 = first_cash_installment.trim().parse().expect("Type a number please");
    let mut number_of_weeks: i32 = 1;
    while first_cash_installment <= (amount as f64) {
       first_cash_installment=first_cash_installment*(number_of_weeks as f64);
       number_of_weeks=number_of_weeks+1;
    }
    println!("You will pay of your new computer in {} weeks ",number_of_weeks);
    let  money_left: f64 =(amount as f64)-first_cash_installment;
    if money_left > 0.0{
      println!("The money left in your bank account is {}",money_left);
    }
    else{
        println!("You have no money left in you bank account.");
    }
}
