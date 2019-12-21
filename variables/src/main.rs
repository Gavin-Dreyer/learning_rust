fn main() {
    //mut allows you to change a variable value without calling let
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    //const declares a variable with the data type annotated, that can never be changed
    // const MAX_POINTS: u32 = 100_000;

    //let allows you to mutate a value
    // let x = 5;

    // let x = x + 1;

    // let x = x * 2;

    // println!("The value of x is: {}", x);

    //let allows you to mutate variable types
    // let spaces = "    ";
    // let spaces = spaces.len();
    // println!("The number of spaces in spaces is: {}", spaces);

    //float
    // let x = 2.0; // f64
    // let y: f32 = 3.0; // f32

    //operators
    // // addition
    // let sum = 5 + 10;

    // // subtraction
    // let difference = 95.5 - 4.3;

    // // multiplication
    // let product = 4 * 30;

    // // division
    // let quotient = 56.7 / 32.2;

    // // remainder
    // let remainder = 43 % 5;

    //boolean
    // let t = true;
    // let f: bool = false; // with explicit type annotation

    // char
    // let c = 'z';
    // let z = 'ℤ';
    // let heart_eyed_cat = '😻';

    //tuple
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let tup = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // println!("The value of y is: {}", y);

    //destructing tuple with x
    // let x: (i32, f64, u8) = (500, 6.4, 1);
    // let five_hundred = x.0;
    // let six_point_four = x.1;
    // let one = x.2;

    //an array with an explicit type
    // let a: [i32; 5] = [1, 2, 3, 4, 5];

    //creating an array with the same value for each index
    // let a = [3; 5];
    // println!("Array a at all index is {}", a[0]);
}
