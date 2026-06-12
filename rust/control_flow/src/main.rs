fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let number = if number % 3 == 0 {5} else {6};
    println!("The value of number is {number}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // This will be the value passed to result.
            /*
                Can also return from inside a loop.
                While break only exits the current loop,
                return always exits the current function.
             */
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // You can disambiguate loops with loop labels.
    // They start with a '.
    let mut count = 0;
    'outer_loop: loop {
        println!("count = {count}");
        let mut remaining = 10;

        'inner_loop: loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // Ommiting the 'inner_loop label here
                // would have the same effect.
                break 'inner_loop;
            }
            if count == 2 {
                break 'outer_loop;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut num: i32 = 3;

    while num != 0 {
        println!("{num}!");

        num -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // Error prone approach.
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("current: {element}");
    }

    // number assumes values 3,2,1
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    let fruits = ["apple", "banana", "cherry"];

    // The tuple pattern (index, element) unpacks both values
    for (index, fruit) in fruits.iter().enumerate() {
        println!("Index: {}, Fruit: {}", index, fruit);
    }

    let mut numbers = [10, 20, 30];

    for (index, value) in numbers.iter_mut().enumerate() {
        // Dereference `value` with * to modify the array item directly
        *value += index; 
    }
    
    println!("{:?}", numbers); // Outputs: [10, 21, 32]

    let items = ['a', 'b', 'c'];
    for i in 0..items.len() {
        println!("Index: {}, Value: {}", i, items[i]);
    }
}
