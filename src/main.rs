use std::{iter::once, iter::repeat};

fn main() {
    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
    let fizzbuzzes = fizzes.zip(buzzes);
    let mut counter: i8 = 1;
    let mut fizbuz_iter = fizzbuzzes.into_iter();
    loop {
        let element = fizbuz_iter.next();

        match (counter, element) {
            (15, Some(("", ""))) => {
                println!("15");
                break;
            }
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
        if counter == i8::MAX {
            fizbuz_iter.next();
            fizbuz_iter.next();
        }
    }
}
