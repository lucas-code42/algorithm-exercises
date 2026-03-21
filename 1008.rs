use std::io;

fn main() {
        let employee_number = input_integer();
        let workerd_hours = input_float();
        let salary = input_float();
        println!("NUMBER = {}", employee_number);
        println!("SALARY = U$ {:.2}", (workerd_hours * salary));
}

fn input_integer() -> i32 {
        let mut input: String = String::new();
        match io::stdin().read_line(&mut input) {
                Ok(_n) => {
                        let tmp: i32 = input
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
                        panic!("{error}");
                }
        }
}
