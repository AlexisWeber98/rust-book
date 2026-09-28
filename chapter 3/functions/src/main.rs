fn main() {
    println!("Hello World!");
    another_function(5);
    print_labeled_masurement(5, 'h');
}

fn another_function(number: i32) {
    println!("The value of number is: {number}");
}

fn print_labeled_masurement(value: i32, unit_label: char) {
    println!("The masurement is: {value} {unit_label}");
}

// --------------------------------- Sentencias y expresiones ----------------------------------- //
