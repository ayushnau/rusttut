fn main() {
    println!("Hello, world!");
    // let x: u8 = 14 as u16; //error
    // let x: u8 = 14_u16; // error
    // let x = 14_011u16;
    // let x = 12_32_32323u32;

    // println!("{}", x);
    let x = 21_i8;
    let y: i32 = 42;
    let result = x / (y as i8);
    println!("{}", result);
}
