const ONE_MINUTE: u32 = 60;
const ONE_HOUR: u32 = ONE_MINUTE * 60;

fn main() {
    let mut number = 10;
    println!("number = {number}");

    number += 1;
    println!("number = {number}");

    println!("One minute is {ONE_MINUTE}s")
}