fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    println!("Three hours are equal to {THREE_HOURS_IN_SECONDS} seconds.");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x in the outer scope is: {x}");

    let spaces = "   ";
    println!("Separate {spaces} me");
    let spaces = spaces.len();
    println!("Separate {spaces} me");

    let _x = 2.0;
    let _y: f32 = 3.0;

    let _t = true;
    let _f: bool = false;

    // Characters are represented according to Unicode scalar values.
    let _c = 'z';
    let _z: char = 'ℤ';

    // Compound types.
    // Here you have to define the types directly inside the tuple definition.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Here I unwrap 
    let (x, y, z) = tup;

    println!("Here it goes\nx = {x}\ny = {y}\nz = {z}");

    // Tuple elements are indexed.
    let tup_first = tup.0;
    let tup_sec = tup.1;
    let tup_trd = tup.2;
    println!("five_hundred: {tup_first}");
    println!("six_point_four: {tup_sec}");
    println!("one: {tup_trd}");

    // Arrays
    let a: [i32; 5] = [1,2,3,4,5];
    let _months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
                              "August", "September", "October", "November", "December"];        
    // let b = [3, 3, 3, 3, 3];
    let _b = [3; 5];
    let _a_first = a[0];
    let _a_second = a[1];
    // Array indexes goes from 0 to len - 1;
}
