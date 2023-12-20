use std::io;

// 1. Suma parnih brojeva od 1 do n, n je user input, sa loop, for, ili while petljom (mora 2, prva loop, druga koja god)

fn main() {
    let mut input_string = String::new();

    println!("Enter n >>");
    
    io::stdin().read_line(&mut input_string).unwrap();
    
    let n: i32 = input_string.trim().parse().unwrap();

    let mut sum_loop: i32 = 0;

    let mut number: i32 = 0;
    
    loop {
        number = number + 1;

        if number % 2 == 0 {
            sum_loop = sum_loop + number;
        }

        if number == n {
            break;
        }
    }

    println!("Sum is {} with loop", sum_loop);

    let mut sum_for: i32 = 0;

    for number in 0..n+1 {
        if number % 2 == 0 {
            sum_for = sum_for + number;
        }
    }

    println!("Sum is {} with for", sum_for)
}
