use std::io;

fn main() {
    let n1 = input_integer() as f32;
    let n2 = input_floater();

    println!("{:.3} km/l", n1 / n2);
}

fn input_integer() -> i32 {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            let n: i32 = input.trim().parse().unwrap();
            return n;
        }
        Err(error) => {
            println!("{}", error);
            panic!("error to parse number")
        }
    }
}

fn input_floater() -> f32 {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            let n: f32 = input.trim().parse().unwrap();
            return n;
        }
        Err(error) => {
            println!("{}", error);
            panic!("error to parse number")
        }
    }
}
