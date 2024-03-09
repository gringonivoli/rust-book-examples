fn main() {
    // let s = String::from("hello");
    // takes_ownership(s);
    // let x = 5;
    // makes_copy(x);
    // println!("{x}");

    // let s1 = gives_ownership();
    // let s2 = String::from("hello");
    // let s3 = takes_and_gives_back(s2);
    // println!("{s1}");
    // println!("{s3}");
    let s1 = String::from("hello");
    let len = calculate_length3(&s1);
    println!("The length of '{s1}' is {len}");
    // let mut s2 = String::from("hello");
    // change(&mut s2);
    // println!("s2 = {s2}");
    // let reference_to_nothing = no_dangle();

    // let ss = "Hello World!";
    // let s = String::from("hello world!");
    // println!("The first word is: {}", first_word(&s));
    // println!("The first word is: {}", first_word(&ss));
}

fn first_word(s: &str) -> &str {
    // let bytes = s.as_bytes();
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// fn no_dangle() -> String {
//     let s = String::from("hello");
//     s
// }

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// fn change(a_strig: &mut String) {
//     a_strig.push_str(", world!");
// }

fn calculate_length3(some_string: &String) -> usize {
    some_string.len()
}

// fn calculate_length2(some_string: String) -> (usize, String) {
//     // let length = some_string.len();
//     (some_string.len(), some_string)
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("yours");
//     some_string
// }
// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

// fn takes_ownership(some_string: String) {
//     println!("{some_string}");
// }

// fn makes_copy(some_integer: i32) {
//     println!("some integer: {some_integer}");
// }
