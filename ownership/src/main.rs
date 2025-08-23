fn main() {
    // let mut s = String::from("Hello");
    // s.push_str(", world!");
    // println!("{s}");

    // let x = 5;
    // let y = x;
    // println!("{y}")

    // let s1 = String::from("Hello");
    // let s2 = s1.clone();
    // println!("s 1 = {s1} \ns 2 = {s2}")


    // let s = String::from("hello");
    // takes_ownership(s);
    // println!("{s}");
    // let x = 5;
    // makes_copy(x);
    // println!("{x}");
    // let s1 = gives_ownership();
    // let s2 = String::from("hello");
    // let s3 = takes_and_gives_back(s2);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");

}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

// fn gives_ownership() -> String {
//     let some_string = String::from("yours");
//     some_string
// }
//
// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }
// fn takes_ownership(some_string: String) {
//     println!("{some_string}");
//     return;
// }
//
// fn makes_copy(some_integer: i32) {
//     println!("{some_integer}");
// }
