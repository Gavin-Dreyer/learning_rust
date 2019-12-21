fn main() {
    // let number = 7;

    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }

    // causes an error because if requires a boolean
    // let number = 3;
    // if number {
    //     println!("number was three");
    // }

    //runs because it is receiving a bool
    // let number = 3;
    // if number != 0 {
    //     println!("number was something other than zero");
    // }

    //checks until it finds a match and ignores the rest
    // let number = 6;
    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }

    //if is an expression so you can assign it to a variable
    // let condition = true;
    // let number = if condition { 5 } else { 6 };
    // println!("The value of number is: {}", number);

    //if expressions have to have the same type in all arms
    //this throws an error
    // let condition = true;
    // let number = if condition {
    //     5
    // } else {
    //     "six"
    // };
    // println!("The value of number is: {}", number);
}
