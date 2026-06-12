fn main() {
    // :: namespaces the 'from' function under the String type.
    // String literal text is hardcode directly into the final
    // executable
    let no_mut_s  = String::from("hello");
    let mut s = no_mut_s;
    s.push_str(", world!");
    println!("{s}");

    {
        // x memory is allocated here.
        let _x = 5;
    } // here, x memory is freed by Rust's 'drop' function.
    /*
        In C++, this pattern of deallocating resources at the end
        of an item’s lifetime is sometimes called
        Resource Acquisition Is Initialization (RAII).
     */

    let s1 = String::from("hello");
    let s2 = s1;
    // s1 is no longer value, and s2 is the owner of that memory address.
    // This is called a move. s1 was moved into s2.
    /*
        Here, s1 and s2 are structures with the ptr value pointing
        to the same memory address. This is because string is an 'Object'.
     */
    // printing s1 here won't work.
    println!("{s2}, world!");

    s = String::from("ahoy");
    println!("{s}, world!");
    // Here, the 'hello' space in memory will be freed right after
    // ressigning s.

    let a = String::from("damn");
    let b = a.clone();
    // Clone reservates two distinct regions in memory with the same content.
    // When you see a call to clone, you know that some arbitrary code is
    // being executed and that code may be expensive.
    // It’s a visual indicator that something different is going on.
    println!("a: {a}, b: {b}");

    let x = 5;
    let y = x;
    // This is not a move and x stills valid.
    // This is because integers have a known size at
    // compile time, and are stored entirely on the stack.
    // There's no diff between deep and shallow copy here.
    // Internally, integers implement the Copy trait annotation.
    // Rust doesn't let you implement Copy trait if Drop trait is implemented.
    println!("{x} is {y}");

    /*
        Types that implement copy trait:
            - All the integer types, such as u32.
            - The Boolean type, bool, with values true and false.
            - All the floating-point types, such as f64.
            - The character type, char.
            - Tuples, if they only contain types that also implement Copy.
              For example, (i32, i32) implements Copy, but (i32, String) does not.
     */
    takes_ownership(a);
    // println!("{a}"); // won't work here, because `a` memory has been freed up.
    // takes_ownership took ownership over `a`.
    makes_copy(x);

    // Return values and Scope.
    // Returning values can also transfer ownership
    let _mine = gives_ownership();
    let borrow = String::from("borrow");
    let borrowed = takes_and_gives_back(borrow);

    let (ss, len) = calc_length(borrowed);
    println!("Length of `{ss}` is `{len}`");
}

fn gives_ownership() -> String {
    // move its return value into the function that calls it
    String::from("yours") // some_string comes into scope and is returned moving out
    // to the calling function

}

fn takes_ownership(some_string: String) { // some_string comes into scopes
    println!("tooked ownership over {some_string}");
} // `drop` is called over some_string, freeing up backing memory.

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn makes_copy(some_int: i32) { // some_integer comes into scope.
    println!("copied {some_int}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn calc_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
