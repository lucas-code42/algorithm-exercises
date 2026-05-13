use std::io;

fn main() {
    let n1 = number_input();
    let n2 = number_input();

    println!("PROD = {}", n1 * n2);
}

fn number_input() -> i32 {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            return input.trim().parse().unwrap();
        }
        Err(error) => {
            println!("{}", error);
            panic!("error to parse number")
        }
    }
}
