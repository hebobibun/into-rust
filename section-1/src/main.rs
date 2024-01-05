fn main() {
    variable();
    constant();
}

fn constant() {
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("Number of seconds in a minute: {}", SECONDS_IN_MINUTE);
}

fn variable() {
    // Declare and initialize a mutable variable 'x' with the value 4.
    let mut x = 4;
    println!("X is: {}", x);

    // Create a new scope with a shadowed variable 'x'.
    {
        // Declare a new 'x' in the inner scope, subtracting 2 from the outer 'x'.
        let x = x - 2;
        println!("X is: {}", x);
    }

    // Update the outer 'x' by adding 1.
    x = x + 1;
    println!("X is now: {}", x);

    // Declare and initialize an immutable variable 'y' with the value 6.
    let y = 6;
    println!("Y is: {}", y);

    // Shadow 'y' with a new binding of type string.
    let y = "Hello World!";
    println!("Y is now: {}", y);
}
