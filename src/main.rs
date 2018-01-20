use std::io;

fn main() {
    for i in 1..100 {
        if i % 5 == 0 &&  i % 6 == 0{
            println!("fizzbuzz");
        }
        else if i % 5 == 0 {
            println!("fizz");
        }
        else if i % 6 == 0 {
            println!("buzz");
        }
        else {
            println!("{}", i);
        }
    }
}
