fn main() {
    // variable in rust default immutable
    // But when use 'mut' keyword, the variable is mutable
    // IMPORTANCE: Constant is immutable and cannot use mut keyword to make it to mutable
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    shadow();
    data_type();
}

/*
 * In rust have 2 data type:
 * Scalar consist of 4 types: integer, float-point number, boolean, charactor
 * Compound consist of 2 types: tuples, arrays
 */
fn data_type() {
    // Scalar
    let interger_type1: u32 = 43;
    let interger_type2 = 43;

    println!("{}", interger_type1);
    println!("{}", interger_type2);


    let _x = 2.0;
    let _y: f32 = 3.0;
    // Compound
    // tuples
    let t = (10, "hello", true);

    println!("{}", t.0);
    println!("{}", t.1);

    let (_id, _name, _actice) = t;

    // arrays
    let arr = [1,2,3];
    println!("{}", arr[0]);

    for x in arr {
        println!("{}", x);
    }
}

fn shadow() {
    let _str = "     ";
    let _str = _str.len();

    println!("Length of _str: {_str}");
}
