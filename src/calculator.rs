use std::env::{args, Args};

pub fn calculator() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();

    let result: f32 = operate(operator, first_number, second_number);
    println!("{}", result);
}

fn operate(operation: char, first_number: f32, second_number: f32) -> f32 {
    if operation == '+' {
        return first_number + second_number;
    } else if operation == '-' {
        return first_number - second_number;
    } else if operation == '*' {
        return first_number * second_number;
    } else if operation == '/' {
        return first_number / second_number;
    } else {
        return 0.0;
    }
}
