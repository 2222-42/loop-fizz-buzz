use std::{iter::once, iter::repeat};

fn main() {
    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
    let fizzbuzzes = fizzes.zip(buzzes);
    let mut counter : i8 = 0;
    let mut fizbuz_iter = fizzbuzzes.into_iter();
    loop {
        let element = fizbuz_iter.next();
        match (counter, element) {
            (i, Some(("", ""))) => {
                println!("{}", i);
            }
            (_, Some((fizz, buzz))) => {
                println!("{}{}", fizz, buzz);
            }
            (_, None) => {
                break;
            }
        }
        counter += 1; 
    }

}
