// Create a program that reads an array of integers from the user, calculates the sum of
// the integers using a separate function that borrows the array using references.
// Ensure the main function retains ownership of the array.

fn main() {
    let mut vec = Vec::<i32>::new();

    println!("Enter number or x to stop >>");

    loop {
        let mut input_string = String::new();

        std::io::stdin().read_line(&mut input_string).unwrap();

        if input_string.trim().eq("x") {
            break;
        }

        vec.push(input_string.trim().parse().unwrap());
    }

    let sum = sum(&vec);

    println!("Sum is {}", sum);
    
    println!("Elements are");

    for item in vec {
        println!("{}", item);
    }
}

fn sum(vec: &Vec<i32>) -> i32 {
    let mut sum = 0;

    for item in vec {
        sum = sum + item;
    }

    sum
}
