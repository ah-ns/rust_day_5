fn main() {
    // Practice with ternary operator and array
    let condition = false;
    let [x, y] = if condition {[2, 8]} else {[5, 9]};
    let t = [x, y];

    for element in t.iter() { // Can't iterate over a tuple since they can be different datatypes
        println!("{}", element);
    }

    let z = first_function(x, y);
    println!("The value of z is {}", z);

    let mut counter = 0; // Must make mutable so we can update in a loop
    let result = loop {
        counter += 1;

        if counter == z {
            break counter + 7; // Break and return counter + 7
        } // No semicolon needed after if statements
    }; // Add semicolon after loops

    println!("The result of z + 7 is {}", if result % 2 == 0 {"even"} else {"odd"}) // Ternary operator inside of print
}

fn first_function(x: i32, y: i32) -> i32 {
    let z = x + y;
    z   // Can use "return" or simply the thing to return because it is implied.
        // Last expression has the semicolon removed in a function 
}