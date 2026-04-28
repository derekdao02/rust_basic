fn main() {
    print_lable_measurement(5, 'h');

    print_statement_expression();
    let r = print_funtion_return_value();
    println!("The value of funtion return value is: {r}");

    let d = plus_one_unit(10);
    println!("The result when plus 1 unit is: {d}");
}

fn print_lable_measurement(value: i32, unit_lable: char) {
    print!("The measurement is: {value}{unit_lable}");
}

fn print_statement_expression() {
    let t = {
        let z = 4;
        z + 1
    };

    println!("The value of t is: {t}");
}

fn print_funtion_return_value() -> i32 {
    4
}

fn plus_one_unit(data: i32) -> i32 {
    data + 1
}
