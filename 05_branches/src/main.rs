fn main() {
    let number = 5;

    if number < 5 {
        println!("The number is too low!");
    } else if number == 5 {
        println!("The number is a perfect match!");
    } else {
        println!("The number is high enough!");
    }

    let rounded = if number < 5 { 0 } else { 1 };
    println!("The rounded value is: {rounded}");
}
