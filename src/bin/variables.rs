fn main() {
    //  mutabulity
    let mut x = 5;
    println!("This is x's value {x}");
    x = 6;
    println!("This is x's value {x}");

    // understanding shadowing

    let y = 5;
    println!("This is y's value {y}");
    let y = y + 1;
    {
        let y = y * 2;
        println!("This is y's value {y}");
    }
    println!("This is y's value {y}");

    // data types
    let floating_variable: f32 = -5.0 / 3.0;
    println!("The value of the division is {floating_variable}");

    let remainder = 43.0 % 5.5;
    println!("The value of the division is {remainder}");

    // tuples
    let tup: (u8, f32, char) = (3, 23.4, 'a');
    let (_, a, _) = tup;
    println!("The value pf 'a' from the tuple is {a}");

    // Arrays are immutable in termsof  length as well
    // Use these when you don't want to update the values in it
    let weekday: [&str; 5] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    println!("The weekdays are {:?}", weekday);

    const ARR_LENGTH: usize = 5;

    let arr = [3; ARR_LENGTH];
    println!("The arr values are {:?}", arr);

    println!("Value of funtion five() {}", five());

    let conditional_value = if weekday.len() == 5 {
        "Five days in weekdays"
    } else {
        "Wrong number"
    };
    println!("The value of conditiona;_value is {conditional_value}");

    let mut counter = 0;
    let counting_up = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The counting_up value is {counting_up}");

    // Looping through arrays
    for element in weekday {
        println!("The day is {element}");
    }
}

fn five() -> i8 {
    let d = 5;
    return d;
}
/*
 * Checking multiline comments
 * here;
 * We have reached the end of functions
 */
