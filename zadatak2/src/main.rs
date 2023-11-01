// Faktorijel sa rekurzijom

fn factor(number: i32) -> i32 {
    if number == 0 {
        return 1;
    }

    number * factor(number - 1)
}

fn main() {
    let mut input_string = String::new();

    println!("Enter number >>");

    std::io::stdin().read_line(&mut input_string).unwrap();

    let number: i32 = input_string.trim().parse().unwrap();

    let result = factor(number);
    
    println!("Result is {}", result);
}
