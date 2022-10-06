fn main() {
    let a = [10, 20, 30, 40, 50];
    
    /*while loop to iterate an array.
      this could be error prone if the array length is incorrect
      it is also slow since compiler has to add the conditional check for each element in array
    */
    let  mut index = 0;

    while index < 5 {
        println!("the value is: {}",a[index]);

        index = index + 1;
    }
    
    println!("separator line for the two loop types");

    //for loop example for the same

    for element in a.iter(){
        println!("the value is: {}", element);
    }
}
