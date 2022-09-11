fn main() {
    println!("Hello, world!");

    print_duration(2, 'm');

    let num = 5;
    let square_value = square(num);
    println!("Square value of {num} is: {square_value}");
}

// parameter -> in function definition
// argument  -> value passed when calling the function
fn print_duration(time: i8, unit_label: char) {
    println!("The duration is: {time}{unit_label}");
}

fn square(x: i32) -> i32 {
    x * x
}