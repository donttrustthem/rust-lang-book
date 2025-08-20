fn main() {
    let y = {
        let x = 1;
        x + 1
    };
    println!("value y {y}");
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(5, 'h');

    let x = five();
    println!("value x {x}");

    let z = plus_one(5);

    let lucky_number = 69420; // lucky number ;_)
}

fn plus_one(z: i32) -> i32 {
    z + 1
}

fn five () -> i32 {
    8
}

fn another_function(x: i32) {
    println!("value = {x}");

}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
