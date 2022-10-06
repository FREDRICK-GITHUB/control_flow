fn main() {
    let condition = true;

    let number = if condition {
        5
    }else{
        6
    };
 //conditions of the arms in this if expression must be of the same type
    println!("the value of number is: {}", number);
}
