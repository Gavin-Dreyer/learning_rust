fn main() {
    println!("Hello, world!");

    //arguments
    // another_function(5, 6);

    //statements, expressions, and scope
    // let x = 5;
    // let y = {
    //     let x = 3;
    //     x + 1
    //expressions don't use semicolons. If used, turns expression into a statement, and statements don't return values
    // };
    // println!("The value of y is: {}", y);

    //calling a function
    // let x = five();
    // println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

// function with parameters
// fn another_function(x: i32, y: i32) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }

// fn five() -> i32 {
//     5
// }

//no semicolon
fn plus_one(x: i32) -> i32 {
    x + 1
}
