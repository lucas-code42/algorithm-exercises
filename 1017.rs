use std::io;

const KM_PER_LITER: f32 = 12.0;

fn main() {
        let h = input_float();
        let v = input_float();
        println!("{:.3}", (h * v) / KM_PER_LITER);
}

fn input_float() -> f32 {
        let mut input: String = String::new();
        match io::stdin().read_line(&mut input) {
                Ok(_n) => {
                        let tmp: f32 = input
                                .trim()
                                .replace("\n", "")
                                .parse()
                                .expect("error to parse data");
                        return tmp;
                }
                Err(error) => {
                        panic!("{}", error);
                }
        }
}
