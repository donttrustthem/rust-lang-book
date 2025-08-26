fn main() {
    // let mut s = String::from("Hello");
    // let len = calculate_len(&s);
    // change(&mut s);
    // println!("The length of {s} is {len}. ");

    // { let r1 = &mut s;}
    // let r2 = &mut s;
    //
    // println!("{r1}, {r2}")

    // let r1 = &s;
    // let r2 = &s;
    // println!("{r1}, {r2}");
    //
    // let r3 = &mut s;
    // println!("{r3}")
    // let reference_to_nothing = dangle();
    let s = String::from("Hellodworld");
    let word = first_word(&s);

    println!("{word}");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// fn second_word(s: &String) -> (usize, usize) {
// }

// fn dangle () -> String { // &
//     let s = String::from("Hello");
//     s
//     // &s
// }
// fn calculate_len(s: &String) -> usize {
//     s.len()
// }
//
// fn change(some_string: &mut String) {
//     some_string.push_str(", world")
// }
