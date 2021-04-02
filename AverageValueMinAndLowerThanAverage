use std::io;
fn main() {
    const N:usize=10;//declare and initialize the size of the array.Holds the size of the array.
    let mut arr:[i32;N]=[0;N];
    let mut sum:i32 = 0;//declare and initialize sum variable.Holds the sum of the array elements
    let mut average: f64=0.0;//declare and initialize average variable.Holds the average of the elements
    let mut element_lower_than_average:i32;//declare and initialize variable element
    let mut minimum_array_element:i32;
    let mut position_of_minumun_array_elemnt:i32;
    let mut input_value;
    let mut lower_than_average:i32=0;
    let mut position_of_minimum_array_element:usize=0;
    let mut i:usize=0;
   
    while i<N{
       println!("Type the {} element of the array: ",i+1);
       input_value=String::new();
       io::stdin()
           .read_line(&mut input_value)
           .expect("Failed to read line");
       let input_value:i32=input_value.trim().parse().expect("Type a number please");
       arr[i]=input_value;
       sum=sum+arr[i];
       i=i+1;
    }//end of while loop

    //find the average of the elements of the array
    average=((sum/(N as i32)) as f64);
    println!("The average value is:{}",average);
   
    //find the elements which are lower than the average value
    i=0;
    while i<N{
        if (arr[i] as f64) < average{
            println!("The array element {} is lower than the average value {}",arr[i],average);
            lower_than_average=lower_than_average+1;
        }
        i=i+1;
    }//end of while loop
   
    //print the element lower than the average value
    println!("Total elements lower than the average:{}",lower_than_average);
    minimum_array_element=arr[0];
    i=1;
    while i<N{
        if arr[i]<minimum_array_element{
           minimum_array_element=arr[i];
           position_of_minimum_array_element=i;
        }
        i=i+1;
    }//end of while loop

    //print the minimum array element
    println!("Minimum array element:{}",minimum_array_element);
    //print the position of minimum array element
    println!("Position of minimum array element:{}",position_of_minimum_array_element+1);
}
