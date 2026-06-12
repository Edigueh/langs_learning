fn main() {
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1 // This is an expression as
              // this does not end with a semicolon.
    };

    // The value of y is 4
    println!("The value of y is {y}");
    let y = increment(y);
    println!("The value of y is {y}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// In Rust, the return value of a function is 
// synonymous with the value of the final expression in
// the block of the body of a function.
fn increment(x: i32) -> i32 {
    x + 1 // Adding a ';' here would cause compile errors.
}
