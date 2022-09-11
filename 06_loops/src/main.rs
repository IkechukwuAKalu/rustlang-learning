fn main() {
    // loops
    loop {
        println!("Testing loops!");
        break;
    }

    // returning values from loops
    let mut counter_1 = 1;

    let result = loop {
        counter_1 += 1;

        if counter_1 == 10 {
            break counter_1 * 2;
        }
    };

    println!("The result of the first loop is: {result}");

    // using loop labels (can be any arbitrary string)
    let mut counter_2 = 1;

    'outer_loop: loop {
        counter_2 += 1;
        let mut inner_counter = counter_2;

        loop {
            inner_counter -= 1;

            if inner_counter == 0 {
                break;
            } else if inner_counter % 13 == 0 {
                println!("Breaking from the inner loop at: {inner_counter}");
                break 'outer_loop;
            }
        }
    }

    println!("The result for the second loop using labels is: {counter_2}");

    // while loops
    let mut count_down = 3;

    while count_down != 0 {
        println!("{count_down}");
        count_down -= 1;
    }

    println!("Go Go Go!!!");

    // for loop
    let arr = ["Hello", ", ", "World", "!"];

    for text in arr {
        print!("{text}");
    }

    print!("\n");

    for x in (1..4).rev() {
        println!("{x}");
    }

    println!("Go Go Go Again!!!");

    for i in 1..20 {
        let fib_value = fibonacci(i);
        println!("The Fibonacci value of {i} is: {fib_value}");
    }
}

fn fibonacci(n: u32) -> u32 {
    if n == 1 {
        0
    } else if n == 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}