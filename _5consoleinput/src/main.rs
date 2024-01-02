use std::io; //package::module and in the module we will use the function.

fn main() {
    println!("Hello, world!");
    let mut input: String = String::new();

    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("failed to read line");
    // creating the mutable ref to &mut input variable input which is allow the the function read_line to directly modify the String variable.
    // expect will catch any error.

    // println!("{}", input);

    // let mut input = 4;
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("failed to read line");
    // creating the mutable ref to &mut input variable input which is allow the the function read_line to directly modify the String variable.
    // expect will catch any error.
    // println!("{}", input)

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let numberKey: i64 = input.trim().parse().unwrap();
    println!("{}", numberKey)
}
