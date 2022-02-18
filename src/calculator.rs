use std::env::{args, Args};

pub fn calculator() {
    // let mut args: Args = args();

    // let first = args.nth(1).unwrap();
    // let operator = args.nth(0).unwrap().chars().next().unwrap();
    // let second = args.nth(0).unwrap();

    // let first_number = first.parse::<f32>().unwrap();
    // let second_number = second.parse::<f32>().unwrap();

    // let result: f32 = operate(operator, first_number, second_number);
    // println!("{}", ouput(first_number, operator, second_number, result));
    test();
}

fn operate(operation: char, first_number: f32, second_number: f32) -> f32 {
    if second_number == 0.0 && operation == '/' {
        panic!("Cannot divide with 0");
    }

    match operation {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        '%' => first_number % second_number,
        _ => panic!("Invalid operator used."),
    }
}

fn ouput(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!(
        "{} {} {} = {}",
        first_number, operator, second_number, result
    )
}

fn test() {
    let operations = ['+', '-', '/', 'x', '%'];
    for i in operations {
        for first_number_test in 0..200 {
            for second_number_test in 1..201 {
                let result: f32 = operate(i, first_number_test as f32, second_number_test as f32);
                println!(
                    "{}",
                    ouput(
                        first_number_test as f32,
                        i,
                        second_number_test as f32,
                        result as f32,
                    )
                );
            }
        }
    }
}
