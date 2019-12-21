fn main() {
    //infinite loop
    // loop {
    //     println!("again!");
    // }

    //returning a value from a loop
    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("The result is {}", result);

    // looping with while
    // let mut number = 3;
    // while number != 0 {
    //     println!("{}!", number);

    //     number -= 1;
    // }
    // println!("LIFTOFF!!!");

    //looping over an array
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }

    //using a for loop is faster and less error prone
    // let a = [10, 20, 30, 40, 50];
    // for element in a.iter() {
    //     println!("the value is: {}", element);
    // }

    //using for loop with range in reverse
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
