use std::io;

const FIRST_EXAM: f32 = 2.;
const SECOND_EXAM: f32 = 3.;
const THIRD_EXAM: f32 = 4.;
const FOURTH_EXAM: f32 = 1.;
const SUM_ALL: f32 = FIRST_EXAM + SECOND_EXAM + THIRD_EXAM + FOURTH_EXAM;
const APPROVED: f32 = 7.;
const REPPROVED: f32 = 5.;
const GRADE_LIMIT: f32 = 6.9;
const EXAM_APPROVED: f32 = 5.;
const EXAM_REPPROVED: f32 = 4.9;

fn main() {
    let avarage_grade: f32 = calculate();

    println!(Media: {:.1}, avarage_grade);
    match avarage_grade {
        n if n >= APPROVED => println!(Aluno aprovado.),
        n if n < REPPROVED => println!(Aluno reprovado.),

        n if n >= REPPROVED && avarage_grade <= GRADE_LIMIT => {
            println!(Aluno em exame.);
            let input = input();
            let splited_values = split_input(&input);
            let exam_grade = extract_numbers(&splited_values)[0];

            print!(Nota do exame: );
            println!({}, exam_grade);

            let new_grade = (exam_grade + avarage_grade) / 2.;
            match new_grade {
                nn if nn >= EXAM_APPROVED => println!(Aluno aprovado.),
                nn if nn <= EXAM_REPPROVED => println!(Aluno reprovado.),
                _ => panic!(),
            }
            println!(Media final: {:.1}, new_grade);
        }
        _ => panic!(),
    }
}

fn calculate() -> f32 {
    let input = input();
    let splited_values = split_input(&input);
    let grades = extract_numbers(&splited_values);

    let avarage_grade: f32 = calculate_average(&grades);
    return avarage_grade;
}

fn calculate_average(data: &Vec<f32>) -> f32 {
    let result = (FIRST_EXAM * data[0]
        + SECOND_EXAM * data[1]
        + THIRD_EXAM * data[2]
        + FOURTH_EXAM * data[3])
        / SUM_ALL;

    return result;
}

fn extract_numbers(data: &Vec<&str>) -> Vec<f32> {
    let mut numbers: Vec<f32> = Vec::new();
    for i in 0..data.len() {
        let tmp: f32 = data[i]
            .trim()
            .replace(n, )
            .parse()
            .expect(error to parse data);

        numbers.push(tmp);
    }

    return numbers;
}

fn split_input(input: &String) -> Vec<&str> {
    let splited_input = input.split( ).collect();
    return splited_input;
}

fn input() -> String {
    let mut input: String = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            return input;
        }
        Err(error) => {
            panic!({error});
        }
    }
}

