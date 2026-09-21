fn main() {
    // The s1 variable's refernce is removed from the stack
    // Using rust's drop() function to free up memory that is not being used
    let mut s1: String = String::from("hello");
    s1.push_str(", world!");

    let s2 = s1;
    println!("{}", s2);

    // Deep cloning does a deep copy of the value stored on the heap
    let str: String = String::from("NewString");
    let str2 = str.clone();
    // println!("{}", str2); will not work because the scope of the value str2 changes
    // which causes the scope to lose ownership of the data

    print_string(str2);

    println!("{}", str);

    // but integers can be moved because it's "Copied" due to being stored on the Stack
    let x: u8 = 5;
    print_integer(x);
}

fn print_string(str: String) {
    println!("{}", str);
}

fn print_integer(integer: u8) {
    println!("{}", integer);
}
