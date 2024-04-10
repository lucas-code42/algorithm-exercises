use std::io;

fn main() {
    let total_1 = simple_calc();
    let total_2 = simple_calc();

    println!("VALOR A PAGAR: R$ {:.2}", total_1 + total_2);
}

fn simple_calc() -> f32 {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {}
        Err(error) => {
            println!("{}", error);
        }
    }

    let parts = input.split(" ");
    let collection: Vec<&str> = parts.collect();

    let qt: i32 = collection[1].trim().parse().expect("teste");
    let value: f32 = collection[2].trim().replace("\n", "").parse().expect("msg");

    let total: f32 = qt as f32 * value;

    return total;
}
