fn main() {
    let var = String::from("asas");
    let mut s = String::from("hello world");

    // fwlen will get the value 5
    let fw = first_word(&s); 

    println!("first word is {fw}");

    // this empties the String, making it equal to ""
    s.clear();

    // fwlen still has the value 5 here, but s no longer has any content that we
    // could meaningfully use with the value 5, so fwlen is now totally invalid!    

    // let hello = &s[0..5]; is equivalent
    let _hello = &s[..5];
    
    // let world = &s[6..s.len()]; is equivalent
    let _world = &s[6..];

    // let slice = &s[0..s.len()d]; is equivalent
    let _slice = &s[..];
}

// &str is the string slice type.
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
