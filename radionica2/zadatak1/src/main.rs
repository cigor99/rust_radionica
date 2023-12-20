// Napisite rust program koji definise funkciju za swapovanje 2 integera
// Treba da se ispostuje da int koji ulaze u funkciju, da se funkciji daje ownership tog inta, a kad se zavrsi, da se vrati ownership

fn main() {
    let mut prvi: i32 = 1;
    let mut drugi: i32 = 2;

    println!("Old value is {} and {}", prvi, drugi);
    
    (prvi, drugi) = swap(prvi, drugi);

    println!("Old value is {} and {}", prvi, drugi);
}

fn swap(mut param1: i32, mut param2: i32) -> (i32, i32) {
    let temporary = param1;
    
    param1 = param2;
    param2 = temporary;
    
    (param1, param2)
}
