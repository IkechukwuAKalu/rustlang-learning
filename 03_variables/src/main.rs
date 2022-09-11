fn main() {
    // mutable variable declaration
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // immutable variable declaration
    let count = {
        let temp = 3;
        temp + 1
    };
    println!("The value of count is: {count}");

    // constant declaration
    const THREE_HOURS_IN_SECONDS : u32 = 3 * 60 * 60;
    println!("The constant THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // shadowing
    let y = 10;
    println!("The value of y is: {y}");
    let y = y + 2;
    println!("The value of y is: {y}");

    {
        let y = y * 2;
        println!("The value of y is: {y}");
    }

    println!("The value of y is: {y}");

    // tuples - fixed length, can be different data types
    let  tuple_a : (i16, f32, u8) = (500, 32.88, 10);
    let _tuple_b = ("Hello", 1, 'W', 0x00);
    
    let (_a, b, _c) = tuple_a;
    println!("The second element in tuple_a is: {b}");
    
    let tuple_a_first_elem = tuple_a.0;
    println!("The first element in tuple_a is: {tuple_a_first_elem}");

    // arrays - fixed length, only same data types
    let  array_a : [u8; 5] = [1, 2, 3, 4, 5];
    let array_a_first_elem = array_a[0];
    println!("The first element of array_a is: {array_a_first_elem}");

    let _array_b = ["Hello", "World"];
    let _array_c = [0; 4]; // [0, 0, 0, 0]
}