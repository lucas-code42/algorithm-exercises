use std::io;

fn main() {
        let user_input_1 = input();
        let entry_1 = split_input(&user_input_1);

        let user_input_2 = input();
        let entry_2 = split_input(&user_input_2);

        // println!("entry_1: {:.?}", entry_1);
        // println!("entry_2: {:.?}", entry_2);

        let result = calc(entry_1, entry_2);
        println!("{:.4}", result);
}
//            0    1
// entry_1 = [x1, y1]

//            0    1
// entry_2 = [x2, y1]
fn calc(e_1: Vec<f32>, e_2: Vec<f32>) -> f32 {
        let x = (e_2[0] - e_1[0]).powf(2.0);

        let y = (e_2[1] - e_1[1]).powf(2.0);
        (x + y).sqrt()
}

fn split_input(input: &String) -> Vec<f32> {
        let splited_input: Vec<&str> = input.split(" ").collect();

        let mut numbers: Vec<f32> = vec![];

        for i in 0..splited_input.len() {
                let tmp: f32 = splited_input[i].parse().expect("error to parse number");
                numbers.push(tmp);
        }
        numbers
}

fn input() -> String {
        let mut input: String = String::new();

        match io::stdin().read_line(&mut input) {
                Ok(_n) => {
                        return input.trim().replace("\n", "");
                }
                Err(error) => {
                        panic!("{}", error);
                }
        }
}
