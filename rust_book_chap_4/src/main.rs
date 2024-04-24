fn main() {
    /* Ownership */
    
    let s = "hello";

    { 
        let s = "hello from the other side";
        // this string goes out of scope and is dropped
    }

    let mut s = String::from("hello");
    s.push_str(" world....");
    println!("{s}");
    // for a string literal the value is hardcoded directly whereas for a string object the value is stored in the heap.
    // as the string object's size is not fixed

    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;

    

}
