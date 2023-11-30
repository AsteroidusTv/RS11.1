use std::io;

fn main() {
    let iterations = input_number();
    let mut a = 7.3;
    println!("Number {} is equal to 1", a);
    for mut i in 0..iterations - 1 {
        i += 1;
        let result = (a - a % 1.0) / 10.0 + (a % 1.0) * 10.0;

        let number: f64;
        if result > a {
            number = result - a;
        } else {
            number = a - result;
        }

        let number = (number * 10.0).round() / 10.0;
        a = number;
        println!("Number {} is equal to {}", i + 1, number);
    }

}

fn input_number() -> i32 {
    loop {
        println!("Please enter a number:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Error, try again !");

        match input.trim().parse() {
            Ok(nombre) => return nombre,
            Err(_) => {
                println!("Error, please enter a valid number !");
                continue;
            }
        }
    }
}