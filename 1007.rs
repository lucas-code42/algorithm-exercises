use std::io;

fn main() {
        let mut v: [i32; 4] = [0, 0, 0, 0];
        for i in 0..v.len() {
                v[i] = input();
        }
        println!("DIFERENCA = {}", (v[0] * v[1] - v[2] * v[3]))
}

fn input() -> i32 {
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
                        panic!("{error}");
                }
        }
}
