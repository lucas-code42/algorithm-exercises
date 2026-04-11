use std::io;

fn main() {
    let range: usize = input().parse().expect("error to parse range");
    // if range > 1000 || range < 1 {
    //     panic!("numero deve ser entre 1 e 1000")
    // }

    let numbers_input = input();
    let input_vec = input_to_vec(numbers_input);

    // if input_vec.len() != range {
    //     panic!("quantidade de numeros deve ser igual ao range declarado")
    // }

    let result = find_lower_and_position(input_vec);
    println!("Menor valor: {}", result.0);
    println!("Posicao: {}", result.1);
}

fn find_lower_and_position(v: Vec<i32>) -> (i32, i32) {
    let mut lower = v[0];
    let mut pos = 0;

    for i in 0..v.len() {
        let current = v[i];
        if current < lower {
            lower = current;
            pos = i as i32;
        }
    }

    (lower, pos)
}

fn input_to_vec(input: String) -> Vec<i32> {
    let splited_input: Vec<&str> = input.split_whitespace().collect();

    let mut numbers: Vec<i32> = vec![];

    for i in 0..splited_input.len() {
        let tmp: i32 = splited_input[i].parse().expect("error to parse number");
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
            panic!("error to read line: {}", error);
        }
    }
}
