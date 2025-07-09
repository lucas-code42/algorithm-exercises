use std::{io, vec};

fn main() {
    let mut temp: Vec<i32> = vec![];
    for i in number_input() {
        temp.push(i - 1);
    }

    let mut sum = 0;
    for i in temp {
        sum = sum + i;
    }

    println!("{}", sum + 1);
}

fn number_input() -> Vec<i32> {
    let mut numbers: Vec<i32> = vec![];

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {}
        Err(error) => {
            println!("{}", error);
        }
    }

    let collection: Vec<&str> = input.split(" ").collect();

    for x in collection {
        let number: i32 = x.trim().parse().unwrap();
        numbers.push(number);
    }

    numbers
}
