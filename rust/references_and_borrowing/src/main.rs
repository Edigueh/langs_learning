fn main() {
    let s1 = String::from("hello");

    let len = calc_len(&s1);

    println!("The length of {s1} is {len}");

    let mut s = String::from("this is a string");

    let _r1 = &mut s;
    mutate(&mut s);

    // Doesn't work because r1 first borrowed s.
    // s is a second borrow, references cannot be referenced more than once.

    println!("{s}");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{r1}, {r2}, and {r3}");
}

fn calc_len(s: &String) -> usize { // s is a reference to a String
    s.len()
} // s goes out of scope. s doesn't have ownership, so nothing is dropped.

fn _change(_some_string: &String) {
    // `_some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    // this doesn't work!
    // _some_string.push_str(", world!");
}

fn mutate(some_string: &mut String) {
    some_string.push_str(" that can be mutated");
}
/*
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope and is dropped, so its memory goes away.
  // Danger!
*/

fn _no_dangle() -> String {
    let s = String::from("hello");
    s
}
